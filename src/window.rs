// extern crate glium;

use glium::{Display, DisplayBuild, Surface};
use glium::backend::glutin_backend::PollEventsIter;

/// Main object used in Peregrine
pub struct Window {
    glium_display: Display
}

impl Window {
    pub fn new(options: WindowBuilder) -> Window {
        let mut display_builder = ::glium::glutin::WindowBuilder::new()
            .with_dimensions(options.dimension.0, options.dimension.1)
            .with_title(options.title);

        //FIXME: Re-enable this once we have a way for user to quit once fullscreen
        // if options.fullscreen {
        //     display_builder = display_builder.with_fullscreen(::glium::glutin::get_primary_monitor());
        // }

        let display = display_builder.build_glium().unwrap();

        return Window {glium_display: display};
    }

    /// simple wrapper around glium::Display::poll_events
    pub fn poll_events(&self) -> PollEventsIter {
        return self.glium_display.poll_events();
    }

    /// main render function
    pub fn render(&self) {
        // could put this on a separate render thread...
    }
}


/// Builder for creating the Window
#[derive(Debug)]
pub struct WindowBuilder {
    position: (i32, i32),
    dimension: (u32, u32),
    title: String,
    fullscreen: bool    
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        return WindowBuilder { position: (0,0), 
                               dimension: (1024,768), 
                               title: String::from("Peregrine"),
                               fullscreen: false }
    }

    pub fn with_position(&mut self, x: i32, y: i32) -> &mut WindowBuilder {
        self.position = (x,y);
        return self;
    }

    pub fn with_dimensions(&mut self, x: u32, y: u32) -> &mut WindowBuilder {
        self.dimension = (x,y);
        return self;
    }

    pub fn with_title(&mut self, title: String) -> &mut WindowBuilder {
        self.title = title;
        return self;
    }

    pub fn with_fullscreen(&mut self) -> &mut WindowBuilder {
        self.fullscreen = true;
        return self;
    }

    pub fn build(self) -> ::Result<Window> {
        return Ok(Window::new(self));
    }
}