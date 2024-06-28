# Implementing the GeoSot in Rust
```rust
use geosot::{get_code, to_string};
fn main() {
    let x = 76.233;
    let y = 27.688;
    let level = 32;
    let code = get_code(x, y, level);
    
    // 32级
    // code: 339638376531246140
    // grid: G001023122-203103-131010.33003300330
    println!("经维度: {} {}", x, y);
    println!("code: {}", code);
    println!("grid: {}", to_string(code, level));
}
```