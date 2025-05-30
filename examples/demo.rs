use geosot::{get_code, to_string, decode_by_geomgrid};
use geosot::utils::get_cell_size_in_degree;

fn main() {
    let x = 76.233;
    let y = 27.688;
    let z = 100;
    let level = 32;
    let code = get_code(x, y, level.clone());
    // let code_3d = get_code_3d(x, y, z, level);
    
    // 32级
    // 经维高: 76.233 27.688 100
    // code 2d: 339638376531246140
    // code 3d: 000000001000010011001010010010000011001000011001011001000001000011011000000111111000000111011000
    // grid: G001023122-203103-131010.33003300330

    println!("经维高: {} {} {}", x, y, z);
    println!("code 2d: {}", code);
    // println!("code 3d: {:096b}", code_3d);
    println!("grid: {}", to_string(code, level));
    // 网格编码转经纬度
    println!("get_cell_size_in_degree: {}", get_cell_size_in_degree(32).unwrap());
    let (lng, lat) = decode_by_geomgrid(code);
    println!("Longitude: {}, Latitude: {}", lng, lat);
}