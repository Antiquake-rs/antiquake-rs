//allow for experimental drain_filter
#![feature(drain_filter)]

#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate cgmath;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate rand;
extern crate regex;
extern crate rodio;
extern crate winit;
extern crate serde; 
extern crate serde_json; 
extern crate strum;
 
extern crate nom; 
extern crate png;

pub mod client;
pub mod common;
 