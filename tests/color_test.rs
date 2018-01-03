extern crate rusttracer;

use rusttracer::color::Color;

#[test]
fn add_works() {
    let c1 = Color::new(0.1, 0.2, 0.3);
    let c2 = Color::new(0.4, 0.5, 0.6);

    let res = c1 + c2;

    assert_eq!(res, Color::new(0.5, 0.7, 0.9));
}

#[test]
fn mul_works() {
    let c1 = Color::new(0.1, 0.2, 0.3);
    let c2 = Color::new(0.4, 0.5, 0.6);

    let res = c1 * c2;

    assert_eq!(res, Color::new(0.04, 0.1, 0.18));
}

#[test]
fn mul_scalar_works() {
    let c1 = Color::new(0.1, 0.2, 0.3);

    let res = c1 * 0.1;

    assert_eq!(res, Color::new(0.01, 0.02, 0.03));
}
