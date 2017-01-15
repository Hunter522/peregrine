#[macro_use]
extern crate peregrine;

use peregrine::window::{Window, WindowBuilder};

fn main() {

    // create a Window and display it
    let display = WindowBuilder::new().build().unwrap();
    
    // use layer API
        
    // use entity API to add entities to a layer

    loop {
        // update state if necessary    

        // expose the glium::glutin::Events
        for ev in display.poll_events() {
            match ev {
                peregrine::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}