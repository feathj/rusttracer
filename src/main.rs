extern crate rtlib;
extern crate sdl2;

use rtlib::tracer::Tracer;
use rtlib::base::Color;
use rtlib::base::Light;
use rtlib::base::Vector3;
use rtlib::primitives::Sphere;
use rtlib::primitives::Plane;

use sdl2::gfx::primitives::DrawRenderer;
//use sdl2::pixels::PixelFormatEnum;
//use sdl2::rect::Rect;
use sdl2::pixels::Color as SColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn convert_to_sdl_color(a_color:Color) -> SColor {
    let r = (a_color.r * 255.0).round().min(255.0);
    let g = (a_color.g * 255.0).round().min(255.0);
    let b = (a_color.b * 255.0).round().min(255.0);

    return SColor::RGB(r as u8, g as u8, b as u8);
}

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

	// Test plane
	let mut p1 = Plane::new(Vector3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
	// Black
	p1.surface.ambient = Color::new(0.05375, 0.05, 0.06625);
	p1.surface.diffuse = Color::new(0.18275, 0.17, 0.22525);
	p1.surface.specular = Color::new(0.332741, 0.328634, 0.346435);
	p1.surface.shiny = 86.0;
	p1.surface.reflect = 0.3;
	tracer.add_primitive(Box::new(p1));

    // Test light 1
	let mut l1 = Light::new(-20.0, 10.0, 20.0);
	l1.ambient = Color::new(0.001, 0.001, 0.001);
	l1.diffuse = Color::new(1.0, 1.0, 1.0);
	l1.specular = Color::new(1.0, 1.0, 1.0);
    tracer.add_light(l1);

    // Test light 2
	let mut l2 = Light::new(20.0, 50.0, 20.0);
	l2.ambient = Color::new(0.001, 0.001, 0.001);
	l2.diffuse = Color::new(1.0, 1.0, 1.0);
	l2.specular = Color::new(1.0, 1.0, 1.0);
    tracer.add_light(l2);

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in events.poll_iter() {
            match event {

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => {
                            break 'main;
                        },
                        Keycode::Up => {
                            let mut eye = tracer.get_camera_eye();
                            eye.y += 0.25;
                            tracer.set_camera_eye(eye);
                            let mut look_at = tracer.get_camera_look_at();
                            look_at.y += 0.25;
                            tracer.set_camera_look_at(look_at);
                        }
                        Keycode::Down => {
                            let mut eye = tracer.get_camera_eye();
                            eye.y -= 0.25;
                            tracer.set_camera_eye(eye);
                            let mut look_at = tracer.get_camera_look_at();
                            look_at.y -= 0.25;
                            tracer.set_camera_look_at(look_at);
                        }
                        Keycode::Left => {
                            let mut eye = tracer.get_camera_eye();
                            eye.x -= 0.25;
                            tracer.set_camera_eye(eye);
                            let mut look_at = tracer.get_camera_look_at();
                            look_at.x -= 0.25;
                            tracer.set_camera_look_at(look_at);
                        }
                        Keycode::Right => {
                            let mut eye = tracer.get_camera_eye();
                            eye.x += 0.25;
                            tracer.set_camera_eye(eye);
                            let mut look_at = tracer.get_camera_look_at();
                            look_at.x += 0.25;
                            tracer.set_camera_look_at(look_at);
                        }
                        _ => {}
                    }
                },

                Event::MouseButtonDown {x, y, ..} => {
                    println!("mouse btn down at ({},{})", x, y);
                },

                _ => {}
            }
        }

        for t in tracer.get_pixels().iter() {
            canvas.pixel(t.0 as i16, (SCREEN_HEIGHT - t.1 as u32) as i16, convert_to_sdl_color(t.2)).unwrap();
        }
        canvas.present();
    }
}
