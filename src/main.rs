extern crate piston;
extern crate pistoncore_glutin_window;
extern crate graphics;
extern crate piston2d_opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, PressEvent, Button, Key};
use pistoncore_glutin_window::GlutinWindow as Window;
use piston2d_opengl_graphics::{GlGraphics, OpenGL};
use graphics::rectangle;

struct Vehicle {
    x: f64,
    y: f64,
}

impl Vehicle {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn draw(&self, gl: &mut GlGraphics, args: &piston::input::RenderArgs) {
        const VEHICLE_SIZE: f64 = 50.0;

        let square = rectangle::square(self.x, self.y, VEHICLE_SIZE);
        let red = [1.0, 0.0, 0.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            rectangle(red, square, transform, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Smart Intersection", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: Window = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut vehicles = Vec::new();

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            });

            for vehicle in &vehicles {
                vehicle.draw(&mut gl, &args);
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => {
                    println!("Arrow Up pressed: Generate vehicle from South to North");
                    vehicles.push(Vehicle::new(100.0, 100.0));
                },
                Key::Down => {
                    println!("Arrow Down pressed: Generate vehicle from North to South");
                    vehicles.push(Vehicle::new(200.0, 200.0));
                },
                Key::Left => {
                    println!("Arrow Left pressed: Generate vehicle from East to West");
                    vehicles.push(Vehicle::new(300.0, 300.0));
                },
                Key::Right => {
                    println!("Arrow Right pressed: Generate vehicle from West to East");
                    vehicles.push(Vehicle::new(400.0, 400.0));
                },
                Key::R => {
                    println!("R key pressed: Generate random vehicle");
                    vehicles.push(Vehicle::new(150.0, 150.0));
                },
                Key::Escape => {
                    println!("Escape key pressed: Exiting");
                    break;
                },
                _ => (),
            }
        }
    }
}
