import geosot

def test_get_code():
    lng = 116.397428
    lat = 39.90923
    precision = 32
    code = geosot.get_code(lng, lat, precision)
    assert code == 526548384406042203

def test_decode_by_geomgrid():
    code = 526548384406042203
    lng, lat = geosot.decode_by_geomgrid(code)
    assert abs(lng - 116.397428) < 1e-6
    assert abs(lat - 39.90923) < 1e-6

def test_dec2code():
    dec = 76.233
    precision = 32
    code = geosot.dec2code(dec, precision)
    assert code == 639358566

def test_code2dec():
    code = 639358566
    dec = geosot.code2dec(code)
    assert abs(dec - 76.23299994574653) < 1e-9

if __name__ == "__main__":
    test_get_code()
    test_decode_by_geomgrid()
    test_dec2code()
    test_code2dec()
    print("All core tests passed!")
