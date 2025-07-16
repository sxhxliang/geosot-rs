use pyo3::prelude::*;
use std::collections::{HashSet, BTreeSet};
use crate::{get_code, decode_by_geomgrid, to_string};

// GeoSot网格单元，包含编码和精度级别
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GeoSotCell {
    #[pyo3(get, set)]
    pub code: u64,
    #[pyo3(get, set)]
    pub level: usize,
}

#[pymethods]
impl GeoSotCell {
    /// 创建新的GeoSot单元
    #[new]
    pub fn new(code: u64, level: usize) -> Self {
        Self { code, level }
    }

    /// 从经纬度创建GeoSot单元
    #[staticmethod]
    pub fn from_coords(lng: f64, lat: f64, level: usize) -> Self {
        let code = get_code(lng, lat, level);
        Self::new(code, level)
    }

    /// 转换为字符串表示
    pub fn to_string_py(&self) -> String {
        to_string(self.code, self.level)
    }

    /// 获取父级单元（降低一级精度）
    pub fn parent(&self) -> Option<Self> {
        if self.level <= 1 {
            return None;
        }
        let parent_level = self.level - 1;
        let _shift = (32 - parent_level) * 2;
        let parent_code = (self.code >> 2) << 2;  // 清除最低2位
        Some(Self::new(parent_code, parent_level))
    }

    /// 获取子级单元（增加一级精度）
    pub fn children(&self) -> Vec<Self> {
        if self.level >= 32 {
            return vec![];
        }
        let child_level = self.level + 1;
        let mut children = Vec::new();

        // 每个单元有4个子单元（00, 01, 10, 11）
        for i in 0..4 {
            let child_code = (self.code << 2) | i;
            children.push(Self::new(child_code, child_level));
        }
        children
    }

    /// 检查是否为另一个单元的祖先
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        if self.level >= other.level {
            return false;
        }
        let level_diff = other.level - self.level;
        let shifted_other = other.code >> (level_diff * 2);
        shifted_other == self.code
    }

    /// 检查是否为另一个单元的后代
    pub fn is_descendant_of(&self, other: &Self) -> bool {
        other.is_ancestor_of(self)
    }

    /// 检查两个单元是否相邻
    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        if self.level != other.level {
            return false;
        }
        // 简化的相邻检查，实际实现需要考虑Morton编码的特性
        let diff = if self.code > other.code {
            self.code - other.code
        } else {
            other.code - self.code
        };
        // 相邻单元的编码差值应该是特定的值
        diff == 1 || diff == 2 || diff == 4 || diff == 8
    }
}

/// GeoSOT 编码的空间区域表示
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeoSotRegion {
    /// 编码集合，使用 BTreeSet 保持有序
    #[pyo3(get, set)]
    pub codes: BTreeSet<u64>,
    /// 精度等级
    #[pyo3(get, set)]
    pub level: usize,
}

#[pymethods]
impl GeoSotRegion {
    /// 创建新的 GeoSOT 区域
    #[new]
    pub fn new(level: usize) -> Self {
        Self {
            codes: BTreeSet::new(),
            level,
        }
    }

    /// 从编码向量创建区域
    #[staticmethod]
    pub fn from_codes(codes: Vec<u64>, level: usize) -> Self {
        let mut region = Self::new(level);
        for code in codes {
            region.codes.insert(code);
        }
        region
    }

    /// 从矩形区域创建 GeoSOT 编码集合
    #[staticmethod]
    pub fn from_rectangle(min_lng: f64, min_lat: f64, max_lng: f64, max_lat: f64, level: usize) -> Self {
        let mut region = Self::new(level);

        // 计算步长 - 使用更精确的步长计算
        let step = match level {
            l if l >= 0 && l <= 9 => 2.0f64.powf(9.0 - l as f64),
            l if l >= 10 && l <= 15 => 2.0f64.powf(15.0 - l as f64) / 60.0,
            l if l >= 16 && l <= 32 => 2.0f64.powf(21.0 - l as f64) / 3600.0,
            _ => 0.001, // 默认步长
        };

        // 遍历矩形区域内的所有网格点
        let mut lat = min_lat;
        while lat <= max_lat {
            let mut lng = min_lng;
            while lng <= max_lng {
                let code = get_code(lng, lat, level);
                region.codes.insert(code);
                lng += step;
            }
            lat += step;
        }

        region
    }

    /// 从多边形创建 GeoSOT 编码集合（简化版本，使用包围盒）
    #[staticmethod]
    pub fn from_polygon(points: Vec<(f64, f64)>, level: usize) -> Self {
        if points.is_empty() {
            return Self::new(level);
        }

        // 计算包围盒
        let min_lng = points.iter().map(|(lng, _)| *lng).fold(f64::INFINITY, f64::min);
        let max_lng = points.iter().map(|(lng, _)| *lng).fold(f64::NEG_INFINITY, f64::max);
        let min_lat = points.iter().map(|(_, lat)| *lat).fold(f64::INFINITY, f64::min);
        let max_lat = points.iter().map(|(_, lat)| *lat).fold(f64::NEG_INFINITY, f64::max);

        // 使用包围盒创建区域（实际应用中可以加入点在多边形内的判断）
        Self::from_rectangle(min_lng, min_lat, max_lng, max_lat, level)
    }

    /// 添加单个编码
    pub fn add_code(&mut self, code: u64) {
        self.codes.insert(code);
    }

    /// 添加经纬度点
    pub fn add_point(&mut self, lng: f64, lat: f64) {
        let code = get_code(lng, lat, self.level);
        self.codes.insert(code);
    }

    /// 检查是否包含指定编码
    pub fn contains(&self, code: u64) -> bool {
        self.codes.contains(&code)
    }

    /// 检查是否包含指定点
    pub fn contains_point(&self, lng: f64, lat: f64) -> bool {
        let code = get_code(lng, lat, self.level);
        self.contains(code)
    }

    /// 获取区域大小（编码数量）
    pub fn size(&self) -> usize {
        self.codes.len()
    }

    /// 判断区域是否为空
    pub fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    /// 获取区域的字符串表示
    pub fn to_strings(&self) -> Vec<String> {
        self.codes.iter().map(|&code| to_string(code, self.level)).collect()
    }

    /// 获取区域的经纬度点集合
    pub fn to_coordinates(&self) -> Vec<(f64, f64)> {
        self.codes.iter().map(|&code| decode_by_geomgrid(code)).collect()
    }

    /// 计算两个区域的交集
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 两个区域的交集
    pub fn intersection(&self, other: &GeoSotRegion) -> GeoSotRegion {
        // 确保精度等级相同
        if self.level != other.level {
            panic!("Cannot compute intersection of regions with different levels");
        }

        let intersection_codes: BTreeSet<u64> = self.codes
            .intersection(&other.codes)
            .cloned()
            .collect();

        GeoSotRegion {
            codes: intersection_codes,
            level: self.level,
        }
    }

    /// 计算两个区域的并集
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 两个区域的并集
    pub fn union(&self, other: &GeoSotRegion) -> GeoSotRegion {
        if self.level != other.level {
            panic!("Cannot compute union of regions with different levels");
        }

        let union_codes: BTreeSet<u64> = self.codes
            .union(&other.codes)
            .cloned()
            .collect();

        GeoSotRegion {
            codes: union_codes,
            level: self.level,
        }
    }

    /// 计算两个区域的差集（self - other）
    ///
    /// # 参数
    /// * `other` - 要减去的区域
    ///
    /// # 返回
    /// self 中不在 other 中的部分
    pub fn difference(&self, other: &GeoSotRegion) -> GeoSotRegion {
        if self.level != other.level {
            panic!("Cannot compute difference of regions with different levels");
        }

        let difference_codes: BTreeSet<u64> = self.codes
            .difference(&other.codes)
            .cloned()
            .collect();

        GeoSotRegion {
            codes: difference_codes,
            level: self.level,
        }
    }

    /// 计算两个区域的对称差集（并集减去交集）
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 两个区域的对称差集
    pub fn symmetric_difference(&self, other: &GeoSotRegion) -> GeoSotRegion {
        if self.level != other.level {
            panic!("Cannot compute symmetric difference of regions with different levels");
        }

        let sym_diff_codes: BTreeSet<u64> = self.codes
            .symmetric_difference(&other.codes)
            .cloned()
            .collect();

        GeoSotRegion {
            codes: sym_diff_codes,
            level: self.level,
        }
    }

    /// 计算区域在指定范围内的补集
    ///
    /// # 参数
    /// * `universe` - 全集区域
    ///
    /// # 返回
    /// 当前区域在全集中的补集
    pub fn complement(&self, universe: &GeoSotRegion) -> GeoSotRegion {
        if self.level != universe.level {
            panic!("Cannot compute complement with different levels");
        }

        universe.difference(self)
    }

    /// 判断当前区域是否是另一个区域的子集
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 如果当前区域是 other 的子集则返回 true
    pub fn is_subset(&self, other: &GeoSotRegion) -> bool {
        if self.level != other.level {
            return false;
        }
        self.codes.is_subset(&other.codes)
    }

    /// 判断当前区域是否是另一个区域的超集
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 如果当前区域是 other 的超集则返回 true
    pub fn is_superset(&self, other: &GeoSotRegion) -> bool {
        if self.level != other.level {
            return false;
        }
        self.codes.is_superset(&other.codes)
    }

    /// 判断两个区域是否相交
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 如果两个区域有交集则返回 true
    pub fn intersects(&self, other: &GeoSotRegion) -> bool {
        if self.level != other.level {
            return false;
        }
        !self.codes.is_disjoint(&other.codes)
    }

    /// 判断两个区域是否不相交
    ///
    /// # 参数
    /// * `other` - 另一个区域
    ///
    /// # 返回
    /// 如果两个区域没有交集则返回 true
    pub fn is_disjoint(&self, other: &GeoSotRegion) -> bool {
        if self.level != other.level {
            return true;
        }
        self.codes.is_disjoint(&other.codes)
    }
}

#[pymodule]
pub fn spatial_analysis(_py: Python, m: &PyModule) -> PyResult<()> {
    /// 计算两个区域的 Jaccard 相似度
    #[pyfn(m)]
    #[pyo3(name = "jaccard_similarity")]
    fn jaccard_similarity_py(region1: &GeoSotRegion, region2: &GeoSotRegion) -> f64 {
        if region1.level != region2.level {
            return 0.0;
        }

        let intersection_size = region1.intersection(region2).size();
        let union_size = region1.union(region2).size();

        if union_size == 0 {
            return 1.0; // 两个空集的相似度为1
        }

        intersection_size as f64 / union_size as f64
    }

    /// 计算两个区域的重叠率
    #[pyfn(m)]
    #[pyo3(name = "overlap_ratio")]
    fn overlap_ratio_py(region1: &GeoSotRegion, region2: &GeoSotRegion) -> f64 {
        if region1.level != region2.level || region1.is_empty() {
            return 0.0;
        }

        let intersection_size = region1.intersection(region2).size();
        intersection_size as f64 / region1.size() as f64
    }

    /// 计算区域的紧密度（连通性度量）
    #[pyfn(m)]
    #[pyo3(name = "compactness")]
    fn compactness_py(region: &GeoSotRegion) -> f64 {
        if region.size() <= 1 {
            return 1.0;
        }

        let codes: Vec<u64> = region.codes.iter().cloned().collect();
        let mut adjacent_pairs = 0;
        let total_pairs = codes.len() * (codes.len() - 1) / 2;

        for i in 0..codes.len() {
            for j in (i + 1)..codes.len() {
                if are_adjacent_codes(codes[i], codes[j], region.level) {
                    adjacent_pairs += 1;
                }
            }
        }

        if total_pairs == 0 {
            1.0
        } else {
            adjacent_pairs as f64 / total_pairs as f64
        }
    }

    Ok(())
}


/// 简化的相邻编码判断函数
/// 实际实现中应该根据 GeoSOT 编码的空间结构来判断
fn are_adjacent_codes(code1: u64, code2: u64, _level: usize) -> bool {
    // 这里是一个简化的实现，实际应该根据莫顿码的性质来判断相邻性
    let diff = if code1 > code2 { code1 - code2 } else { code2 - code1 };
    // 相邻的莫顿码通常差值较小
    diff <= 4 // 简化判断
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geosot_cell_creation() {
        let cell = GeoSotCell::from_coords(116.397, 39.916, 20);
        assert_eq!(cell.level, 20);
        println!("Cell: {}", cell.to_string());
    }

    #[test]
    fn test_parent_child_relationship() {
        let cell = GeoSotCell::from_coords(116.397, 39.916, 20);
        let parent = cell.parent().unwrap();
        assert_eq!(parent.level, 19);
        
        let children = parent.children();
        assert_eq!(children.len(), 4);
        assert!(children.iter().any(|c| c.is_descendant_of(&parent)));
    }
    #[test]
    fn test_region_creation() {
        let mut region = GeoSotRegion::new(20);
        region.add_point(116.0, 39.0); // 北京
        region.add_point(121.0, 31.0); // 上海
        
        assert_eq!(region.size(), 2);
        assert!(region.contains_point(116.0, 39.0));
    }

    #[test]
    fn test_intersection() {
        let region1 = GeoSotRegion::from_codes(vec![1, 2, 3, 4], 20);
        let region2 = GeoSotRegion::from_codes(vec![3, 4, 5, 6], 20);
        
        let intersection = region1.intersection(&region2);
        assert_eq!(intersection.size(), 2);
        assert!(intersection.contains(3));
        assert!(intersection.contains(4));
    }

    #[test]
    fn test_union() {
        let region1 = GeoSotRegion::from_codes(vec![1, 2, 3], 20);
        let region2 = GeoSotRegion::from_codes(vec![3, 4, 5], 20);
        
        let union = region1.union(&region2);
        assert_eq!(union.size(), 5);
    }

    #[test]
    fn test_difference() {
        let region1 = GeoSotRegion::from_codes(vec![1, 2, 3, 4], 20);
        let region2 = GeoSotRegion::from_codes(vec![2, 3], 20);
        
        let difference = region1.difference(&region2);
        assert_eq!(difference.size(), 2);
        assert!(difference.contains(1));
        assert!(difference.contains(4));
    }

    #[test]
    fn test_spatial_relationships() {
        let region1 = GeoSotRegion::from_codes(vec![1, 2, 3], 20);
        let region2 = GeoSotRegion::from_codes(vec![2, 3, 4], 20);
        let region3 = GeoSotRegion::from_codes(vec![1, 2], 20);
        
        assert!(region1.intersects(&region2));
        assert!(region3.is_subset(&region1));
        assert!(region1.is_superset(&region3));
    }

    #[test]
    fn test_jaccard_similarity() {
        let region1 = GeoSotRegion::from_codes(vec![1, 2, 3], 20);
        let region2 = GeoSotRegion::from_codes(vec![2, 3, 4], 20);
        
        let similarity = spatial_analysis::jaccard_similarity(&region1, &region2);
        assert!((similarity - 0.5).abs() < 1e-10); // 交集2个，并集4个，相似度0.5
    }
}