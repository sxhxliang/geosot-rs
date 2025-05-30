// examples/spatial_demo.rs
use geosot::spatial::{GeoSotRegion, spatial_analysis};

fn main() {
    println!("=== GeoSOT 空间关系计算演示 ===\n");

    // 创建测试区域
    let level = 20;
    use geosot::spatial::GeoSotRegion;

    // 创建矩形区域
    let region = GeoSotRegion::from_rectangle(
        115.0, 38.0,  // 左下角经纬度
        117.0, 40.0,  // 右上角经纬度
        18            // 精度等级
    );

    println!("矩形区域包含 {} 个网格", region.size());
    
    // 区域1：北京周边
    let mut beijing_region = GeoSotRegion::new(level);
    beijing_region.add_point(116.0, 39.0);   // 北京中心
    beijing_region.add_point(116.1, 39.0);   // 北京东
    beijing_region.add_point(116.0, 39.1);   // 北京北
    beijing_region.add_point(116.1, 39.1);   // 北京东北
    
    // 区域2：重叠区域
    let mut overlap_region = GeoSotRegion::new(level);
    overlap_region.add_point(116.0, 39.0);   // 与北京重叠
    overlap_region.add_point(116.2, 39.0);   // 北京东边
    overlap_region.add_point(116.0, 38.9);   // 北京南边
    
    // 区域3：上海区域（不重叠）
    let mut shanghai_region = GeoSotRegion::new(level);
    shanghai_region.add_point(121.0, 31.0);  // 上海中心
    shanghai_region.add_point(121.1, 31.0);  // 上海东
    shanghai_region.add_point(121.0, 31.1);  // 上海北
    
    println!("创建的区域信息:");
    println!("北京区域大小: {}", beijing_region.size());
    println!("重叠区域大小: {}", overlap_region.size());
    println!("上海区域大小: {}", shanghai_region.size());
    println!();

    // 演示交集操作
    println!("=== 交集运算 ===");
    let intersection = beijing_region.intersection(&overlap_region);
    println!("北京区域 ∩ 重叠区域 = {} 个网格", intersection.size());
    println!("交集编码: {:?}", intersection.codes);
    println!();

    // 演示并集操作
    println!("=== 并集运算 ===");
    let union = beijing_region.union(&overlap_region);
    println!("北京区域 ∪ 重叠区域 = {} 个网格", union.size());
    println!();

    // 演示差集操作
    println!("=== 差集运算 ===");
    let difference = beijing_region.difference(&overlap_region);
    println!("北京区域 - 重叠区域 = {} 个网格", difference.size());
    
    let reverse_diff = overlap_region.difference(&beijing_region);
    println!("重叠区域 - 北京区域 = {} 个网格", reverse_diff.size());
    println!();

    // 演示对称差集
    println!("=== 对称差集运算 ===");
    let sym_diff = beijing_region.symmetric_difference(&overlap_region);
    println!("北京区域 ⊕ 重叠区域 = {} 个网格", sym_diff.size());
    println!();

    // 演示补集操作
    println!("=== 补集运算 ===");
    let universe = beijing_region.union(&overlap_region).union(&shanghai_region);
    let complement = beijing_region.complement(&universe);
    println!("北京区域在全集中的补集 = {} 个网格", complement.size());
    println!();

    // 演示空间关系判断
    println!("=== 空间关系判断 ===");
    println!("北京区域是否与重叠区域相交: {}", beijing_region.intersects(&overlap_region));
    println!("北京区域是否与上海区域相交: {}", beijing_region.intersects(&shanghai_region));
    println!("北京区域是否与上海区域不相交: {}", beijing_region.is_disjoint(&shanghai_region));
    println!();

    // 子集和超集关系
    let small_beijing = GeoSotRegion::from_codes(
        beijing_region.codes.iter().take(2).cloned().collect(), 
        level
    );
    println!("小北京区域是否是北京区域的子集: {}", small_beijing.is_subset(&beijing_region));
    println!("北京区域是否是小北京区域的超集: {}", beijing_region.is_superset(&small_beijing));
    println!();

    // 演示空间分析指标
    println!("=== 空间分析指标 ===");
    let jaccard = spatial_analysis::jaccard_similarity(&beijing_region, &overlap_region);
    println!("北京区域与重叠区域的Jaccard相似度: {:.4}", jaccard);
    
    let overlap_ratio = spatial_analysis::overlap_ratio(&beijing_region, &overlap_region);
    println!("重叠区域在北京区域中的覆盖率: {:.4}", overlap_ratio);
    
    let compactness = spatial_analysis::compactness(&beijing_region);
    println!("北京区域的紧密度: {:.4}", compactness);
    println!();

    // 演示矩形区域创建
    println!("=== 矩形区域创建 ===");
    let rect_region = GeoSotRegion::from_rectangle(
        115.9, 38.9,  // 左下角
        116.2, 39.2,  // 右上角
        18  // 使用较低精度以减少网格数量
    );
    println!("矩形区域(115.9,38.9)到(116.2,39.2)包含 {} 个网格", rect_region.size());
    
    // 显示前几个网格的字符串表示
    let grid_strings = rect_region.to_strings();
    println!("前5个网格的字符串表示:");
    for (i, grid_str) in grid_strings.iter().take(5).enumerate() {
        println!("  {}: {}", i + 1, grid_str);
    }
    println!();

    // 演示多边形区域创建（使用包围盒）
    println!("=== 多边形区域创建 ===");
    let polygon_points = vec![
        (116.0, 39.0),
        (116.2, 39.0),
        (116.2, 39.2),
        (116.0, 39.2),
    ];
    let polygon_region = GeoSotRegion::from_polygon(&polygon_points, 18);
    println!("多边形区域包含 {} 个网格", polygon_region.size());
    println!();

    // 复合操作示例
    println!("=== 复合操作示例 ===");
    let complex_result = beijing_region
        .union(&overlap_region)
        .difference(&shanghai_region)
        .intersection(&universe);
    println!("复合操作结果包含 {} 个网格", complex_result.size());
    
    // 坐标转换示例
    println!("=== 坐标转换示例 ===");
    let coordinates = small_beijing.to_coordinates();
    println!("小北京区域的经纬度坐标:");
    for (i, (lng, lat)) in coordinates.iter().enumerate() {
        println!("  网格{}: ({:.6}, {:.6})", i + 1, lng, lat);
    }
}