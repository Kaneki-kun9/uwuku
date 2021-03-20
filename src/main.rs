#![deny(missing_docs)]

//! My super cool uwuku game.

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

mod bullshit_window;

use graphics::Graphics;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};



fn main() {
    println!("Hello, world!");
    println!("Piston Tutorial");

    let opengl = OpenGL::V3_2;

    let setting: WindowSettings = WindowSettings::new("Sudoku", [512; 2]).exit_on_esc(true).fullscreen(true);
    
    let mut window: GlutinWindow = setting.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c,g| {
                use graphics::{clear, Rectangle};
                use graphics::types::Color;

                clear([1.0; 4], g);
                let rec = Rectangle::new([1.0,0.0,0.0,1.0]);
                rec.draw([2.0,2.0,64.0,64.0], &c.draw_state, c.transform, g);
            });
        }
    }

    println!("{}", setting.get_exit_on_esc());
}
