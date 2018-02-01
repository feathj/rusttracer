extern crate mylib;

use tracer::Tracer;

use base::Color;
use base::Vector3;

use primitives::Sphere;

pub fn main() {
	let tracer = Tracer::new(1024, 768, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -5.0));
	tracer.set_world_color(Color::new(0.282, 0.475, 0.745));
	
	// Test sphere 1
	let s1 = Sphere::new(Vector3::new(0.0, 1.0, -10.0), 2.0);
	// Chrome
	s1.surface.ambient = Color::new(0.25, 0.25, 0.25);
	s1.surface.diffuse = Color::new(0.4, 0.4, 0.4);
	s1.surface.specular = Color::new(0.774597, 0.774597, 0.774597);
	s1.surface.shiny = 50.0;
	s1.surface.reflect = 0.0;

    tracer.add_primitive(Box::new(s1));

    tracer.trace(on_calc_pixel);
}
