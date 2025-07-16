use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use std::f64;

pub mod utils;
pub mod spatial;

/// 将十进制经纬度获取 geomgrid 值
/// 二维莫顿码
/// Magicbits masks (2D encode)
/// # 参数
/// - `x`: 经度
/// - `y`: 纬度
/// - `precision`: 精度,取值范围 [1~32]
#[pyfunction]
pub fn get_code(lng: f64, lat: f64, precision: usize) -> u64 {
    let lng = dec2code(lng, precision);
    let lat = dec2code(lat, precision);

    magic_bits(lng, lat)
}

/// 将 geomgrid 编码转为经纬度
/// # 参数
/// - `code`: geomgrid 编码
/// # 返回
/// - `(f64, f64)`: 返回经度和纬度的元组
#[pyfunction]
pub fn decode_by_geomgrid(code: u64) -> (f64, f64) {
    let (lng, lat) = un_magic_bits(code);
    (code2dec(lng), code2dec(lat))
}

/// 将十进制经纬度值转为经纬度编码
/// # 参数
/// - `dec`: 经度或纬度编码
/// - `precision`: 精度,取值范围 [1~32]
#[pyfunction]
pub fn dec2code(dec: f64, precision: usize) -> u32 {
    let mut code: u32;
    let val = dec.abs();
    let g = if dec < 0.0 { 1 } else { 0 };
    let d = val.trunc() as u32;
    let dm = round((val - d as f64) * 60.0, 6);
    let m = dm.trunc() as u32;
    let seconds = round((dm - m as f64) * 60.0, 4);
    let s = seconds.trunc() as u32;
    let dot_seconds = (seconds - s as f64) * 2048.0;
    let s11 = dot_seconds.round() as u32;

    code = (g << 31) | (d << 23) | (m << 17) | (s << 11) | s11;
    if precision < 32 {
        code >>= 32 - precision;
        code <<= 32 - precision;
    }

    code
}


///
/// 将经纬度编码转为十进制经纬度值
/// # 参数
/// - `x`: 经度或纬度编码
///# Example
/// ```
/// use geosot::{dec2code,code2dec};
/// let lng = 76.233;
/// println!("{:?}", lng);
/// let precision = 32;
/// let lng = dec2code(lng, precision);
/// println!("{:?}", lng);
/// println!("{:?}", code2dec(lng));
/// ```
///  --------------------------------
/// - 76.233
/// - 639358566
/// - 76.23299994574653
#[pyfunction]
pub fn code2dec(x: u32) -> f64 {
    let g = x >> 31;          // 1b
    let d = (x >> 23) & 0xFF; // 8b
    let m = (x >> 17) & 0x3F; // 6b
    let s = (x >> 11) & 0x3F; // 6b
    let s11 = x & 0x7FF;      // 11b

    let dec = d as f64 + m as f64 / 60.0 + (s as f64 + s11 as f64 / 2048.0) / 3600.0;
    if g == 1 {
        -dec
    } else {
        dec
    }
}

/// 将经度编码和纬度编码转为 geomgrid 值
/// # 参数
/// - `lng`: 经度
/// - `lat`: 纬度
fn magic_bits(lng: u32, lat: u32) -> u64 {
    split_by_bits(lng) | (split_by_bits(lat) << 1)
}

/// 将 32 位经纬度编码转为 64 位形式
/// # 参数
/// - `a`: 度分秒按位转换后的值
/// # Examples
/// ```
/// use geosot::{dec2code, split_by_bits};
/// let lng = 76.233;
/// let precision = 32;
/// let lng = dec2code(lng, precision);
/// println!("split_by_bits {}", split_by_bits(lng));
/// ```
/// split_by_bits 639358566
#[pyfunction]
pub fn split_by_bits(a: u32) -> u64 {
    let mut x = a as u64;
    x = (x | x << 32) & 0x00000000FFFFFFFF;
    x = (x | x << 16) & 0x0000FFFF0000FFFF;
    x = (x | x << 8) & 0x00FF00FF00FF00FF;
    x = (x | x << 4) & 0x0F0F0F0F0F0F0F0F;
    x = (x | x << 2) & 0x3333333333333333;
    x = (x | x << 1) & 0x5555555555555555;
    x
}

/// 四舍五入
/// # 参数
/// - `x`: 要操作的值
/// - `y`: 保留的位数
fn round(x: f64, y: i32) -> f64 {
    let mul = 10f64.powi(y);
    if x >= 0.0 {
        (x * mul + 0.5).trunc() / mul
    } else {
        (x * mul - 0.5).trunc() / mul
    }
}

/// 将 geomgrid 转为文本形式
/// # 参数
/// - `code`: geomgrid 编码
/// - `level`: 精度等级, 取值范围 [1~32]
#[pyfunction]
pub fn to_string(code: u64, level: usize) -> String {
    let mut str_out = String::from("G");
    let level = level - 1;
    for i in (31 - level..=31).rev() {
        let v = (code >> (i * 2)) & 0x3;
        // 将整数转换为字符串，并追加到 str_out 中
        str_out.push_str(&v.to_string());
        // 根据条件添加连接符号
        if i > 32 - level {
            match i {
                23 | 17 => str_out.push('-'),
                11 => str_out.push('.'),
                _ => (),
            }
        }
    }
    str_out
}


///  根据 geomgrid 分离出经度和纬度的编码
/// # 参数
/// - `m`: geomgrid 值
/// # Examples
/// ```
/// use geosot::{un_magic_bits, code2dec};
/// let m: u64 = 339638376531246140;
/// let (lng, lat) = un_magic_bits(m);
/// println!("Longitude: {}, Latitude: {}", lng, lat);
/// println!("Longitude: {}, Latitude: {}", code2dec(lng), code2dec(lat));
/// ```
/// --------------------------------------------------
/// - Longitude: 639358566, Latitude: 231900774
/// - Longitude: 76.23299994574653, Latitude: 27.68799994574653
///
#[pyfunction]
pub fn un_magic_bits(m: u64) -> (u32, u32) {
    let lng = merge_by_bits(m);
    let lat = merge_by_bits(m >> 1);
    (lng, lat)
}

///
///  从 geomgrid 值分离其中的单个 32 位编码
/// # 参数
/// - `m`: geomgrid 值
///
#[pyfunction]
pub fn merge_by_bits(m: u64) -> u32 {
    let mut x = m & 0x5555555555555555;
    x = (x ^ (x >> 1)) & 0x3333333333333333;
    x = (x ^ (x >> 2)) & 0x0F0F0F0F0F0F0F0F;
    x = (x ^ (x >> 4)) & 0x00FF00FF00FF00FF;
    x = (x ^ (x >> 8)) & 0x0000FFFF0000FFFF;
    x = (x ^ (x >> 16)) & 0x00000000FFFFFFFF;
    x as u32
}

#[pymodule]
fn geosot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_code, m)?)?;
    m.add_function(wrap_pyfunction!(decode_by_geomgrid, m)?)?;
    m.add_function(wrap_pyfunction!(dec2code, m)?)?;
    m.add_function(wrap_pyfunction!(code2dec, m)?)?;
    m.add_function(wrap_pyfunction!(split_by_bits, m)?)?;
    m.add_function(wrap_pyfunction!(to_string, m)?)?;
    m.add_function(wrap_pyfunction!(un_magic_bits, m)?)?;
    m.add_function(wrap_pyfunction!(merge_by_bits, m)?)?;
    m.add_class::<spatial::GeoSotCell>()?;
    m.add_class::<spatial::GeoSotRegion>()?;
    m.add_wrapped(wrap_pymodule!(spatial::spatial_analysis))?;
    Ok(())
}