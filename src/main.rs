

use glam;
use wgpu;
use winit;

use bytemuck::{Pod, Zeroable};
use std::{borrow::Cow, f32::consts, future::Future, mem, pin::Pin, task};
use wgpu::util::DeviceExt;


#[macro_use]
mod parse;
pub mod pak;
pub mod error;
//pub mod render;
pub mod bsp;
pub mod bitset;
pub mod render;

#[macro_use]
extern crate error_chain;

use std::time::{Instant, Duration};
use std::rc::Rc;
use std::io::Cursor;
 
use render::renderspace::cube::CubeRenderspace;
use render::renderspace::cube::framework; //have to get it through there 
//use render::framework::FrameworkInstance;


//Execution begins here 
fn main() {

    //load the pak file 
    let filename:&str = "id1/pak0.pak";
   // let pakRc = Rc::new(pak::PackFile::new(filename).expect("Unable to load pak0"));

    let pak =pak::PackFile::new(filename).expect("Unable to load pak0");

    let start = bsp::BspFile::parse(
        &mut Cursor::new(pak.file("maps/start.bsp").unwrap())
    ).unwrap();

    println!("Loaded pak");

    /*
    let mut renderer = render::Renderer::new(
        pak.clone(), start,
        adapter, surface,
        size,
    ).unwrap();
    */



    //run is framework  453 

    //start is framework 26 0

 

    framework::run::<CubeRenderspace>("Soulgate");
    framework::run::<CubeRenderspace>("Soulgate2");
}



