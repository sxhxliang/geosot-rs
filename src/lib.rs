use std::f64;

/// 将十进制经纬度获取 geomgrid 值
/// # 参数
/// - `x`: 经度
/// - `y`: 纬度
/// - `precision`: 精度,取值范围 [1~32]
pub fn get_code(lng: f64, lat: f64, precision: usize) -> u64 {
    let lng = dec2code(lng, precision);
    let lat = dec2code(lat, precision);

    magic_bits(lng, lat)
}

/// 将十进制经纬度值转为经纬度编码
/// # 参数
/// - `dec`: 经度或纬度编码
/// - `precision`: 精度,取值范围 [1~32]
fn dec2code(dec: f64, precision: usize) -> u32 {
    let mut code = 0;
    let val = if dec < 0.0 { -dec } else { dec };
    let g = if dec < 0.0 { 1 } else { 0 };
    let d = val.trunc() as u32;
    let dm = round((val - d as f64) * 60.0, 6);
    let m = dm.trunc() as u32;
    let seconds = round((dm - m as f64) * 60.0, 4);
    let s = seconds.trunc() as u32;
    let dot_seconds = (seconds - s as f64) * 2048.0;
    let s11 = dot_seconds.round() as u32;

    code = (g << 31) | (d << 23) | (m << 17) | (s << 11) | s11;
    code >>= 32 - precision;
    code <<= 32 - precision;

    code
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
fn split_by_bits(a: u32) -> u64 {
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
