// tests/spatial_test.rs
#[cfg(test)]
mod tests {
    use geosot::spatial::{GeoSotRegion, spatial_analysis};
    use geosot::{get_code, to_string};

    #[test]
    fn test_basic_spatial_operations() {
        println!("=== 基础空间操作测试 ===");
        
        // 创建两个测试区域
        let level = 20;
        let region_a = GeoSotRegion::from_codes(vec![
            get_code(116.0, 39.0, level),  // 北京
            get_code(116.1, 39.0, level),  // 北京东
            get_code(116.0, 39.1, level),  // 北京北
        ], level);
        
        let region_b = GeoSotRegion::from_codes(vec![
            get_code(116.0, 39.0, level),  // 重叠点
            get_code(116.2, 39.0, level),  // 更远的东边
            get_code(121.0, 31.0, level),  // 上海
        ], level);

        println!("区域A大小: {}", region_a.size());
        println!("区域B大小: {}", region_b.size());

        // 测试交集
        let intersection = region_a.intersection(&region_b);
        println!("交集大小: {}", intersection.size());
        assert_eq!(intersection.size(), 1); // 只有一个重叠点

        // 测试并集
        let union = region_a.union(&region_b);
        println!("并集大小: {}", union.size());
        assert_eq!(union.size(), 5); // 总共5个不同的点

        // 测试差集
        let diff_a_b = region_a.difference(&region_b);
        let diff_b_a = region_b.difference(&region_a);
        println!("A-B差集大小: {}", diff_a_b.size());
        println!("B-A差集大小: {}", diff_b_a.size());
        assert_eq!(diff_a_b.size(), 2); // A中有2个点不在B中
        assert_eq!(diff_b_a.size(), 2); // B中有2个点不在A中

        // 测试对称差集
        let sym_diff = region_a.symmetric_difference(&region_b);
        println!("对称差集大小: {}", sym_diff.size());
        assert_eq!(sym_diff.size(), 4); // 除了重叠的1个点，其他4个点
    }

    #[test]
    fn test_spatial_relationships() {
        println!("=== 空间关系测试 ===");
        
        let level = 18; // 使用较低精度以便测试
        
        // 创建包含关系的区域
        let large_region = GeoSotRegion::from_rectangle(
            115.5, 38.5,  // 左下角
            116.5, 39.5,  // 右上角
            level
        );
        
        let small_region = GeoSotRegion::from_rectangle(
            115.8, 38.8,  // 左下角（在大区域内）
            116.2, 39.2,  // 右上角（在大区域内）
            level
        );
        
        // 创建不相交的区域
        let distant_region = GeoSotRegion::from_rectangle(
            120.0, 30.0,  // 上海附近
            121.0, 31.0,
            level
        );

        println!("大区域大小: {}", large_region.size());
        println!("小区域大小: {}", small_region.size());
        println!("远距离区域大小: {}", distant_region.size());

        // 测试包含关系
        assert!(small_region.is_subset(&large_region), "小区域应该是大区域的子集");
        assert!(large_region.is_superset(&small_region), "大区域应该是小区域的超集");

        // 测试相交关系
        assert!(large_region.intersects(&small_region), "大区域和小区域应该相交");
        assert!(!large_region.intersects(&distant_region), "大区域和远距离区域不应该相交");
        assert!(large_region.is_disjoint(&distant_region), "大区域和远距离区域应该不相交");
    }

    #[test]
    fn test_spatial_analysis_metrics() {
        println!("=== 空间分析指标测试 ===");
        
        let level = 20;
        
        // 创建两个有部分重叠的区域
        let codes_a = vec![1u64, 2, 3, 4, 5];
        let codes_b = vec![3u64, 4, 5, 6, 7];
        
        let region_a = GeoSotRegion::from_codes(codes_a, level);
        let region_b = GeoSotRegion::from_codes(codes_b, level);

        // 测试Jaccard相似度
        let jaccard = spatial_analysis::jaccard_similarity(&region_a, &region_b);
        println!("Jaccard相似度: {:.4}", jaccard);
        
        // 交集: {3, 4, 5} = 3个元素
        // 并集: {1, 2, 3, 4, 5, 6, 7} = 7个元素
        // Jaccard = 3/7 ≈ 0.4286
        assert!((jaccard - 3.0/7.0).abs() < 1e-10, "Jaccard相似度计算错误");

        // 测试重叠率
        let overlap_ratio = spatial_analysis::overlap_ratio(&region_a, &region_b);
        println!("重叠率: {:.4}", overlap_ratio);
        
        // 重叠率 = 交集大小 / region_a大小 = 3/5 = 0.6
        assert!((overlap_ratio - 0.6).abs() < 1e-10, "重叠率计算错误");

        // 测试紧密度
        let compactness = spatial_analysis::compactness(&region_a);
        println!("紧密度: {:.4}", compactness);
        assert!(compactness >= 0.0 && compactness <= 1.0, "紧密度应该在0-1之间");
    }

    #[test]
    fn test_complement_operation() {
        println!("=== 补集操作测试 ===");
        
        let level = 20;
        let universe_codes = vec![1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let region_codes = vec![2u64, 4, 6, 8];
        
        let universe = GeoSotRegion::from_codes(universe_codes, level);
        let region = GeoSotRegion::from_codes(region_codes, level);
        
        let complement = region.complement(&universe);
        
        println!("全集大小: {}", universe.size());
        println!("区域大小: {}", region.size());
        println!("补集大小: {}", complement.size());
        
        assert_eq!(complement.size(), 6); // 10 - 4 = 6
        
        // 验证补集包含正确的元素
        let expected_complement_codes = vec![1u64, 3, 5, 7, 9, 10];
        for code in expected_complement_codes {
            assert!(complement.contains(code), "补集应该包含编码 {}", code);
        }
    }

    #[test]
    fn test_region_creation_methods() {
        println!("=== 区域创建方法测试 ===");
        
        let level = 18; // 使用较低精度以减少计算量
        
        // 测试点创建
        let mut point_region = GeoSotRegion::new(level);
        point_region.add_point(116.0, 39.0);
        point_region.add_point(121.0, 31.0);
        assert_eq!(point_region.size(), 2);
        
        // 测试矩形创建
        let rect_region = GeoSotRegion::from_rectangle(
            115.9, 38.9,
            116.1, 39.1,
            level
        );
        println!("矩形区域大小: {}", rect_region.size());
        assert!(rect_region.size() > 0);
        
        // 测试多边形创建（使用包围盒）
        let polygon_points = vec![
            (115.9, 38.9),
            (116.1, 38.9),
            (116.1, 39.1),
            (115.9, 39.1),
        ];
        let polygon_region = GeoSotRegion::from_polygon(&polygon_points, level);
        println!("多边形区域大小: {}", polygon_region.size());
        assert!(polygon_region.size() > 0);
        
        // 矩形和多边形应该产生相似的结果（因为多边形是矩形）
        assert_eq!(rect_region.size(), polygon_region.size());
    }

    #[test]
    fn test_coordinate_conversion() {
        println!("=== 坐标转换测试 ===");
        
        let level = 20;
        let mut region = GeoSotRegion::new(level);
        let mut original_points = vec![
            (116.0, 39.0),
            (121.0, 31.0),
            (113.0, 23.0), // 广州
        ];
        
        // 添加点到区域
        for (lng, lat) in &original_points {
            region.add_point(*lng, *lat);
        }
        
        // 转换回坐标
        let converted_coords = region.to_coordinates();
        println!("原始点数: {}", original_points.len());
        println!("转换后点数: {}", converted_coords.len());
        
        assert_eq!(converted_coords.len(), original_points.len());
        
        // 验证转换精度（应该非常接近原始坐标）
        original_points.reverse();
        for (i, (orig_lng, orig_lat)) in original_points.iter().enumerate() {
            let (conv_lng, conv_lat) = converted_coords[i];
            println!("原始: ({:.6}, {:.6}) -> 转换: ({:.6}, {:.6})", 
                     orig_lng, orig_lat, conv_lng, conv_lat);
            
            assert!((orig_lng - conv_lng).abs() < 0.01, "经度转换误差过大");
            assert!((orig_lat - conv_lat).abs() < 0.01, "纬度转换误差过大");
        }
    }
}

fn main() {
    println!("运行 'cargo test' 来执行所有测试");
}