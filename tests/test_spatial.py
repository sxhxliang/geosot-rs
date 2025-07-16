import geosot
from geosot import GeoSotCell, GeoSotRegion, spatial_analysis

def test_geosot_cell():
    cell = GeoSotCell.from_coords(116.397, 39.916, 20)
    assert cell.level == 20
    assert cell.to_string_py().startswith("G")

    parent = cell.parent()
    assert parent.level == 19

    children = parent.children()
    assert len(children) == 4
    assert any(c.is_descendant_of(parent) for c in children)

def test_geosot_region():
    region = GeoSotRegion(20)
    region.add_point(116.0, 39.0)
    region.add_point(121.0, 31.0)
    assert region.size() == 2
    assert region.contains_point(116.0, 39.0)

    region1 = GeoSotRegion.from_codes([1, 2, 3, 4], 20)
    region2 = GeoSotRegion.from_codes([3, 4, 5, 6], 20)

    intersection = region1.intersection(region2)
    assert intersection.size() == 2
    assert 3 in intersection.codes
    assert 4 in intersection.codes

    union = region1.union(region2)
    assert union.size() == 6

def test_spatial_analysis():
    region1 = GeoSotRegion.from_codes([1, 2, 3], 20)
    region2 = GeoSotRegion.from_codes([2, 3, 4], 20)

    similarity = spatial_analysis.jaccard_similarity(region1, region2)
    assert abs(similarity - 0.5) < 1e-9

if __name__ == "__main__":
    test_geosot_cell()
    test_geosot_region()
    test_spatial_analysis()
    print("All spatial tests passed!")
