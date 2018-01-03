extern crate rusttracer;

use rusttracer::vector3::Vector3;

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

    assert_eq!(res, Vector3::new(-3.0, 6.0, -3.0));
}

#[test]
fn normalize_works() {
    let mut v1 = Vector3::new(1.0, 2.0, 3.0);
    let mag = 1.0 / v1.magnitude();

    v1.normalize();

    assert_eq!(v1, Vector3::new(1.0 * mag, 2.0 * mag, 3.0 * mag));
}

#[test]
fn add_works() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let res = v1 + v2;

    assert_eq!(res, Vector3::new(5.0, 7.0, 9.0));
}

#[test]
fn sub_works() {
    let v1 = Vector3::new(4.0, 5.0, 6.0);
    let v2 = Vector3::new(1.0, 2.0, 3.0);
    let res = v1 - v2;

    assert_eq!(res, Vector3::new(3.0, 3.0, 3.0));
}

#[test]
fn mul_works() {
    let v1 = Vector3::new(4.0, 5.0, 6.0);
    let v2 = Vector3::new(1.0, 2.0, 3.0);
    let res = v1 * v2;

    assert_eq!(res, Vector3::new(4.0, 10.0, 18.0));
}

#[test]
fn mul_scalar_works() {
    let v1 = Vector3::new(4.0, 5.0, 6.0);
    let res = v1 * 2.0;

    assert_eq!(res, Vector3::new(8.0, 10.0, 12.0));
}
