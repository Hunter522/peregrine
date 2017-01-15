// TODO: remove these when everything is implemented
#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate glium;

// expose glutin backend
pub use glium::backend::glutin_backend::glutin;


pub mod window;



/// Specialization of std::result
type Result<T> = std::result::Result<T, &'static str>;


// Unit tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
