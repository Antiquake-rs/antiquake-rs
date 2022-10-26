

mod game;
mod menu;
pub mod capture;
pub mod trace;

use glam;
use wgpu;
use winit;

use bytemuck::{Pod, Zeroable};
use std::{borrow::Cow, f32::consts, future::Future, mem, pin::Pin, task};
use wgpu::util::DeviceExt;


use std::{
    cell::{Ref, RefCell, RefMut},
    fs::File,
    io::{Cursor, Read, Write},
    net::SocketAddr,
    path::{Path, PathBuf},
    process::exit,
    rc::Rc,
};

use structopt::StructOpt;
 
use game::Game;

use soulgateengine::client; 
 
use soulgateengine::client::input::{Input,InputFocus};
use soulgateengine::common::console::{CvarRegistry,CmdRegistry,Console};
use soulgateengine::common::vfs::Vfs;
use soulgateengine::common::host::{Host, Program};
//use soulgateengine::client::input::InputFocus::{Game};
use soulgateengine::client::render::{self,UiRenderer,GraphicsState,Extent2d,DIFFUSE_ATTACHMENT_FORMAT};

use winit::window::CursorGrabMode;

use soulgateengine::client::menu::Menu;
use soulgateengine::client::Client;
use soulgateengine::common::default_base_dir;


#[macro_use]
extern crate error_chain;

use std::time::{Instant};

use chrono::Duration;
 
 
//use soulgateengine::render::renderspace::level::LevelRenderspace;
//use soulgateengine::render::renderspace::level::framework; //have to get it through there 

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::Window,
};


//use render::framework::FrameworkInstance;







struct ClientProgram {
    vfs: Rc<Vfs>,
    cvars: Rc<RefCell<CvarRegistry>>,
    cmds: Rc<RefCell<CmdRegistry>>,
    console: Rc<RefCell<Console>>,
    menu: Rc<RefCell<Menu>>,

    window: Window,
    window_dimensions_changed: bool,

    surface: wgpu::Surface,
   // swap_chain: RefCell<wgpu::SwapChain>,
    gfx_state: RefCell<GraphicsState>,
    ui_renderer: Rc<UiRenderer>,

    game: Game,
    input: Rc<RefCell<Input>>,
}

impl ClientProgram {
    pub async fn new(window: Window, base_dir: Option<PathBuf>, trace: bool) -> ClientProgram {
        let vfs = Vfs::with_base_dir(base_dir.unwrap_or( default_base_dir()));

        let con_names = Rc::new(RefCell::new(Vec::new()));

        let cvars = Rc::new(RefCell::new(CvarRegistry::new(con_names.clone())));
        client::register_cvars(&cvars.borrow()).unwrap();
        render::register_cvars(&cvars.borrow());

        let cmds = Rc::new(RefCell::new(CmdRegistry::new(con_names)));
        // TODO: register commands as other subsystems come online

        let console = Rc::new(RefCell::new(Console::new(cmds.clone(), cvars.clone())));
        let menu = Rc::new(RefCell::new(menu::build_main_menu().unwrap()));

        let input = Rc::new(RefCell::new(Input::new(
            InputFocus::Console,
            console.clone(),
            menu.clone(),
        )));
        input.borrow_mut().bind_defaults();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::PUSH_CONSTANTS
                        | wgpu::Features::TEXTURE_BINDING_ARRAY
                      //  | wgpu::Features::TEXTURE_ARRAY_DYNAMIC_INDEXING
                        | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,//TEXTURE_ARRAY_NON_UNIFORM_INDEXING,
                    limits: wgpu::Limits {
                        max_sampled_textures_per_shader_stage: 256,
                        max_uniform_buffer_binding_size: 65536,
                        max_push_constant_size: 256,
                        ..Default::default()
                    },
                },
                if trace {
                    Some(Path::new("./trace/"))
                } else {
                    None
                },
            )
            .await
            .unwrap();
        let size: Extent2d = window.inner_size().into();
      /*  let swap_chain = RefCell::new(device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: DIFFUSE_ATTACHMENT_FORMAT,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Immediate,
            },
        ));*/

        let vfs = Rc::new(vfs);

        // TODO: warn user if r_msaa_samples is invalid
        let mut sample_count = cvars.borrow().get_value("r_msaa_samples").unwrap_or(2.0) as u32;
        if !&[2, 4].contains(&sample_count) {
            sample_count = 2;
        }

        let gfx_state = GraphicsState::new(device, queue, size, sample_count, vfs.clone()).unwrap();
        let ui_renderer = Rc::new(UiRenderer::new(&gfx_state, &menu.borrow()));

        // TODO: factor this out
        // implements "exec" command
        let exec_vfs = vfs.clone();
        let exec_console = console.clone();
        cmds.borrow_mut().insert_or_replace(
            "exec",
            Box::new(move |args| {
                match args.len() {
                    // exec (filename): execute a script file
                    1 => {
                        let mut script_file = match exec_vfs.open(args[0]) {
                            Ok(s) => s,
                            Err(e) => {
                                return format!("Couldn't exec {}: {:?}", args[0], e);
                            }
                        };

                        let mut script = String::new();
                        script_file.read_to_string(&mut script).unwrap();

                        exec_console.borrow().stuff_text(script);
                        String::new()
                    }

                    _ => format!("exec (filename): execute a script file"),
                }
            }),
        ).unwrap();

        // this will also execute config.cfg and autoexec.cfg (assuming an unmodified quake.rc)
        console.borrow().stuff_text("exec quake.rc\n");

        let client = Client::new(
            vfs.clone(),
            cvars.clone(),
            cmds.clone(),
            console.clone(),
            input.clone(),
            &gfx_state,
            &menu.borrow(),
        );

        let game = Game::new(cvars.clone(), cmds.clone(), input.clone(), client).unwrap();

        ClientProgram {
            vfs,
            cvars,
            cmds,
            console,
            menu,
            window,
            window_dimensions_changed: false,
            surface,
           // swap_chain,
            gfx_state: RefCell::new(gfx_state),
            ui_renderer,
            game,
            input,
        }
    }

    /// Builds a new swap chain with the specified present mode and the window's current dimensions.
    fn recreate_swap_chain(&self, present_mode: wgpu::PresentMode) {
        let winit::dpi::PhysicalSize { width, height } = self.window.inner_size();
        let swap_chain = self.gfx_state.borrow().device().create_swap_chain(
            &self.surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: DIFFUSE_ATTACHMENT_FORMAT,
                width,
                height,
                present_mode,
            },
        );
        let _ = self.swap_chain.replace(swap_chain);
    }

    fn render(&mut self) {
        let swap_chain_output = self.swap_chain.borrow_mut().get_current_frame().unwrap();
        let winit::dpi::PhysicalSize { width, height } = self.window.inner_size();
        self.game.render(
            &self.gfx_state.borrow(),
            &swap_chain_output.output.view,  //color_attachment_view: &wgpu::TextureView,
            width,
            height,
            &self.console.borrow(),
            &self.menu.borrow(),
        );
    }
}

impl Program for ClientProgram {
    fn handle_event<T>(
        &mut self,
        event: Event<T>,
        _target: &EventLoopWindowTarget<T>,
        _control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                self.window_dimensions_changed = true;
            }

            e => self.input.borrow_mut().handle_event(e).unwrap(),
        }
    }

    fn frame(&mut self, frame_duration: Duration) {
        // recreate swapchain if needed
        if self.window_dimensions_changed {
            self.window_dimensions_changed = false;
            self.recreate_swap_chain(wgpu::PresentMode::Immediate);
        }

        let size: Extent2d = self.window.inner_size().into();

        // TODO: warn user if r_msaa_samples is invalid
        let mut sample_count = self
            .cvars
            .borrow()
            .get_value("r_msaa_samples")
            .unwrap_or(2.0) as u32;
        if !&[2, 4].contains(&sample_count) {
            sample_count = 2;
        }

        // recreate attachments and rebuild pipelines if necessary
        self.gfx_state.borrow_mut().update(size, sample_count);
        self.game.frame(&self.gfx_state.borrow(), frame_duration);

        match self.input.borrow().focus() {
            InputFocus::Game => {
                if let Err(e) = self.window.set_cursor_grab(CursorGrabMode::Locked) {
                    // This can happen if the window is running in another
                    // workspace. It shouldn't be considered an error.
                    log::debug!("Couldn't grab cursor: {}", e);
                }

                self.window.set_cursor_visible(false);
            }

            _ => {
                if let Err(e) = self.window.set_cursor_grab(CursorGrabMode::None) {
                    log::debug!("Couldn't release cursor: {}", e);
                };
                self.window.set_cursor_visible(true);
            }
        }

        // run console commands
        self.console.borrow().execute();

        self.render();
    }

    fn shutdown(&mut self) {
        // TODO: do cleanup things here
    }

    fn cvars(&self) -> Ref<CvarRegistry> {
        self.cvars.borrow()
    }

    fn cvars_mut(&self) -> RefMut<CvarRegistry> {
        self.cvars.borrow_mut()
    }
}










#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    trace: bool,

    #[structopt(long)]
    connect: Option<SocketAddr>,

    #[structopt(long)]
    dump_demo: Option<String>,

    #[structopt(long)]
    demo: Option<String>,

    #[structopt(long)]
    base_dir: Option<PathBuf>,
}



//Client  
//Execution begins here 
fn main() {

    let opt = Opt::from_args(); //input args 


    let event_loop = EventLoop::new();

    let window = {
        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowBuilderExtWindows as _;
            winit::window::WindowBuilder::new()
                // disable file drag-and-drop so cpal and winit play nice
                .with_drag_and_drop(false)
                .with_title("Soulgate")
                .with_inner_size(winit::dpi::PhysicalSize::<u32>::from((1366u32, 768)))
                .build(&event_loop)
                .unwrap()
        }

        #[cfg(not(target_os = "windows"))]
        {
            winit::window::WindowBuilder::new()
                .with_title("Soulgate")
                .with_inner_size(winit::dpi::PhysicalSize::<u32>::from((1366u32, 768)))
                .build(&event_loop)
                .unwrap()
        }
    };

    let client_program =
    futures::executor::block_on(ClientProgram::new(window, opt.base_dir, opt.trace));

/*
    //load the pak file 
    let filename:&str = "id1/pak0.pak";
   // let pakRc = Rc::new(pak::PackFile::new(filename).expect("Unable to load pak0"));

    let pak =pak::PackFile::new(filename).expect("Unable to load pak0");

    let bsp_file = bsp::BspFile::parse(
        &mut Cursor::new(pak.file("maps/start.bsp").unwrap())
    ).unwrap();

    println!("Loaded pak");*/

    /*
    let mut renderer = render::Renderer::new(
        pak.clone(), bsp_file,
        adapter, surface,
        size,
    ).unwrap();
    */



    //run is framework  453 

    //start is framework 26 0

 

 //   framework::run::<LevelRenderspace>("Soulgate");
  
}



