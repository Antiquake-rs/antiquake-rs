use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{view::BobVars, Client, render::RenderSceneConstants, };
use crate::{
    client::{
        unit::{
            particle::{Particle, Particles, TrailKind, MAX_PARTICLES},
            Beam, ClientUnit, Light, LightDesc, Lights, MAX_BEAMS, MAX_LIGHTS, MAX_TEMP_ENTITIES,
        },
        input::game::{Action, GameInput},
        render::{Camera, WorldspawnRenderData, },
        sound::{AudioSource, EntityMixer, Listener, StaticSound},
        view::{IdleVars, KickVars, MouseVars, RollVars, View},
        ClientError, ColorShiftCode, IntermissionKind, MoveVars, MAX_STATS,
    },
    common::{
        bsp, engine,
        math::{self, Angles},
        model::{Model, ModelFlags, ModelKind, SyncType},
        net::{
            self, BeamEntityKind, ButtonFlags, ColorShift, UnitEffects, ItemFlags, PlayerData,
            PointEntityKind, TempEntity,
        },
        vfs::Vfs, tickcounter::TickCounter, console::CvarRegistry,
         gamestate::{GameStateDeltaBuffer, DeltaCommand, GameStateDelta, 
         system as ecs_systems,
         component::{self as ecs_components, physics::PhysicsComponent},
         entity::{BevyEntityLookupRegistry}
        },
    },
};
use arrayvec::ArrayVec;
use cgmath::{Angle as _, Deg, InnerSpace as _, Matrix4, Vector3, Zero as _};
use chrono::Duration;
use net::{ClientCmd, ClientStat, UnitState, EntityUpdate, PlayerColor};
use rand::{
    distributions::{Distribution as _, Uniform},
    rngs::SmallRng,
    SeedableRng,
};
use rodio::OutputStreamHandle;

use bevy_ecs::{world::{World as BevyWorld, Mut}, schedule::{Schedule, SystemStage}, prelude::{Component, Entity, EventWriter}, system::{Resource, SystemState, Query}, query::{QueryIter, WorldQuery}};
 


use std::time::{SystemTime, UNIX_EPOCH};

/* 

what was this?  a hack for the demo ? 
const CACHED_SOUND_NAMES: &[&'static str] = &[
    "hknight/hit.wav",
    "weapons/r_exp3.wav",
    "weapons/ric1.wav",
    "weapons/ric2.wav",
    "weapons/ric3.wav",
    "weapons/tink1.wav",
    "wizard/hit.wav",
];
*/

pub struct PlayerInfo {
    pub name: String,
    pub frags: i32,
    pub colors: PlayerColor,
    // translations: [u8; VID_GRADES],
}


// this is an ecs resource 
pub struct LoadedAssetsCache {
    pub models: Vec<Model>, 
    pub model_names: HashMap<String, usize>,    
    pub sounds: Vec<AudioSource>,
    pub sound_names: HashMap<String,usize>
  //  pub cached_sounds: HashMap<String, AudioSource>,
}

impl LoadedAssetsCache {
    pub fn new() -> LoadedAssetsCache {
        LoadedAssetsCache {
            models: vec![Model::none()],
            model_names: HashMap::new(),
            sounds: Vec::new(),
            sound_names: HashMap::new(),

         }
    }

    pub fn get_sound(&self,name:&str) -> Option<&AudioSource> {

        let sound_id = self.sound_names.get(&name.to_string())?;

        return Some(&self.sounds[*sound_id]);
    }
}


// client information regarding the current level

//move more and more of this into ECS resources
pub struct ClientState  {
    // local rng
    rng: SmallRng,

    // model precache
  //  pub models: Vec<Model>,   //this should be in a resource !! in ECS ! 
    // name-to-id map
   // pub model_names: HashMap<String, usize>,   //this should be in a resource !! in ECS ! 

    // audio source precache
  //  pub sounds: Vec<AudioSource>, //this should be in a resource !! in ECS ! 

    // sounds that are always needed even if not in precache
   // cached_sounds: HashMap<String, AudioSource>, //this should be in a resource !! in ECS ! 

    // ambient sounds (infinite looping, static position)
    pub static_sounds: Vec<StaticSound>,

    // entities and entity-like things -- REMOVE THESE
   // pub entities: Vec<ClientUnit>,
   // pub entities: HashMap<usize, i32>, //quake entity_id to bevy_id  

   //move these to ECS 
    pub static_entities: Vec<ClientUnit>,
    pub temp_entities: Vec<ClientUnit>,

        

    // dynamic point lights
    pub lights: Lights,
    // lightning bolts and grappling hook cable
    pub beams: [Option<Beam>; MAX_BEAMS],

    // particle effects  --should be part of the ECS but i dont think every particle should be an entity ?
    pub particles: Particles,

    // visible entities, rebuilt per-frame
    pub visible_entity_ids: Vec<usize>,

    //pub light_styles: HashMap<u8, String>,

    // various values relevant to the player and level (see common::net::ClientStat)
    pub stats: [i32; MAX_STATS],

    pub max_players: usize,
    pub player_info: [Option<PlayerInfo>; net::MAX_CLIENTS],

    // the last two timestamps sent by the server (for lerping)
    pub msg_times: [Duration; 2],
    pub time: Duration,
    pub lerp_factor: f32,

    

    pub items: ItemFlags,
    pub item_get_time: [Duration; net::MAX_ITEMS],
    pub face_anim_time: Duration,
    pub color_shifts: [Rc<RefCell<ColorShift>>; 4],
    pub view: View,



    // These should def not exist in here !! 
    pub msg_velocity: [Vector3<f32>; 2],
    pub velocity: Vector3<f32>,

    // paused: bool,
    pub on_ground: bool,
    pub in_water: bool,
    // --------


    pub loaded_assets_cache: LoadedAssetsCache,

    pub intermission: Option<IntermissionKind>,
    pub start_time: Duration,
    pub completion_time: Option<Duration>,

    pub mixer: EntityMixer,
    pub listener: Listener,

    pub tick_counter: TickCounter,

    pub ecs_world: BevyWorld,
    pub ecs_tick_schedule: Schedule, //runs every tick  -- physics 
    pub ecs_frame_schedule: Schedule,


    pub worldspawn_render_data: Option<WorldspawnRenderData>,  
 
  //  pub client_gamestate_delta_buffer: GameStateDeltaBuffer,
}

impl ClientState {
    // TODO: add parameter for number of player slots and reserve them in entity list
    pub fn new(stream: OutputStreamHandle) -> ClientState {
        

        //this is disgusting !!! break it into resources, components , systems 
        let mut c_state = ClientState {
            rng: SmallRng::from_entropy(),

            loaded_assets_cache: LoadedAssetsCache::new(),

            worldspawn_render_data: None,
            
         /*   models: vec![Model::none()],
            model_names: HashMap::new(),
            sounds: Vec::new(),
            cached_sounds: HashMap::new(),*/ 

            //put these into ECS 
            static_sounds: Vec::new(),

            //entities: HashMap::new(),
            static_entities: Vec::new(),
            temp_entities: Vec::new(),

            lights: Lights::with_capacity(MAX_LIGHTS),
            beams: [None; MAX_BEAMS],
            particles: Particles::with_capacity(MAX_PARTICLES),
            visible_entity_ids: Vec::new(),
        //    light_styles: HashMap::new(),
            stats: [0; MAX_STATS],
            max_players: 0,
            player_info: Default::default(),
            msg_times: [Duration::zero(), Duration::zero()],
            time: Duration::zero(),

        


            lerp_factor: 0.0,
            items: ItemFlags::empty(),
            item_get_time: [Duration::zero(); net::MAX_ITEMS],
            color_shifts: [
                Rc::new(RefCell::new(ColorShift {
                    dest_color: [0; 3],
                    percent: 0,
                })),
                Rc::new(RefCell::new(ColorShift {
                    dest_color: [0; 3],
                    percent: 0,
                })),
                Rc::new(RefCell::new(ColorShift {
                    dest_color: [0; 3],
                    percent: 0,
                })),
                Rc::new(RefCell::new(ColorShift {
                    dest_color: [0; 3],
                    percent: 0,
                })),
            ],
            view: View::new(),
            face_anim_time: Duration::zero(),
            msg_velocity: [Vector3::zero(), Vector3::zero()],
            velocity: Vector3::zero(),
            on_ground: false,
            in_water: false,
            intermission: None,
            start_time: Duration::zero(),
            completion_time: None,
            mixer: EntityMixer::new(stream),
            listener: Listener::new(),

            tick_counter: TickCounter::new( Duration::milliseconds( 33 )  ),
            ecs_world: BevyWorld::new(),
            ecs_tick_schedule: Schedule::default(),
            ecs_frame_schedule: Schedule::default(),
             
            
           // client_gamestate_delta_buffer: GameStateDeltaBuffer::new()

        };





        c_state.init_ecs();



        return c_state;
    }

    pub fn from_server_info(
        vfs: &Vfs,
        stream: OutputStreamHandle,
        max_clients: u8,
        model_precache: Vec<String>,
        sound_precache: Vec<String>,
    ) -> Result<ClientState, ClientError> {


        // Add these models and sounds to the assets precache resource !!! 

        // TODO: validate submodel names
        let mut models = Vec::with_capacity(model_precache.len());
        models.push(Model::none()); // this is bc the map is slot 0 ? 
        let mut model_names = HashMap::new();
        for mod_name in model_precache {
            // BSPs can have more than one model
            if mod_name.ends_with(".bsp") {
                let bsp_data = vfs.open(&mod_name)?;
                let (mut brush_models, _) = bsp::load(bsp_data).unwrap(); 
                for bmodel in brush_models.drain(..) {
                    let id = models.len();
                    let name = bmodel.name().to_owned();
                    models.push(bmodel);
                    model_names.insert(name, id);
                }
            } else if !mod_name.starts_with("*") {
                // model names starting with * are loaded from the world BSP
                debug!("Loading model {}", mod_name);
                let id = models.len();
                models.push(Model::load(vfs, &mod_name)?);
                model_names.insert(mod_name, id);
            }

            // TODO: send keepalive message?
        }

     
        let mut sounds= Vec::with_capacity(sound_precache.len());
        let mut sound_names = HashMap::new();
        for ref snd_name in sound_precache {            
            let id=sounds.len();
            sounds.push( AudioSource::load(vfs, snd_name)?);
            sound_names.insert(snd_name.to_string(), id);
        }

 

        Ok(ClientState {
           
            loaded_assets_cache:LoadedAssetsCache {
                models,
                model_names,
                sounds,
                sound_names,
             },
            
            max_players: max_clients as usize,  //put this in an ecs resource -- like server info 
            ..ClientState::new(stream)
        })
    }

    
   
    pub fn get_world_mut(&mut self) -> & mut BevyWorld {
        return &mut self.ecs_world; 
    }


    /*
    
    WorldRenderer::new(
                        gfx_state,
                        self.state.models(),
                        1,*/


    
    fn init_ecs(&mut self){
        let primary_stage:&str = "primary";
        let render_stage:&str = "render";

        //add plugins , add resources 



        self.ecs_tick_schedule.add_stage(primary_stage, SystemStage::parallel() );
        self.ecs_tick_schedule.add_system_to_stage(primary_stage, ecs_systems::physics::update_physics_movement);
        
        //self.ecs_frame_schedule.add_stage(primary_stage, SystemStage::parallel() );
        //self.ecs_frame_schedule.add_system_to_stage(primary_stage, ecs_systems::render::update_physics_movement);

        //doing render in ecs would suck.. how would we do the menu then ?
      //  self.ecs_frame_schedule.add_stage(render_stage, SystemStage::single_threaded() );
     //   self.ecs_frame_schedule.add_system_to_stage(primary_stage, ecs_systems::render::render_pass);
        

        self.ecs_world.insert_resource(GameStateDeltaBuffer::new());
        self.ecs_world.insert_resource(RenderSceneConstants::new());
      

        self.ecs_world.insert_resource(BevyEntityLookupRegistry::new());

       

        //https://docs.rs/bevy/0.8.0/bevy/ecs/system/struct.SystemState.html
        
        

    }



    //could be improved - ? 
    pub fn on_loaded_models(&mut self) {
        self.build_bsp_collision_hulls(  );

    }


    pub fn advance_time(&mut self, frame_time:Duration, cvars: &CvarRegistry, is_connected:bool) -> Result<Duration,ClientError>{
        self.time = self.time + frame_time;


        let (accum,trigger) = self.tick_counter.update(frame_time);

        if trigger {
            self.on_tick();            
        }

        let pause_rendering = accum > Duration::milliseconds(1000);


        let cl_nolerp = Client::cvar_value(cvars,"cl_nolerp")?;
        let sv_gravity = Client::cvar_value(cvars,"sv_gravity")?;
          


            //MOVE ALL THESE INTO ECS 

        self.update_interp_ratio(cl_nolerp);

        //this need to happen in the special ticks since we are running a predictive sim now -- not getting absolute positions from server only DELTAS !
        // interpolate entity data and spawn particle effects, lights
       // self.update_entities()?;

        // update temp entities (lightning, etc.)
     //   self.update_temp_entities()?;    //RE IMPLEMENT ME 

        // remove expired lights
        self.lights.update(self.time);

        // apply particle physics and remove expired particles
        self.particles  .update(self.time, frame_time, sv_gravity);



        if is_connected {

            let idle_vars = Client::idle_vars(cvars)?;
            let kick_vars = Client::kick_vars(cvars)?;
            let roll_vars = Client::roll_vars(cvars)?;
            let bob_vars = Client::bob_vars(cvars)?;

            self.calc_final_view(idle_vars, kick_vars, roll_vars, bob_vars);

            // update ear positions
            self.update_listener();

            // spatialize sounds for new ear positions
            self.update_sound_spatialization();

            // update camera color shifts for new position/effects
            self.update_color_shifts(frame_time)?;


        }

        if !pause_rendering {
            let world = &mut self.ecs_world;
            self.ecs_frame_schedule.run_once(world);
        }
       

        return Ok(accum)
    }


    //ecs help https://bevy-cheatbook.github.io/programming/queries.html





    //runs each 'tick_period' milliseconds to advance the virtual machine (to match the server)
    pub fn on_tick(&mut self){
        //println!("client run tick");




        //flush gamestate delta buffer 

       //self.flush_gamestate_delta_buffer();



        //need to do this with gamestate deltas which are built by the client from input keys 
        //also need to do this inside of physics component or something...
       
       
      
        // Add a Stage to our schedule. Each Stage in a schedule runs all of its systems
        // before moving on to the next Stage
       /*  self.ecs_schedule.add_stage("update", SystemStage::parallel()
            .with_system(ecs_systems::physics::update_physics_movement)
        );  */

        
    
        // Run the schedule once. If your app has a "loop", you would run this once per loop
        let world = &mut self.ecs_world;
        self.ecs_tick_schedule.run_once(world);

    }



 

    pub fn get_resource<T>(&self) -> &T
    where 
    T: Resource
    {
        let resource = self.ecs_world.get_resource::<T >().unwrap();

        return resource;
    }

    pub fn get_resource_mut<T>(&mut self) -> Mut<T>
    where  T: Resource
    {
       

        let resource = self.ecs_world.get_resource_mut::<T>().unwrap();

        return resource
       
    }
    
    pub fn get_entity(&self, unit_id:usize) -> Option< &Entity>
  
     {

        let lookup_registry = self.get_resource::<BevyEntityLookupRegistry>();

        let ent = lookup_registry.get( unit_id ) ;
 
        return ent

    }

    pub fn unit_exists(&self, unit_id:usize) -> bool {

        match self.get_entity(unit_id) {
            Some(_) => return true,
            None => return false
        }
    }

    pub fn get_component_of_entity<T>(&self, unit_id:usize) -> Option<&T>
    where T: bevy_ecs::component::Component
    {

        let ent = self.get_entity( unit_id )?;

        let component = self.ecs_world.get::<T>(*ent)?;

    

        return Some(component)

    }
    pub fn get_mut_component_of_entity<T>(&mut self, unit_id:usize) -> Option<Mut<T>>
    where T: bevy_ecs::component::Component
    {
    

        let ent = self.get_entity( unit_id )?;

        let mut component = self.ecs_world.get_mut::<T>(*ent) ;

        return component
      
 
    }



    pub fn push_to_gamestate_deltas( &mut self , delta_cmd:DeltaCommand ){
        
        //really should not push a move or angle if there already are some there !

            let new_delta = GameStateDelta::new(
                delta_cmd, 
                self.view_unit_id()  ,
                self.player_id() ,
                self.tick_count()  ,

            ) ;

            let mut delta_buffer = self.get_resource_mut::<GameStateDeltaBuffer>();

           
            delta_buffer.push( new_delta );
                
            
           
    }



    /// Update the client state interpolation ratio.
    ///
    /// This calculates the ratio used to interpolate entities between the last
    /// two updates from the server.
    pub fn update_interp_ratio(&mut self, cl_nolerp: f32) {
        if cl_nolerp != 0.0 {
            self.time = self.msg_times[0];
            self.lerp_factor = 1.0;
            return;
        }

        let server_delta = engine::duration_to_f32(match self.msg_times[0] - self.msg_times[1] {
            // if no time has passed between updates, don't lerp anything
            d if d == Duration::zero() => {
                self.time = self.msg_times[0];
                self.lerp_factor = 1.0;
                return;
            }

            d if d > Duration::milliseconds(100) => {
                self.msg_times[1] = self.msg_times[0] - Duration::milliseconds(100);
                Duration::milliseconds(100)
            }

            d if d < Duration::zero() => {
                warn!(
                    "Negative time delta from server!: ({})s",
                    engine::duration_to_f32(d)
                );
                d
            }

            d => d,
        });

        let frame_delta = engine::duration_to_f32(self.time - self.msg_times[1]);

        self.lerp_factor = match frame_delta / server_delta {
            f if f < 0.0 => {
                if f < -0.01 {
                    self.time = self.msg_times[1];
                }

                0.0
            }

            f if f > 1.0 => {
                if f > 1.01 {
                    self.time = self.msg_times[0];
                }

                1.0
            }

            f => f,
        }
    }


 






    fn build_bsp_collision_hulls(&mut self){

       // let models = self.models(); 

       self.worldspawn_render_data = Some( WorldspawnRenderData::new(self.models(), 1) );

       

      /* match worldspawn_bsp_data  {

            Some( worldspawn ) => { 


            },


            None => panic!("No worldspawn render data to generate collision hulls")

        } */ 

        //for each model, if its worldspawn add an entity to ECS that has a physics collision hull -- use that for gamestate deltas 


    }


    /// Update all entities in the game world.
    ///
    /// This method is responsible for the following:
    /// - Updating entity position
    /// - Despawning entities which did not receive an update in the last server
    ///   message
    /// - Spawning particles on entities with particle effects
    /// - Spawning dynamic lights on entities with lighting effects
   /* pub fn update_entities(&mut self) -> Result<(), ClientError> {
        lazy_static! {
            static ref MFLASH_DIMLIGHT_DISTRIBUTION: Uniform<f32> = Uniform::new(200.0, 232.0);
            static ref BRIGHTLIGHT_DISTRIBUTION: Uniform<f32> = Uniform::new(400.0, 432.0);
        }

        let lerp_factor = self.lerp_factor;

        self.velocity =
            self.msg_velocity[1] + lerp_factor * (self.msg_velocity[0] - self.msg_velocity[1]);

        // TODO: if we're in demo playback, interpolate the view angles

        let obj_rotate = Deg(100.0 * engine::duration_to_f32(self.time)).normalize();

        // rebuild the list of visible entities
        self.visible_entity_ids.clear();

        // in the extremely unlikely event that there's only a world entity and nothing else, just
        // return
        if self.entities.len() <= 1 {
            return Ok(());
        }

        // NOTE that we start at entity 1 since we don't need to link the world entity
        for (ent_id, ent) in self.entities.iter_mut().enumerate().skip(1) {
            if ent.model_id == 0 {
                // nothing in this entity slot
                continue;
            }

            // if we didn't get an update this frame, remove the entity
            if ent.msg_time != self.msg_times[0] {
                ent.model_id = 0;
                continue;
            }

            let prev_origin = ent.origin;

            if ent.force_link {
                trace!("force link on entity {}", ent_id);
                ent.origin = ent.msg_origins[0];
                ent.angles = ent.msg_angles[0];
            } else {
                let origin_delta = ent.msg_origins[0] - ent.msg_origins[1];
                let ent_lerp_factor = if origin_delta.magnitude2() > 10_000.0 {
                    // if the entity moved more than 100 units in one frame,
                    // assume it was teleported and don't lerp anything
                    1.0
                } else {
                    lerp_factor
                };

                ent.origin = ent.msg_origins[1] + ent_lerp_factor * origin_delta;

                // assume that entities will not whip around 180+ degrees in one
                // frame and adjust the delta accordingly. this avoids a bug
                // where small turns between 0 <-> 359 cause the demo camera to
                // face backwards for one frame.
                for i in 0..3 {
                    let mut angle_delta = ent.msg_angles[0][i] - ent.msg_angles[1][i];
                    if angle_delta > Deg(180.0) {
                        angle_delta = Deg(360.0) - angle_delta;
                    } else if angle_delta < Deg(-180.0) {
                        angle_delta = Deg(360.0) + angle_delta;
                    }

                    ent.angles[i] =
                        (ent.msg_angles[1][i] + angle_delta * ent_lerp_factor).normalize();
                }
            }

            let model = &self.models[ent.model_id];
            if model.has_flag(ModelFlags::ROTATE) {
                ent.angles[1] = obj_rotate;
            }

            if ent.effects.contains(UnitEffects::BRIGHT_FIELD) {
                self.particles.create_entity_field(self.time, ent);
            }

            // TODO: factor out EntityEffects->LightDesc mapping
            if ent.effects.contains(UnitEffects::MUZZLE_FLASH) {
                // TODO: angle and move origin to muzzle
                ent.light_id = Some(self.lights.insert(
                    self.time,
                    LightDesc {
                        origin: ent.origin + Vector3::new(0.0, 0.0, 16.0),
                        init_radius: MFLASH_DIMLIGHT_DISTRIBUTION.sample(&mut self.rng),
                        decay_rate: 0.0,
                        min_radius: Some(32.0),
                        ttl: Duration::milliseconds(100),
                    },
                    ent.light_id,
                ));
            }

            if ent.effects.contains(UnitEffects::BRIGHT_LIGHT) {
                ent.light_id = Some(self.lights.insert(
                    self.time,
                    LightDesc {
                        origin: ent.origin,
                        init_radius: BRIGHTLIGHT_DISTRIBUTION.sample(&mut self.rng),
                        decay_rate: 0.0,
                        min_radius: None,
                        ttl: Duration::milliseconds(1),
                    },
                    ent.light_id,
                ));
            }

            if ent.effects.contains(UnitEffects::DIM_LIGHT) {
                ent.light_id = Some(self.lights.insert(
                    self.time,
                    LightDesc {
                        origin: ent.origin,
                        init_radius: MFLASH_DIMLIGHT_DISTRIBUTION.sample(&mut self.rng),
                        decay_rate: 0.0,
                        min_radius: None,
                        ttl: Duration::milliseconds(1),
                    },
                    ent.light_id,
                ));
            }

            // check if this entity leaves a trail
            let trail_kind = if model.has_flag(ModelFlags::GIB) {
                Some(TrailKind::Blood)
            } else if model.has_flag(ModelFlags::ZOMGIB) {
                Some(TrailKind::BloodSlight)
            } else if model.has_flag(ModelFlags::TRACER) {
                Some(TrailKind::TracerGreen)
            } else if model.has_flag(ModelFlags::TRACER2) {
                Some(TrailKind::TracerRed)
            } else if model.has_flag(ModelFlags::ROCKET) {
                ent.light_id = Some(self.lights.insert(
                    self.time,
                    LightDesc {
                        origin: ent.origin,
                        init_radius: 200.0,
                        decay_rate: 0.0,
                        min_radius: None,
                        ttl: Duration::milliseconds(10),
                    },
                    ent.light_id,
                ));
                Some(TrailKind::Rocket)
            } else if model.has_flag(ModelFlags::GRENADE) {
                Some(TrailKind::Smoke)
            } else if model.has_flag(ModelFlags::TRACER3) {
                Some(TrailKind::Vore)
            } else {
                None
            };

            // if the entity leaves a trail, generate it
            if let Some(kind) = trail_kind {
                self.particles
                    .create_trail(self.time, prev_origin, ent.origin, kind, false);
            }

            // don't render the player model
            if self.view.unit_id() != ent_id {
                // mark entity for rendering
                self.visible_entity_ids.push(ent_id);
            }

            // enable lerp for next frame
            ent.force_link = false;
        }

        // apply effects to static entities as well
        for ent in self.static_entities.iter_mut() {
            if ent.effects.contains(UnitEffects::BRIGHT_LIGHT) {
                debug!("spawn bright light on static entity");
                ent.light_id = Some(self.lights.insert(
                    self.time,
                    LightDesc {
                        origin: ent.origin,
                        init_radius: BRIGHTLIGHT_DISTRIBUTION.sample(&mut self.rng),
                        decay_rate: 0.0,
                        min_radius: None,
                        ttl: Duration::milliseconds(1),
                    },
                    ent.light_id,
                ));
            }

            if ent.effects.contains(UnitEffects::DIM_LIGHT) {
                debug!("spawn dim light on static entity");
                ent.light_id = Some(self.lights.insert(
                    self.time,
                    LightDesc {
                        origin: ent.origin,
                        init_radius: MFLASH_DIMLIGHT_DISTRIBUTION.sample(&mut self.rng),
                        decay_rate: 0.0,
                        min_radius: None,
                        ttl: Duration::milliseconds(1),
                    },
                    ent.light_id,
                ));
            }
        }

        Ok(())
    }*/ 

  /*   pub fn update_temp_entities(&mut self) -> Result<(), ClientError> {
        lazy_static! {
            static ref ANGLE_DISTRIBUTION: Uniform<f32> = Uniform::new(0.0, 360.0);
        }

        self.temp_entities.clear();
        for id in 0..self.beams.len() {
            // remove beam if expired
            if self.beams[id].map_or(false, |b| b.expire < self.time) {
                self.beams[id] = None;
                continue;
            }

            let view_ent = self.view_unit_id();
            if let Some(ref mut beam) = self.beams[id] {
                // keep lightning gun bolts fixed to player
                if beam.unit_id == view_ent {
                    beam.start = self.entities[view_ent].origin;
                }

                let vec = beam.end - beam.start;
                let yaw = Deg::from(cgmath::Rad(vec.y.atan2(vec.x))).normalize();
                let forward = (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt();
                let pitch = Deg::from(cgmath::Rad(vec.z.atan2(forward))).normalize();

                let len = vec.magnitude();
                let direction = vec.normalize();
                for interval in 0..(len / 30.0) as i32 {
                    let mut ent = ClientUnit::uninitialized();
                    ent.origin = beam.start + 30.0 * interval as f32 * direction;
                    ent.angles =
                        Vector3::new(pitch, yaw, Deg(ANGLE_DISTRIBUTION.sample(&mut self.rng)));

                    if self.temp_entities.len() < MAX_TEMP_ENTITIES {
                        self.temp_entities.push(ent);
                    } else {
                        warn!("too many temp entities!");
                    }
                }
            }
        }

        Ok(())
    } */

    //this should not happen like this --- do more like ECS 
    pub fn update_player(&mut self, update: PlayerData) {
        self.view
            .set_view_height(update.view_height.unwrap_or(net::DEFAULT_VIEWHEIGHT));
        self.view
            .set_ideal_pitch(update.ideal_pitch.unwrap_or(Deg(0.0)));
        self.view.set_punch_angles(Angles {
            pitch: update.punch_pitch.unwrap_or(Deg(0.0)),
            roll: update.punch_roll.unwrap_or(Deg(0.0)),
            yaw: update.punch_yaw.unwrap_or(Deg(0.0)),
        });

        // store old velocity
        self.msg_velocity[1] = self.msg_velocity[0];
        self.msg_velocity[0].x = update.velocity_x.unwrap_or(0.0);
        self.msg_velocity[0].y = update.velocity_y.unwrap_or(0.0);
        self.msg_velocity[0].z = update.velocity_z.unwrap_or(0.0);

        let item_diff = update.items - self.items;
        if !item_diff.is_empty() {
            // item flags have changed, something got picked up
            let bits = item_diff.bits();
            for i in 0..net::MAX_ITEMS {
                if bits & 1 << i != 0 {
                    // item with flag value `i` was picked up
                    self.item_get_time[i] = self.time;
                }
            }
        }
        self.items = update.items;

        self.on_ground = update.on_ground;
        self.in_water = update.in_water;

        self.stats[ClientStat::WeaponFrame as usize] = update.weapon_frame.unwrap_or(0) as i32;
        self.stats[ClientStat::Armor as usize] = update.armor.unwrap_or(0) as i32;
        self.stats[ClientStat::Weapon as usize] = update.weapon.unwrap_or(0) as i32;
        self.stats[ClientStat::Health as usize] = update.health as i32;
        self.stats[ClientStat::Ammo as usize] = update.ammo as i32;
        self.stats[ClientStat::Shells as usize] = update.ammo_shells as i32;
        self.stats[ClientStat::Nails as usize] = update.ammo_nails as i32;
        self.stats[ClientStat::Rockets as usize] = update.ammo_rockets as i32;
        self.stats[ClientStat::Cells as usize] = update.ammo_cells as i32;

        // TODO: this behavior assumes the `standard_quake` behavior and will likely
        // break with the mission packs
        self.stats[ClientStat::ActiveWeapon as usize] = update.active_weapon as i32;
    }



    //affects the view (camera)
    pub fn handle_input(
        &mut self,
        game_input: &mut GameInput,
        frame_time: Duration,
        move_vars: MoveVars,
        mouse_vars: MouseVars,
    )  {
       

        //let mlook = game_input.action_state(MLook);

        let mlook = true; //use mouselook by default .  how can i just set the default ?

        self.view.handle_input(
            frame_time,
            game_input,
            self.intermission.as_ref(),
            mlook,
            move_vars.cl_anglespeedkey,
            move_vars.cl_pitchspeed,
            move_vars.cl_yawspeed,
            mouse_vars,
        );

       
    }


 
    pub fn build_move_cmd(
        game_input: &mut GameInput,
        frame_time: Duration,
        move_vars: MoveVars,
        mouse_vars: MouseVars,
        angles: Angles,
    ) -> ClientCmd {
        use Action::*;

        //let mlook = game_input.action_state(MLook);

        let mlook = true; //use mouselook by default .  how can i just set the default ?
 


        let mut move_left = game_input.action_state(MoveLeft);
        let mut move_right = game_input.action_state(MoveRight);
        if game_input.action_state(Strafe) {
            move_left |= game_input.action_state(Left);
            move_right |= game_input.action_state(Right);
        }

        let mut sidemove = move_vars.cl_sidespeed * (move_right as i32 - move_left as i32) as f32;
        if(sidemove.is_nan()) {sidemove = 0.0;}

        let mut upmove = move_vars.cl_upspeed
            * (game_input.action_state(MoveUp) as i32 - game_input.action_state(MoveDown) as i32)
                as f32;
        if(upmove.is_nan()) {upmove = 0.0;}

        let mut forwardmove = 0.0;
        if !game_input.action_state(KLook) {
            forwardmove +=
                move_vars.cl_forwardspeed * game_input.action_state(Forward) as i32 as f32;
            forwardmove -= move_vars.cl_backspeed * game_input.action_state(Back) as i32 as f32;
        }
        if(forwardmove.is_nan()) {forwardmove = 0.0;}

        if game_input.action_state(Speed) {
            sidemove *= move_vars.cl_movespeedkey;
            upmove *= move_vars.cl_movespeedkey;
            forwardmove *= move_vars.cl_movespeedkey;
        }

        let mut button_flags = ButtonFlags::empty();

        if game_input.action_state(Attack) {
            button_flags |= ButtonFlags::ATTACK;
        }

        if game_input.action_state(Jump) {
            button_flags |= ButtonFlags::JUMP;
        }

        if !mlook {
            // TODO: IN_Move (mouse / joystick / gamepad)
        }
        

        // this is used to generate delta commands ! 
        ClientCmd::Move {
            send_time:Duration::milliseconds(20),// stub for now 
            angles: Vector3::new(angles.pitch, angles.yaw, angles.roll),
            fwd_move: forwardmove as i16 ,
            side_move: sidemove as i16 ,
            up_move: upmove as i16 ,
            button_flags,
            impulse: game_input.impulse(),
        }
    }

    pub fn handle_damage(
        &mut self,
        armor: u8,
        health: u8,
        source: Vector3<f32>,
        kick_vars: KickVars,
    ) { 
        
            //write something into a gamestate buffer ???  
            // THen that will affect dmg component ????


            //or maybe server is just authoritative about damage. like this.

         
        /*
        self.face_anim_time = self.time + Duration::milliseconds(200);

        let dmg_factor = (armor + health).min(20) as f32 / 2.0;
        let mut cshift = self.color_shifts[ColorShiftCode::Damage as usize].borrow_mut();
        cshift.percent += 3 * dmg_factor as i32;
        cshift.percent = cshift.percent.clamp(0, 150);

        if armor > health {
            cshift.dest_color = [200, 100, 100];
        } else if armor > 0 {
            cshift.dest_color = [220, 50, 50];
        } else {
            cshift.dest_color = [255, 0, 0];
        }

        let v_ent = &self.entities[self.view.unit_id()];

        let v_angles = Angles {
            pitch: v_ent.angles.x,
            roll: v_ent.angles.z,
            yaw: v_ent.angles.y,
        };

        self.view.handle_damage(
            self.time,
            armor as f32,
            health as f32,
            v_ent.origin,
            v_angles,
            source,
            kick_vars,
        );
        */
    }

    pub fn calc_final_view(
        &mut self,
        idle_vars: IdleVars,
        kick_vars: KickVars,
        roll_vars: RollVars,
        bob_vars: BobVars,
    ) {
        
        let controlled_entity_phys_comp = self.get_component_of_entity::
        <ecs_components::physics::PhysicsComponent>
        (self.view.unit_id()  );


        match controlled_entity_phys_comp {
            Some(phys_comp) => {
                let entity_origin = phys_comp.origin.clone();

                self.view.calc_final_angles(
                    self.time,
                    self.intermission.as_ref(),
                    self.velocity,
                    idle_vars,
                    kick_vars,
                    roll_vars,
                );
                self.view.calc_final_origin(
                    self.time,
                    entity_origin,
                    self.velocity,
                    bob_vars,
                );

            }
            _ => {}
        }

       // let entity_origin = self.entities[self.view.unit_id()].origin;

      //  println!("calc_final_view entity origin {} {} {}", entity_origin.x,entity_origin.y,entity_origin.z);
 
       

      //  println!("calc_final_view ->  {} {} {}", self.view.final_origin().x,self.view.final_origin().y,self.view.final_origin().z);
          
        

    }
    

    //https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs

    /// Spawn an entity with the given ID, also spawning any uninitialized
    /// entities between the former last entity and the new one.
    // TODO: skipping entities indicates that the entities have been freed by
    // the server. it may make more sense to use a HashMap to store entities by
    // ID since the lookup table is relatively sparse.
    pub fn spawn_entity(&mut self, id: usize, baseline: UnitState) -> Result<(), ClientError> {
       
       
       // let existing = self.entities.get( &id  );

        match self.get_entity(id) {

            Some(_) => {
                return Err(ClientError::EntityExists(id as usize)) 
            }
            None => {

                debug!(
                    "Spawning entity with id {} from baseline {:?}",
                    id, baseline
                );
                
               // self.entities.push(ClientUnit::from_baseline(baseline));
        

                let bevy_id = self.ecs_world.spawn()
                    .insert(ecs_components::physics::PhysicsComponent::from_baseline(&baseline))
                    .insert(ecs_components::rendermodel::RenderModelComponent::from_baseline(&baseline))
                    .id();
                  

                let mut lookup_registry = self.get_resource_mut::<BevyEntityLookupRegistry>();


                lookup_registry.insert( id, bevy_id   );


              
                //self.entities.insert( id, bevy_id  );

               
                 //all the data that was in client unit will be moved to components 
            }
        }
       
      /*  // don't clobber existing entities
        if id < self.entities.len() {
            Err(ClientError::EntityExists(id))?;
        }

        // spawn intermediate entities (uninitialized)
        for i in self.entities.len()..id {
            debug!("Spawning uninitialized entity with ID {}", i);
            self.entities.push(ClientUnit::uninitialized());
        } */

      
        Ok(())
    }



    //change this to just store a statedelta!! 

    pub fn on_fast_update( &mut self, fast_update:EntityUpdate ) -> Result<(), ClientError> {

        let unit_id = fast_update.ent_id as usize;
        //self.state.on_fast_update(unit_id, fast_update)?;
        //self.update_entity(unit_id,fast_update)?;



        // add this as a gamestate delta !! this is really a delta from the server (should be)! 


        
        Ok(())
    }



    //maybe get rid of this ? 
    pub fn patch_demo_view_angles(&mut self, unit_id:usize,demo_view_angles:Option<Vector3<Deg<f32>>> ) -> Result<(), ClientError> {
                 
        // patch view angles in demos
        if let Some(angles) = demo_view_angles {
            if unit_id == self.view_unit_id() {
                self.update_view_angles(angles);
            }
        }

        Ok(())
    }

    /*pub fn update_entity(&mut self, id: usize, update: EntityUpdate) -> Result<(), ClientError> {
        if id >= self.entities.len() {
            let baseline = UnitState {
                origin: Vector3::new(
                    update.origin_x.unwrap_or(0.0),
                    update.origin_y.unwrap_or(0.0),
                    update.origin_z.unwrap_or(0.0),
                ),
                angles: Vector3::new(
                    update.pitch.unwrap_or(Deg(0.0)),
                    update.yaw.unwrap_or(Deg(0.0)),
                    update.roll.unwrap_or(Deg(0.0)),
                ),
                model_id: update.model_id.unwrap_or(0) as usize,
                frame_id: update.frame_id.unwrap_or(0) as usize,
                colormap: update.colormap.unwrap_or(0),
                skin_id: update.skin_id.unwrap_or(0) as usize,
                effects: UnitEffects::empty(),
            };

            self.spawn_entities(id, baseline)?;
        }

        let entity = &mut self.entities[id];
        entity.update(self.msg_times, update);
        if entity.model_changed() {
            match self.models[entity.model_id].kind() {
                ModelKind::None => (),
                _ => {
                    entity.sync_base = match self.models[entity.model_id].sync_type() {
                        SyncType::Sync => Duration::zero(),
                        SyncType::Rand => unimplemented!(), // TODO
                    }
                }
            }
        }

        if let Some(_c) = entity.colormap() {
            // only players may have custom colormaps
            if id > self.max_players {
                warn!(
                    "Server attempted to set colormap on entity {}, which is not a player",
                    id
                );
            }
            // TODO: set player custom colormaps
        }

        Ok(())
    }*/



    /*
    
        When particles spawn, they need to spawn as an entity 

        particles should use ECS.. ? 
    
    */

    pub fn spawn_temp_entity(&mut self, temp_entity: &TempEntity) {
        lazy_static! {
            static ref ZERO_ONE_DISTRIBUTION: Uniform<f32> = Uniform::new(0.0, 1.0);
        }

        let mut spike_sound = || match ZERO_ONE_DISTRIBUTION.sample(&mut self.rng) {
            x if x < 0.2 => "weapons/tink1.wav",
            x if x < 0.4667 => "weapons/ric1.wav",
            x if x < 0.7333 => "weapons/ric2.wav",
            _ => "weapons/ric3.wav",
        };

        match temp_entity {
            TempEntity::Point { kind, origin } => {
                use PointEntityKind::*;
                match kind {
                    // projectile impacts
                    WizSpike | KnightSpike | Spike | SuperSpike | Gunshot => {
                        let (color, count, sound) = match kind {
                            // TODO: start wizard/hit.wav
                            WizSpike => (20, 30, Some("wizard/hit.wav")),

                            KnightSpike => (226, 20, Some("hknight/hit.wav")),

                            // TODO: for Spike and SuperSpike, start one of:
                            // - 26.67%: weapons/tink1.wav
                            // - 20.0%: weapons/ric1.wav
                            // - 20.0%: weapons/ric2.wav
                            // - 20.0%: weapons/ric3.wav
                            Spike => (0, 10, Some(spike_sound())),
                            SuperSpike => (0, 20, Some(spike_sound())),

                            // no impact sound
                            Gunshot => (0, 20, None),
                            _ => unreachable!(),
                        };

                        self.particles.create_projectile_impact(
                            self.time,
                            *origin,
                            Vector3::zero(),
                            color,
                            count,
                        );

                        if let Some(snd) = sound {
                            self.mixer.start_sound(
                                self.loaded_assets_cache.get_sound(snd).unwrap().clone(),
                                self.time,
                                None,
                                0,
                                1.0,
                                1.0,
                                *origin,
                                &self.listener,
                            );
                        }
                    }

                    Explosion => {
                        self.particles.create_explosion(self.time, *origin);
                        self.lights.insert(
                            self.time,
                            LightDesc {
                                origin: *origin,
                                init_radius: 350.0,
                                decay_rate: 300.0,
                                min_radius: None,
                                ttl: Duration::milliseconds(500),
                            },
                            None,
                        );

                        self.mixer.start_sound(
                            self.loaded_assets_cache
                                .get_sound("weapons/r_exp3.wav")
                                .unwrap()
                                .clone(),
                            self.time,
                            None,
                            0,
                            1.0,
                            1.0,
                            *origin,
                            &self.listener,
                        );
                    }

                    ColorExplosion {
                        color_start,
                        color_len,
                    } => {
                        self.particles.create_color_explosion(
                            self.time,
                            *origin,
                            (*color_start)..=(*color_start + *color_len - 1),
                        );
                        self.lights.insert(
                            self.time,
                            LightDesc {
                                origin: *origin,
                                init_radius: 350.0,
                                decay_rate: 300.0,
                                min_radius: None,
                                ttl: Duration::milliseconds(500),
                            },
                            None,
                        );

                        self.mixer.start_sound(
                            self.loaded_assets_cache
                                .get_sound("weapons/r_exp3.wav")
                                .unwrap()
                                .clone(),
                            self.time,
                            None,
                            0,
                            1.0,
                            1.0,
                            *origin,
                            &self.listener,
                        );
                    }

                    TarExplosion => {
                        self.particles.create_spawn_explosion(self.time, *origin);

                        self.mixer.start_sound(
                            self.loaded_assets_cache
                                .get_sound("weapons/r_exp3.wav")
                                .unwrap()
                                .clone(),
                            self.time,
                            None,
                            0,
                            1.0,
                            1.0,
                            *origin,
                            &self.listener,
                        );
                    }

                    LavaSplash => self.particles.create_lava_splash(self.time, *origin),
                    Teleport => self.particles.create_teleporter_warp(self.time, *origin),
                }
            }

            TempEntity::Beam {
                kind,
                entity_id,
                start,
                end,
            } => {
                use BeamEntityKind::*;
                let model_name = match kind {
                    Lightning { model_id } => format!(
                        "progs/bolt{}.mdl",
                        match model_id {
                            1 => "",
                            2 => "2",
                            3 => "3",
                            x => panic!("invalid lightning model id: {}", x),
                        }
                    ),
                    Grapple => "progs/beam.mdl".to_string(),
                };

                self.spawn_beam(
                    self.time,
                    *entity_id as usize,
                    *self.loaded_assets_cache.model_names.get(&model_name).unwrap(),
                    *start,
                    *end,
                );
            }
        }
    }

    pub fn spawn_beam(
        &mut self,
        time: Duration,
        unit_id: usize,
        model_id: usize,
        start: Vector3<f32>,
        end: Vector3<f32>,
    ) {
        // always override beam with same entity_id if it exists
        // otherwise use the first free slot
        let mut free = None;
        for i in 0..self.beams.len() {
            if let Some(ref mut beam) = self.beams[i] {
                if beam.unit_id == unit_id {
                    beam.model_id = model_id;
                    beam.expire = time + Duration::milliseconds(200);
                    beam.start = start;
                    beam.end = end;
                }
            } else if free.is_none() {
                free = Some(i);
            }
        }

        if let Some(i) = free {
            self.beams[i] = Some(Beam {
                unit_id,
                model_id,
                expire: time + Duration::milliseconds(200),
                start,
                end,
            });
        } else {
            warn!("No free beam slots!");
        }
    }

    pub fn update_listener(&self) {

        let controlled_entity_phys_comp = self.get_component_of_entity::
        <ecs_components::physics::PhysicsComponent>
        (self.view.unit_id()).unwrap();
 

 
        let view_origin = controlled_entity_phys_comp.origin;
        let world_translate = Matrix4::from_translation(view_origin);

        let left_base = Vector3::new(0.0, 4.0, self.view.view_height());
        let right_base = Vector3::new(0.0, -4.0, self.view.view_height());

        let rotate = self.view.input_angles().mat4_quake();

        let left = (world_translate * rotate * left_base.extend(1.0)).truncate();
        let right = (world_translate * rotate * right_base.extend(1.0)).truncate();

        self.listener.set_origin(view_origin);
        self.listener.set_left_ear(left);
        self.listener.set_right_ear(right);
    }

    pub fn update_sound_spatialization(&self) {
        self.update_listener();

        // update entity sounds
        // do this in a system ! ECS
        for e_channel in self.mixer.iter_entity_channels() {
 

            if let Some(ent_id) = e_channel.entity_id() {

  
                let controlled_entity_phys_comp = self.get_component_of_entity::
                <ecs_components::physics::PhysicsComponent>
                (self.view.unit_id()).unwrap();
          

                if e_channel.channel().in_use() {
                    e_channel
                        .channel()
                        .update(controlled_entity_phys_comp.origin, &self.listener);
                }
            }
        }

        // update static sounds
        for ss in self.static_sounds.iter() {
            ss.update(&self.listener);
        }
    }

    fn view_leaf_contents(&self) -> Result<bsp::BspLeafPhysMaterial, ClientError> {

        let controlled_entity_phys_comp = self.get_component_of_entity::
        <ecs_components::physics::PhysicsComponent>
        (self.view.unit_id()).unwrap();
 

        let world_model_kind = self.loaded_assets_cache.models[1].kind();
        
        match world_model_kind {
            ModelKind::Brush(ref bmodel) => {
                let bsp_data = bmodel.bsp_data();
                let leaf_id = bsp_data.find_leaf(controlled_entity_phys_comp.origin);
                let leaf = &bsp_data.leaves()[leaf_id];
                Ok(leaf.contents)
            }
            _ => panic!("non-brush worldmodel"),
        }
    }

   
    pub fn update_color_shifts(&mut self, frame_time: Duration) -> Result<(), ClientError> {
        let float_time = engine::duration_to_f32(frame_time);

        // set color for leaf contents
        self.color_shifts[ColorShiftCode::Contents as usize].replace(
            match self.view_leaf_contents()? {
                bsp::BspLeafPhysMaterial::Empty => ColorShift {
                    dest_color: [0, 0, 0],
                    percent: 0,
                },
                bsp::BspLeafPhysMaterial::Lava => ColorShift {
                    dest_color: [255, 80, 0],
                    percent: 150,
                },
                bsp::BspLeafPhysMaterial::Slime => ColorShift {
                    dest_color: [0, 25, 5],
                    percent: 150,
                },
                _ => ColorShift {
                    dest_color: [130, 80, 50],
                    percent: 128,
                },
            },
        );

        // decay damage and item pickup shifts
        // always decay at least 1 "percent" (actually 1/255)
        // TODO: make percent an actual percent ([0.0, 1.0])
        let mut dmg_shift = self.color_shifts[ColorShiftCode::Damage as usize].borrow_mut();
        dmg_shift.percent -= ((float_time * 150.0) as i32).max(1);
        dmg_shift.percent = dmg_shift.percent.max(0);

        let mut bonus_shift = self.color_shifts[ColorShiftCode::Bonus as usize].borrow_mut();
        bonus_shift.percent -= ((float_time * 100.0) as i32).max(1);
        bonus_shift.percent = bonus_shift.percent.max(0);

        // set power-up overlay
        self.color_shifts[ColorShiftCode::Powerup as usize].replace(
            if self.items.contains(ItemFlags::QUAD) {
                ColorShift {
                    dest_color: [0, 0, 255],
                    percent: 30,
                }
            } else if self.items.contains(ItemFlags::SUIT) {
                ColorShift {
                    dest_color: [0, 255, 0],
                    percent: 20,
                }
            } else if self.items.contains(ItemFlags::INVISIBILITY) {
                ColorShift {
                    dest_color: [100, 100, 100],
                    percent: 100,
                }
            } else if self.items.contains(ItemFlags::INVULNERABILITY) {
                ColorShift {
                    dest_color: [255, 255, 0],
                    percent: 30,
                }
            } else {
                ColorShift {
                    dest_color: [0, 0, 0],
                    percent: 0,
                }
            },
        );

        Ok(())
    }


    /// Update the view angles to the specified value, disabling interpolation.
    pub fn set_view_angles(&mut self, angles: Vector3<Deg<f32>>) {
        self.view.update_input_angles(Angles {
            pitch: angles.x,
            roll: angles.z,
            yaw: angles.y,
        });
        let final_angles = self.view.final_angles();


        let controlled_entity_phys_comp = self.get_mut_component_of_entity::
        <ecs_components::physics::PhysicsComponent>
        (self.view.unit_id()  );
 

        match controlled_entity_phys_comp {
            Some( mut phys_comp  ) => {

                    println!("set angles of my char {:?}", final_angles);
                    phys_comp.set_angles(Vector3::new(
                        final_angles.pitch,
                        final_angles.yaw,
                        final_angles.roll,
                    ));

                }
            _ => {} 
        }

    }

    /// Update the view angles to the specified value, enabling interpolation.
    /// isnt this deprecated ?
    pub fn update_view_angles(&mut self, angles: Vector3<Deg<f32>>) {

        self.view.update_input_angles(Angles {
            pitch: angles.x,
            roll: angles.z,
            yaw: angles.y,
        });

        let final_angles = self.view.final_angles();


        let controlled_entity_phys_comp = self.get_mut_component_of_entity::
        <ecs_components::physics::PhysicsComponent>
        (  self.view.unit_id()  );  
         
         
        match controlled_entity_phys_comp {
            Some(mut phys_comp) => {

                println!("Setting view angles of my comp !");
                phys_comp.set_angles(Vector3::new(
                    final_angles.pitch,
                    final_angles.yaw,
                    final_angles.roll,
                ));

            }
            _ => {} 
        }

      
    }

    //each players view entity id == their player number (?)
    pub fn set_view_entity(&mut self, unit_id: usize) -> Result<(), ClientError> {
        // view entity may not have been spawned yet, so check
        // against both max_players and the current number of
        // entities
        /*if entity_id > self.max_players && entity_id >= self.entities.len() {
            Err(ClientError::InvalidViewEntity(entity_id))?;
        } */
           
        if !self.unit_exists(unit_id) && unit_id > self.max_players {
            Err(ClientError::InvalidViewEntity(unit_id))?;
        }

        self.view.set_unit_id(unit_id);
        Ok(())
    }

    pub fn models(&self) -> &[Model] {
        &self.loaded_assets_cache.models
    } 

 

    pub fn play_sound(&mut self,sound_id:usize,volume:u8,channel:i8,attenuation:f32, unit_id:usize, position:Vector3<f32>) {
         // TODO: apply volume, attenuation, spatialization
         self.mixer.start_sound(
            self.loaded_assets_cache.sounds[sound_id as usize].clone(),
            self.msg_times[0],
            Some(unit_id as usize),
            channel,
            volume as f32 / 255.0,
            attenuation,
            position,
            &self.listener,
        );

    }

    //i guess this is the model of the weapon you are holding 
    pub fn viewmodel_id(&self) -> usize {
        match self.stats[ClientStat::Weapon as usize] as usize {
            0 => 0,
            x => x - 1,
        }
    }

   // we dont do this in ecs!   old way to render...

/* 
   pub fn query_visible_entities<'w , 's  >(&self)  -> dyn FromIterator<&PhysicsComponent> + 'static
    
    {        
    //pass in an iterator of component bundle ? 

    //filtered?  
 
    //query for physics components and model components and whatever else --- for the render state 

    let ecs_world =  &self.ecs_world;

    let query =   ecs_world.query::<  (&PhysicsComponent) >();
 

    let iter = query.iter(&self.ecs_world);
        
    return  iter.collect();
  
    }


    pub fn iter_visible_entities(&self) -> impl Iterator<Item = &ClientUnit> + Clone {
        
        //pass in an iterator of component bundle ? 

        return  self.temp_entities.iter().chain(self.static_entities.iter())
        
        /* self.visible_entity_ids
             .iter()
            .map(move |i| &self.entities[*i])
            .chain(self.temp_entities.iter())
            .chain(self.static_entities.iter()) */
    }*/
 

    pub fn iter_particles(&mut self) -> impl Iterator<Item = &Particle> {
        self.particles.iter()
    }  

    pub fn iter_lights(&self) -> impl Iterator<Item = &Light> {
        self.lights.iter()
    }

    pub fn time(&self) -> Duration {
        self.time
    }

    pub fn view_unit_id(&self) -> usize {
        self.view.unit_id()
    }

    pub fn player_id(&self) -> usize {
        return 1; //FIX ME 
    }


    pub fn tick_count(&self) -> usize {
        return 1; //FIX ME 
    }

    pub fn camera(&self, aspect: f32, fov: Deg<f32>) -> Camera {
        let fov_y = math::fov_x_to_fov_y(fov, aspect).unwrap();
        Camera::new(
            self.view.final_origin(),
            self.view.final_angles(),
            cgmath::perspective(fov_y, aspect, 4.0, 4096.0),
        )
    }

    pub fn demo_camera(&self, aspect: f32, fov: Deg<f32>) -> Camera {
        let fov_y = math::fov_x_to_fov_y(fov, aspect).unwrap();

        let phys_comp = self.get_component_of_entity::
        <ecs_components::physics::PhysicsComponent>
        (self.view.unit_id()).unwrap();

    
        let angles = phys_comp.angles;


        Camera::new(
            self.view.final_origin(),
            Angles {
                pitch: angles.x,
                roll: angles.z,
                yaw: angles.y,
            },
            cgmath::perspective(fov_y, aspect, 4.0, 4096.0),
        )
    }

    //for testing  e1m3 
    pub fn fake_camera(&self, aspect: f32, fov: Deg<f32>) -> Camera {
        let fov_y = math::fov_x_to_fov_y(fov, aspect).unwrap();
 
       // let angles = self.entities[self.view.unit_id()].angles;


        Camera::new(
            Vector3 {
                x: -735.96875,
                y: -1591.9688,  
                z:  112.4 
            },
            Angles {
                pitch: Deg(22.0),
                roll: Deg(0.0),
                yaw: Deg(0.0),
            },
            cgmath::perspective(fov_y, aspect, 4.0, 4096.0),
        )
    } 

    pub fn insert_lightstyle( &mut self, id:u8, value: String ) {

        let mut scene_render_constants = self.get_resource_mut::<RenderSceneConstants>();

        scene_render_constants.light_styles.insert(id, value);

    }

    //ridiculous that we need mut self now (due to bevy query) but oh well ? 
    pub fn lightstyle_values(&self) -> Result<ArrayVec<f32, 64>, ClientError> {
        let mut values = ArrayVec::new();

        let scene_render_constants = self.get_resource::<RenderSceneConstants>();

        let light_styles = &scene_render_constants.light_styles;

        for lightstyle_id in 0..64 {
            match light_styles.get(&lightstyle_id) {
                Some(ls) => {
                    let float_time = engine::duration_to_f32(self.time);
                    let frame = if ls.len() == 0 {
                        None
                    } else {
                        Some((float_time * 10.0) as usize % ls.len())
                    };

                    values.push(match frame {
                        // 'z' - 'a' = 25, so divide by 12.5 to get range [0, 2]
                        Some(f) => (ls.as_bytes()[f] - 'a' as u8) as f32 / 12.5,
                        None => 1.0,
                    })
                }

                None => Err(ClientError::NoSuchLightmapAnimation(lightstyle_id as usize))?,
            }
        }

        Ok(values)
    }

    pub fn intermission(&self) -> Option<&IntermissionKind> {
        self.intermission.as_ref()
    }

    pub fn start_time(&self) -> Duration {
        self.start_time
    }

    pub fn completion_time(&self) -> Option<Duration> {
        self.completion_time
    }

    pub fn stats(&self) -> &[i32] {
        &self.stats
    }

    pub fn items(&self) -> ItemFlags {
        self.items
    }

    pub fn item_pickup_times(&self) -> &[Duration] {
        &self.item_get_time
    }

    pub fn face_anim_time(&self) -> Duration {
        self.face_anim_time
    }

    pub fn color_shift(&self) -> [f32; 4] {
        self.color_shifts.iter().fold([0.0; 4], |accum, elem| {
            let elem_a = elem.borrow().percent as f32 / 255.0 / 2.0;
            if elem_a == 0.0 {
                return accum;
            }
            let in_a = accum[3];
            let out_a = in_a + elem_a * (1.0 - in_a);
            let color_factor = elem_a / out_a;

            let mut out = [0.0; 4];
            for i in 0..3 {
                out[i] = accum[i] * (1.0 - color_factor)
                    + elem.borrow().dest_color[i] as f32 / 255.0 * color_factor;
            }
            out[3] = out_a.min(1.0).max(0.0);
            out
        })
    }

   /*  pub fn check_entity_id(&self, id: usize) -> Result<(), ClientError> {
        match id {
            0 => Err(ClientError::NullEntity),
            e if e >= self.entities.len() => Err(ClientError::NoSuchEntity(id)),
            _ => Ok(()),
        }
    }*/

    pub fn check_player_id(&self, id: usize) -> Result<(), ClientError> {
        if id >= net::MAX_CLIENTS {
            Err(ClientError::NoSuchClient(id))
        } else if id > self.max_players {
            Err(ClientError::NoSuchPlayer(id))
        } else {
            Ok(())
        }
    }
}