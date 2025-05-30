
/// 获取第i级[0,32]的分块大小, 单位为度
/// 
/// # 参数
/// * `i` - 层级
/// 
/// # 返回
/// 返回指定层级的分块大小，单位为度
pub fn get_cell_size_in_degree(i: i32) -> Result<f64, &'static str> {  
    match i {  
        i if i >= 0 && i <= 9 => Ok(2.0f64.powf(9.0 - i as f64)),  
        i if i >= 10 && i <= 15 => Ok(2.0f64.powf(15.0 - i as f64) / 60.0),  
        i if i >= 16 && i <= 32 => Ok(2.0f64.powf(21.0 - i as f64) / 3600.0),  
        _ => Err("i must be between 0 and 32 inclusive"),  
    }  
} 