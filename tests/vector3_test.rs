extern crate rustcaster;

use rustcaster::vector3::Vector3;

#[test]
fn dot_product_works() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    assert_eq!(v1.dot_product(v2), 32.0);
}

#[test]
fn cross_product_works() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let res = v1.cross_product(v2);

    assert_eq!(res.x, 0.0);
    assert_eq!(res.y, 0.0);
    assert_eq!(res.z, 0.0);
}
