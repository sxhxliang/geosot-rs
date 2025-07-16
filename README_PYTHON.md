# 使用 Python 绑定

本项目提供了 `geosot` crate 的 Python 绑定。您可以使用 `pip` 安装它，并像使用普通的 Python 包一样使用它。

## 安装

要安装 `geosot` Python 包，请确保您已经安装了 Rust 和 `maturin`。然后，从项目的根目录运行以下命令：

```bash
pip install .
```

## 使用示例

以下是一些如何使用 `geosot` Python 绑定的示例。

### 核心功能

```python
import geosot

# 将经纬度编码为 geomgrid
lng = 116.397428
lat = 39.90923
precision = 32
code = geosot.get_code(lng, lat, precision)
print(f"Geomgrid code: {code}")

# 将 geomgrid 解码为经纬度
decoded_lng, decoded_lat = geosot.decode_by_geomgrid(code)
print(f"Decoded longitude: {decoded_lng}, latitude: {decoded_lat}")
```

### 空间功能

```python
from geosot import GeoSotCell, GeoSotRegion, spatial_analysis

# 创建 GeoSotCell
cell = GeoSotCell.from_coords(116.397, 39.916, 20)
print(f"Cell level: {cell.level}")
print(f"Cell string: {cell.to_string_py()}")

# 获取父单元和子单元
parent = cell.parent()
print(f"Parent level: {parent.level}")
children = parent.children()
print(f"Number of children: {len(children)}")

# 创建 GeoSotRegion
region1 = GeoSotRegion.from_codes([1, 2, 3, 4], 20)
region2 = GeoSotRegion.from_codes([3, 4, 5, 6], 20)

# 计算交集和并集
intersection = region1.intersection(region2)
print(f"Intersection size: {intersection.size()}")
union = region1.union(region2)
print(f"Union size: {union.size()}")

# 空间分析
similarity = spatial_analysis.jaccard_similarity(region1, region2)
print(f"Jaccard similarity: {similarity}")
```
