extern crate rtlib;
extern crate sdl2;

use rtlib::tracer::Tracer;
use rtlib::base::Color;
use rtlib::base::Light;
use rtlib::base::Vector3;
use rtlib::primitives::Sphere;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color as SColor;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

fn main() {
    let mut tracer = Tracer::new(SCREEN_WIDTH as i64, SCREEN_HEIGHT as i64, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -5.0));
    tracer.set_world_color(Color::new(0.282, 0.475, 0.745));

    // Test sphere 1
    let mut s1 = Sphere::new(Vector3::new(0.0, 1.0, -10.0), 2.0);
    // Chrome
    s1.surface.ambient = Color::new(0.25, 0.25, 0.25);
    s1.surface.diffuse = Color::new(0.4, 0.4, 0.4);
    s1.surface.specular = Color::new(0.774597, 0.774597, 0.774597);
    s1.surface.shiny = 50.0;
    s1.surface.reflect = 0.0;
    tracer.add_primitive(Box::new(s1));

	// Test sphere 2
	let mut s2 = Sphere::new(Vector3::new(1.5, 0.0, -5.0), 1.0);
	// Emerald
	s2.surface.ambient = Color::new(0.0215, 0.1745, 0.0215);
	s2.surface.diffuse = Color::new(0.07568, 0.61424, 0.07568);
	s2.surface.specular = Color::new(0.633, 0.727811, 0.633);
	s2.surface.shiny = 50.0;
	s2.surface.reflect = 0.0;
	tracer.add_primitive(Box::new(s2));

	// Test sphere 3
	let mut s3 = Sphere::new(Vector3::new(-1.5, 0.5, -5.0), 1.0);
	// Ruby
	s3.surface.ambient = Color::new(0.1745, 0.01175, 0.01175);
	s3.surface.diffuse = Color::new(0.61424, 0.04136, 0.04136);
	s3.surface.specular = Color::new(0.727811, 0.626959, 0.626959);
	s3.surface.shiny = 50.0;
	s3.surface.reflect = 0.0;
	tracer.add_primitive(Box::new(s3));


	// Test light 1
	let mut l1 = Light::new(0.0, 10.0, 10.0);
	l1.ambient = Color::new(0.1, 0.1, 0.1);
	l1.diffuse = Color::new(1.0, 1.0, 1.0);
	l1.specular = Color::new(1.0, 1.0, 1.0);
	tracer.add_light(l1);

    tracer.trace();

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    //let mut events = sdl_context.event_pump().unwrap();

    'main: loop {
        for t in tracer.pixels.iter() {
            let c = SColor::RGB((t.2.r * 255.0).round() as u8, (t.2.g * 255.0).round() as u8, (t.2.b * 255.0).round() as u8);
            canvas.pixel(t.0 as i16, t.1 as i16, c).unwrap();
        }
        canvas.present();
    }
}
