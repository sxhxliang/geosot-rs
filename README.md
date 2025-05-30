# Implementing the GeoSot in Rust

#  编码使用示例
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
# GeoSOT 空间关系计算库

基于 GeoSOT 编码的空间区域交、并、差、补关系计算库。

## 功能特性

- ✅ **基础编码功能**：经纬度与 GeoSOT 编码的相互转换
- ✅ **空间区域表示**：支持点、矩形、多边形区域的 GeoSOT 编码表示
- ✅ **空间集合运算**：交集、并集、差集、对称差集、补集
- ✅ **空间关系判断**：相交、包含、子集、超集、不相交判断
- ✅ **空间分析指标**：Jaccard相似度、重叠率、紧密度计算
- ✅ **多种创建方式**：支持点集、矩形、多边形等多种区域创建方式

## 快速开始

### 基本使用

```rust
use geosot::spatial::{GeoSotRegion, spatial_analysis};

fn main() {
    // 创建区域
    let level = 20;  // 精度等级
    let mut region = GeoSotRegion::new(level);
    
    // 添加点
    region.add_point(116.0, 39.0);  // 北京
    region.add_point(121.0, 31.0);  // 上海
    
    println!("区域包含 {} 个网格", region.size());
}
```

### 空间集合运算

```rust
use geosot::spatial::GeoSotRegion;

// 创建两个区域
let region_a = GeoSotRegion::from_codes(vec![1, 2, 3, 4], 20);
let region_b = GeoSotRegion::from_codes(vec![3, 4, 5, 6], 20);

// 交集运算
let intersection = region_a.intersection(&region_b);
println!("交集: {} 个网格", intersection.size());

// 并集运算
let union = region_a.union(&region_b);
println!("并集: {} 个网格", union.size());

// 差集运算
let difference = region_a.difference(&region_b);
println!("差集: {} 个网格", difference.size());

// 对称差集
let sym_diff = region_a.symmetric_difference(&region_b);
println!("对称差集: {} 个网格", sym_diff.size());
```

### 矩形区域创建

```rust
use geosot::spatial::GeoSotRegion;

// 创建矩形区域
let region = GeoSotRegion::from_rectangle(
    115.0, 38.0,  // 左下角经纬度
    117.0, 40.0,  // 右上角经纬度
    18            // 精度等级
);

println!("矩形区域包含 {} 个网格", region.size());
```

### 空间关系判断

```rust
let region1 = GeoSotRegion::from_codes(vec![1, 2, 3], 20);
let region2 = GeoSotRegion::from_codes(vec![2, 3, 4], 20);
let region3 = GeoSotRegion::from_codes(vec![1, 2], 20);

// 判断相交
if region1.intersects(&region2) {
    println!("区域1和区域2相交");
}

// 判断包含关系
if region3.is_subset(&region1) {
    println!("区域3是区域1的子集");
}

// 判断不相交
if region1.is_disjoint(&region2) {
    println!("区域1和区域2不相交");
}
```

### 空间分析指标

```rust
use geosot::spatial::spatial_analysis;

let region1 = GeoSotRegion::from_codes(vec![1, 2, 3, 4], 20);
let region2 = GeoSotRegion::from_codes(vec![3, 4, 5, 6], 20);

// Jaccard 相似度
let similarity = spatial_analysis::jaccard_similarity(&region1, &region2);
println!("Jaccard相似度: {:.4}", similarity);

// 重叠率
let overlap = spatial_analysis::overlap_ratio(&region1, &region2);
println!("重叠率: {:.4}", overlap);

// 紧密度
let compactness = spatial_analysis::compactness(&region1);
println!("紧密度: {:.4}", compactness);
```

### 补集运算

```rust
// 定义全集
let universe = GeoSotRegion::from_codes(vec![1, 2, 3, 4, 5, 6, 7, 8], 20);
let region = GeoSotRegion::from_codes(vec![2, 4, 6], 20);

// 计算补集
let complement = region.complement(&universe);
println!("补集包含 {} 个网格", complement.size());
// 结果：{1, 3, 5, 7, 8}
```

## API 参考

### GeoSotRegion 结构体

#### 创建方法

- `new(level: usize)` - 创建空区域
- `from_codes(codes: Vec<u64>, level: usize)` - 从编码向量创建
- `from_rectangle(min_lng, min_lat, max_lng, max_lat, level)` - 从矩形创建
- `from_polygon(points: &[(f64, f64)], level)` - 从多边形创建

#### 添加操作

- `add_code(&mut self, code: u64)` - 添加编码
- `add_point(&mut self, lng: f64, lat: f64)` - 添加经纬度点

#### 查询操作

- `contains(&self, code: u64) -> bool` - 检查是否包含编码
- `contains_point(&self, lng: f64, lat: f64) -> bool` - 检查是否包含点
- `size(&self) -> usize` - 获取网格数量
- `is_empty(&self) -> bool` - 检查是否为空

#### 集合运算

- `intersection(&self, other: &GeoSotRegion) -> GeoSotRegion` - 交集
- `union(&self, other: &GeoSotRegion) -> GeoSotRegion` - 并集  
- `difference(&self, other: &GeoSotRegion) -> GeoSotRegion` - 差集
- `symmetric_difference(&self, other: &GeoSotRegion) -> GeoSotRegion` - 对称差集
- `complement(&self, universe: &GeoSotRegion) -> GeoSotRegion` - 补集

#### 关系判断

- `is_subset(&self, other: &GeoSotRegion) -> bool` - 子集判断
- `is_superset(&self, other: &GeoSotRegion) -> bool` - 超集判断
- `intersects(&self, other: &GeoSotRegion) -> bool` - 相交判断
- `is_disjoint(&self, other: &GeoSotRegion) -> bool` - 不相交判断

#### 转换方法

- `to_strings(&self) -> Vec<String>` - 转为网格字符串表示
- `to_coordinates(&self) -> Vec<(f64, f64)>` - 转为经纬度坐标

### 空间分析模块 (spatial_analysis)

- `jaccard_similarity(region1, region2) -> f64` - Jaccard相似度计算
- `overlap_ratio(region1, region2) -> f64` - 重叠率计算  
- `compactness(region) -> f64` - 紧密度计算

## 使用示例

### 复合操作

```rust
let beijing = GeoSotRegion::from_rectangle(115.5, 39.0, 116.5, 40.0, 18);
let shanghai = GeoSotRegion::from_rectangle(120.5, 30.5, 121.5, 31.5, 18);
let hangzhou = GeoSotRegion::from_rectangle(119.5, 29.5, 120.5, 30.5, 18);

// 复合操作：(北京 ∪ 上海) - 杭州
let result = beijing.union(&shanghai).difference(&hangzhou);
println!("复合操作结果: {} 个网格", result.size());
```

### 批量区域分析

```rust
let regions = vec![
    GeoSotRegion::from_rectangle(115.0, 39.0, 117.0, 40.0, 18),
    GeoSotRegion::from_rectangle(120.0, 30.0, 122.0, 32.0, 18),
    GeoSotRegion::from_rectangle(113.0, 22.0, 115.0, 24.0, 18),
];

// 计算所有区域的并集
let mut total_union = GeoSotRegion::new(18);
for region in &regions {
    total_union = total_union.union(region);
}

println!("所有区域的并集: {} 个网格", total_union.size());

// 分析区域间的相似度
for i in 0..regions.len() {
    for j in (i+1)..regions.len() {
        let similarity = spatial_analysis::jaccard_similarity(&regions[i], &regions[j]);
        println!("区域{}与区域{}的相似度: {:.4}", i+1, j+1, similarity);
    }
}
```

## 运行示例

```bash
# 运行基本示例
cargo run --example demo

# 运行空间关系示例  
cargo run --example spatial_demo

# 运行测试
cargo test

# 运行特定测试
cargo test test_spatial_operations
```

## 注意事项

1. **精度等级**：精度等级范围为 1-32，数值越大精度越高，但计算量也越大
2. **性能考虑**：大区域（高精度）的空间运算可能较慢，建议根据实际需求选择合适的精度
3. **内存使用**：区域使用 `BTreeSet<u64>` 存储编码，大区域会占用较多内存
4. **坐标系统**：使用 WGS84 坐标系统，经度范围 [-180, 180]，纬度范围 [-90, 90]

## 扩展功能

可以基于此库进一步开发：

- 多层级区域分析
- 时空数据处理
- 空间索引构建
- 地理信息可视化
- 空间聚类分析

## 许可证

MIT License