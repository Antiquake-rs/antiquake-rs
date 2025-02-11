

use std::{
    thread::{self},
    cell::{Ref, RefCell},
    collections::{HashMap,VecDeque},
    rc::Rc,
    net::{ToSocketAddrs,SocketAddr},
    io::{self, BufRead},
    fs::File,
    fmt::{self,Display},
    ops::Range
}; 

use crate::{
    server::precache::{Precache,MAX_PRECACHE_ENTRIES,MAX_PRECACHE_PATH},
    common::{
        parse,
        vfs::{Vfs,VirtualFile},
        math::Hyperplane,
        cvars::{register_cvars},
        model::{Model,ModelFlags,ModelKind},
        bsp::{self},
        console::CvarRegistry,
        engine::{duration_from_f32, duration_to_f32}
    },
    server::world::{
        phys::{self, CollideKind, CollisionFlags, Trace, TraceEndKind},
        MoveKind, EntityFlags, EntitySolid,  World,
    },
    
    server::slime::{
        Slime
    },
    server::{ClientSlots},
    server::slime::{
        SlimeContext, 
        context::SlimeFunc},
};



use cgmath::{InnerSpace, Vector3, Zero, Deg};

use arrayvec::{ArrayVec,ArrayString};
 

const MAX_DATAGRAM: usize = 1024;
const MAX_LIGHTSTYLES: usize = 64;

use chrono::Duration;

use super::{world::{EntityId, WorldError}, slime::context::InternalCallArgs};






#[derive(Debug)]
pub enum LevelError {
    Io(::std::io::Error),
  
    World(WorldError),
     
    Other(String),
}

impl LevelError {
    pub fn with_msg<S>(msg: S) -> Self
    where
        S: AsRef<str>,
    {
        LevelError::Other(msg.as_ref().to_owned())
    }
}

impl From<WorldError> for LevelError {
    fn from(error: WorldError) -> Self {
        LevelError::World(error)
    }
}


// Server-side level state.
#[derive(Debug)]
pub struct LevelState {
    vfs: Rc<Vfs>,
    cvars: Rc<RefCell<CvarRegistry>>,

   // string_table: Rc<RefCell<StringTable>>,
    sound_precache: Precache,
    model_precache: Precache,
    lightstyles:  ArrayVec<String, MAX_LIGHTSTYLES>,//  [String ; MAX_LIGHTSTYLES],


    entmap: String,

    /// Amount of time the current level has been active.
    time: Duration,

    /// QuakeC bytecode execution context.
    ///
    /// This includes the program counter, call stack, and local variables.
   // cx: ExecutionContext,
   slime_context: SlimeContext,

    /// Global values for QuakeC bytecode.
    //globals: Globals,

    /// The state of the game world.
    ///
    /// This contains the entities and world geometry.
    world: World,

    datagram: ArrayVec<u8, MAX_DATAGRAM>,
}

impl LevelState {
    pub fn new(
        vfs: Rc<Vfs>,
        cvars: Rc<RefCell<CvarRegistry>>,
        slime: Slime,
        
        map_name: String,
        brush_models: Vec<Model>, // brush models 
        entmap: String,
    ) -> LevelState {

 
        let Slime {
            slime_context,
          
        } = slime ;

     
        //any model you want to use including bsp MUST be precached 
        
        let mut sound_precache = Precache::new();
       // sound_precache.precache("");

        

        let mut model_precache = Precache::new();
        model_precache.precache(map_name);  //a dirty hack -- ?
         
    

        for model in brush_models.iter() {
            

            println!("brush model is {}",model.name());

           

            //find or insert the model name into string table 
            //let model_name = (*string_table).borrow_mut().find_or_insert(model.name());

         //   println!("model stringid is {}",model_name);


            //add the string to the precache 
            model_precache.precache( model.name() );
        }
 

        let world = World::create(  brush_models ).unwrap();
       
     
        //add to me from slime ? 

        let mut level = LevelState {
            vfs,
            cvars,
            //string_table,
            sound_precache,
            model_precache,
            lightstyles:  ArrayVec::new(), //[String::from(""); MAX_LIGHTSTYLES],
            

            entmap: entmap.to_owned(),
            time: Duration::zero(),

          //  cx,
            slime_context,
           // globals,
            world,

            datagram: ArrayVec::new(),
        };


        level.execute_slime_script("world","fn_prepare").unwrap();
         

        let entity_list = parse::entities(&entmap).unwrap();

        for entity in entity_list {
            level.spawn_entity_from_map(entity).unwrap();
        }

        level
    }


    pub fn get_time(&self) -> Duration{
        return self.time
    }

    #[inline]
    pub fn precache_sound(&mut self, name: String)   {

        
           //find or insert the model name into string table 
          

          println!("Precaching sound {}", name );

           //add the string to the precache 
           self.sound_precache.precache(name );
 
    }


    //this is run by the progs.dat or slime !  
    #[inline]
    pub fn precache_model(&mut self, name: String) {

           //find or insert the model name into string table 
       
            println!("Precaching model {}", name );

           //add the string to the precache    
           self.model_precache.precache(name);
 
    }


/*
    In richter, there was a StringId  (correlated w string table)  (i think this is bc of how progs dat works ) for a model/sound so then you needed to find the ModelId (or soundId) 
     using the string table then you needed to find the  sound id (which is also the precache id)

     I think progs dat would export a string table and all of the string values in the code would be executed as a stringId (a pointer) to the strings table.  

     For antiquake, i believe i can largely do away with the strings table for context execution. 


     In antiquake, the slime config files can execute using full strings since it is parsed TOML.  
*/

    #[inline]
    pub fn sound_id(&self, name: String) -> Option<usize> {
        /*let name = Ref::map(self.string_table.borrow(), |this| {
            this.get(name_id).unwrap()
        });*/
        self.sound_precache.find(&*name)
    }

    #[inline]
    pub fn model_id(&self, name: String) -> Option<usize> {
      /*   let name = Ref::map(self.string_table.borrow(), |this| {
            this.get(name_id).unwrap()
        });*/
        self.model_precache.find(&*name)
    }

    #[inline]
    pub fn set_lightstyle(&mut self, index: usize, val: String )   {
        self.lightstyles[index] = val;
    }


 /*    pub fn get_entity_list(&self) ->  Vec<&Entity>   {

      // let entity_list:Vec<&Entity> = Vec::new();   

       let entity_id_list = self.world.list_entities(); 

       let  entity_list:Vec<&Entity> = entity_id_list.iter().map( |id|   self.world.entity(  id  )   );
 
       return entity_list; 

    }*/

    pub fn get_all_world_model_names(&self) ->  Vec<String>   {
         
        return self.world.get_model_names();

    }


    pub fn get_model_precache_data(&self) ->  Vec<String> {

        return self.model_precache.get_data( )
    }

    
    pub fn get_sound_precache_data(&self) ->  Vec<String> {

        return self.sound_precache.get_data( )
    }



    //need to pass in args here 

    pub fn execute_subroutine(&mut self, sub_rt: SlimeFunc) -> Result<(), LevelError>{


        return match sub_rt {


       //     Spawn => self.builtin_spawn()?,
       //     Remove => self.builtin_remove()?,

            SlimeFunc::PrecacheSound {name}  => self.builtin_precache_sound(name) ,
            SlimeFunc::PrecacheModel {name}  => self.builtin_precache_model(name) ,

            SlimeFunc::InternalCall(internal_call_args)  => self.builtin_internal_call( internal_call_args ) ,
       

            _ => Err(LevelError::Other(format!("that subroutine not yet implemented")))   
        };

     //   Ok(())


    }




    /// Execute a QuakeC function in the VM.
    /// 
    /// we can do this in Rhai instead of QuakeC 

    /* 
    pub fn execute_program(&mut self, f: FunctionId) -> Result<(), ProgsError> {
        let mut runaway = 100000;

        let exit_depth = self.cx.call_stack_depth();

        self.cx.enter_function(&mut self.globals, f)?;

        while self.cx.call_stack_depth() != exit_depth {
            runaway -= 1;

            if runaway == 0 {
                panic!("runaway program");
            }

            let statement = self.cx.load_statement();
            let op = statement.opcode;
            let a = statement.arg1;
            let b = statement.arg2;
            let c = statement.arg3;

            debug!(
                "              {:<9} {:>5} {:>5} {:>5}",
                format!("{:?}", op),
                a,
                b,
                c
            );

            use Opcode::*;

            // Y'all like jump tables?
            match op {
                // Control flow ================================================
                If => {
                    let cond = self.globals.get_float(a)? != 0.0;
                    log::debug!("If: cond == {}", cond);

                    if cond {
                        self.cx.jump_relative(b);
                        continue;
                    }
                }

                IfNot => {
                    let cond = self.globals.get_float(a)? != 0.0;
                    log::debug!("IfNot: cond == {}", cond);

                    if !cond {
                        self.cx.jump_relative(b);
                        continue;
                    }
                }

                Goto => {
                    self.cx.jump_relative(a);
                    continue;
                }

                Call0 | Call1 | Call2 | Call3 | Call4 | Call5 | Call6 | Call7 | Call8 => {
                    // TODO: pass to equivalent of PF_VarString
                    let _arg_count = op as usize - Opcode::Call0 as usize;

                    let f_to_call = self.globals.function_id(a)?;
                    if f_to_call.0 == 0 {
                        panic!("NULL function");
                    }

                    let name_id = self.cx.function_def(f_to_call)?.name_id;
                    let name = self.string_table.borrow().get(name_id).unwrap().to_owned();

                    if let FunctionKind::BuiltIn(b) = self.cx.function_def(f_to_call)?.kind {
                        debug!("Calling built-in function {}", name);
                        use BuiltinFunctionId::*;
                        match b {
                            MakeVectors => self.globals.make_vectors()?,
                            SetOrigin => self.builtin_set_origin()?,
                            SetModel => self.builtin_set_model()?,
                            SetSize => self.builtin_set_size()?,
                            Break => unimplemented!(),
                            Random => self.globals.builtin_random()?,
                            Sound => unimplemented!(),
                            Normalize => unimplemented!(),
                            Error => unimplemented!(),
                            ObjError => unimplemented!(),
                            VLen => self.globals.builtin_v_len()?,
                            VecToYaw => self.globals.builtin_vec_to_yaw()?,
                            Spawn => self.builtin_spawn()?,
                            Remove => self.builtin_remove()?,
                            TraceLine => unimplemented!(),
                            CheckClient => unimplemented!(),
                            Find => unimplemented!(),
                            PrecacheSound => self.builtin_precache_sound()?,
                            PrecacheModel => self.builtin_precache_model()?,
                            StuffCmd => unimplemented!(),
                            FindRadius => unimplemented!(),
                            BPrint => unimplemented!(),
                            SPrint => unimplemented!(),
                            DPrint => self.builtin_dprint()?,
                            FToS => unimplemented!(),
                            VToS => unimplemented!(),
                            CoreDump => unimplemented!(),
                            TraceOn => unimplemented!(),
                            TraceOff => unimplemented!(),
                            EPrint => unimplemented!(),
                            WalkMove => unimplemented!(),

                            DropToFloor => self.builtin_drop_to_floor()?,
                            LightStyle => self.builtin_light_style()?,
                            RInt => self.globals.builtin_r_int()?,
                            Floor => self.globals.builtin_floor()?,
                            Ceil => self.globals.builtin_ceil()?,
                            CheckBottom => unimplemented!(),
                            PointContents => unimplemented!(),
                            FAbs => self.globals.builtin_f_abs()?,
                            Aim => unimplemented!(),
                            Cvar => self.builtin_cvar()?,
                            LocalCmd => unimplemented!(),
                            NextEnt => unimplemented!(),
                            Particle => unimplemented!(),
                            ChangeYaw => unimplemented!(),
                            VecToAngles => unimplemented!(),
                            WriteByte => unimplemented!(),
                            WriteChar => unimplemented!(),
                            WriteShort => unimplemented!(),
                            WriteLong => unimplemented!(),
                            WriteCoord => unimplemented!(),
                            WriteAngle => unimplemented!(),
                            WriteString => unimplemented!(),
                            WriteEntity => unimplemented!(),
                            MoveToGoal => unimplemented!(),
                            PrecacheFile => unimplemented!(),
                            MakeStatic => unimplemented!(),
                            ChangeLevel => unimplemented!(),
                            CvarSet => self.builtin_cvar_set()?,
                            CenterPrint => unimplemented!(),
                            AmbientSound => self.builtin_ambient_sound()?,
                            PrecacheModel2 => unimplemented!(),
                            PrecacheSound2 => unimplemented!(),
                            PrecacheFile2 => unimplemented!(),
                            SetSpawnArgs => unimplemented!(),
                        }
                        debug!("Returning from built-in function {}", name);
                    } else {
                        self.cx.enter_function(&mut self.globals, f_to_call)?;
                        continue;
                    }
                }

                Done | Return => self.op_return(a, b, c)?,

                MulF => self.globals.op_mul_f(a, b, c)?,
                MulV => self.globals.op_mul_v(a, b, c)?,
                MulFV => self.globals.op_mul_fv(a, b, c)?,
                MulVF => self.globals.op_mul_vf(a, b, c)?,
                Div => self.globals.op_div(a, b, c)?,
                AddF => self.globals.op_add_f(a, b, c)?,
                AddV => self.globals.op_add_v(a, b, c)?,
                SubF => self.globals.op_sub_f(a, b, c)?,
                SubV => self.globals.op_sub_v(a, b, c)?,
                EqF => self.globals.op_eq_f(a, b, c)?,
                EqV => self.globals.op_eq_v(a, b, c)?,
                EqS => self.globals.op_eq_s(a, b, c)?,
                EqEnt => self.globals.op_eq_ent(a, b, c)?,
                EqFnc => self.globals.op_eq_fnc(a, b, c)?,
                NeF => self.globals.op_ne_f(a, b, c)?,
                NeV => self.globals.op_ne_v(a, b, c)?,
                NeS => self.globals.op_ne_s(a, b, c)?,
                NeEnt => self.globals.op_ne_ent(a, b, c)?,
                NeFnc => self.globals.op_ne_fnc(a, b, c)?,
                Le => self.globals.op_le(a, b, c)?,
                Ge => self.globals.op_ge(a, b, c)?,
                Lt => self.globals.op_lt(a, b, c)?,
                Gt => self.globals.op_gt(a, b, c)?,
                LoadF => self.op_load_f(a, b, c)?,
                LoadV => self.op_load_v(a, b, c)?,
                LoadS => self.op_load_s(a, b, c)?,
                LoadEnt => self.op_load_ent(a, b, c)?,
                LoadFld => panic!("load_fld not implemented"),
                LoadFnc => self.op_load_fnc(a, b, c)?,
                Address => self.op_address(a, b, c)?,
                StoreF => self.globals.op_store_f(a, b, c)?,
                StoreV => self.globals.op_store_v(a, b, c)?,
                StoreS => self.globals.op_store_s(a, b, c)?,
                StoreEnt => self.globals.op_store_ent(a, b, c)?,
                StoreFld => self.globals.op_store_fld(a, b, c)?,
                StoreFnc => self.globals.op_store_fnc(a, b, c)?,
                StorePF => self.op_storep_f(a, b, c)?,
                StorePV => self.op_storep_v(a, b, c)?,
                StorePS => self.op_storep_s(a, b, c)?,
                StorePEnt => self.op_storep_ent(a, b, c)?,
                StorePFld => panic!("storep_fld not implemented"),
                StorePFnc => self.op_storep_fnc(a, b, c)?,
                NotF => self.globals.op_not_f(a, b, c)?,
                NotV => self.globals.op_not_v(a, b, c)?,
                NotS => self.globals.op_not_s(a, b, c)?,
                NotEnt => self.globals.op_not_ent(a, b, c)?,
                NotFnc => self.globals.op_not_fnc(a, b, c)?,
                And => self.globals.op_and(a, b, c)?,
                Or => self.globals.op_or(a, b, c)?,
                BitAnd => self.globals.op_bit_and(a, b, c)?,
                BitOr => self.globals.op_bit_or(a, b, c)?,

                State => self.op_state(a, b, c)?,
            }

            // Increment program counter.
            self.cx.jump_relative(1);
        }

        Ok(())
    }

    pub fn execute_program_by_name<S>(&mut self, name: S) -> Result<(), ProgsError>
    where
        S: AsRef<str>,
    {
        let func_id = self.cx.find_function_by_name(name)?;
        self.execute_program(func_id)?;
        Ok(())
    }
*/
    /// Link an entity into the `World`.
    ///
    /// If `touch_triggers` is `true`, this will invoke the touch function of
    /// any trigger the entity is touching.
    pub fn link_entity(
        &mut self,
        ent_id: EntityId,
        touch_triggers: bool,
    ) -> Result<(), LevelError> {
        self.world.link_entity(ent_id)?;

      /* //add this back in !  
       if touch_triggers {
            self.touch_triggers(ent_id)?;
        }*/ 

        Ok(())
    }

    pub fn spawn_entity(&mut self) -> Result<EntityId, LevelError> {
        let ent_id = self.world.alloc_uninitialized()?;

        self.link_entity(ent_id, false)?;

        Ok(ent_id)
    }
    

    pub fn spawn_entity_from_map(
        &mut self,
        map: HashMap<&str, &str>,
    ) -> Result<EntityId, LevelError> {
        let classname = match map.get("classname") {
            Some(c) => c.to_owned(),
            None => return Err(LevelError::with_msg("No classname for entity")),
        };

        let entity_id = self.world.alloc_from_map(map)?;

        // TODO: set origin, mins and maxs here if needed

        // set `self` before calling spawn function
       // self.globals
       //     .put_entity_id(ent_id, GlobalAddrEntity::Self_ as i16)?;


         // dont execute progs.dat program right now since some funcs are not impl 
    
        //this populates the precache !!! 
        self.execute_slime_script_for_entity(classname,"fn_prepare",  entity_id.0 as usize )?;


        self.link_entity(entity_id, true)?;

        Ok(entity_id)
    }
/* 
    pub fn set_entity_origin(
        &mut self,
        ent_id: EntityId,
        origin: Vector3<f32>,
    ) -> Result<(), ProgsError> {
        self.world
            .entity_mut(ent_id)?
            .store(FieldAddrVector::Origin, origin.into())?;
        self.link_entity(ent_id, false)?;

        Ok(())
    }

    pub fn set_entity_model(
        &mut self,
        ent_id: EntityId,
        model_name_id: StringId,
    ) -> Result<(), ProgsError> {
        let model_id = {
            let ent = self.world.entity_mut(ent_id)?;

            ent.put_string_id(model_name_id, FieldAddrStringId::ModelName as i16)?;

            let model_id = match self.string_table.borrow().get(model_name_id) {
                Some(name) => match self.model_precache.find(name) {
                    Some(i) => i,
                    None => return Err(ProgsError::with_msg("model not precached")),
                },
                None => return Err(ProgsError::with_msg("invalid StringId")),
            };

            ent.put_float(model_id as f32, FieldAddrFloat::ModelIndex as i16)?;

            model_id
        };

        self.world.set_entity_model(ent_id, model_id)?;

        Ok(())
    }
*/



    pub fn execute_slime_script (&mut self, classname: &str, methodname: &str) -> Result<(), LevelError>
    {
     
        let context = &self.slime_context;

        let subroutines = context.fetch_subroutines(classname,methodname); 

        for sub_rt in subroutines.into_iter(){
           let fn_result = self.execute_subroutine( sub_rt );
        }    
 
        Ok(())
    }


    pub fn execute_slime_script_for_entity(&mut self, classname: &str, methodname: &str, ent_id:usize) -> Result<(), LevelError>
    
    {
 

        println!("execute slime script for entity {}", ent_id  );


        let context = &self.slime_context;

        let subroutines = context.fetch_subroutines(classname,methodname);
 
        //for each subroutine 

        //execute that subroutine in this levelstate !!!! 

        for sub_rt in subroutines.into_iter(){
           let fn_result = self.execute_subroutine( sub_rt );
        }
       


       // let func_id = self.script_context.find_function_by_name(name)?;
      //  self.execute_program(func_id)?;
        Ok(())
    }




    pub fn builtin_internal_call(&mut self, internal_call_args: InternalCallArgs) -> Result<(), LevelError> {

        let context = &self.slime_context;

        let subroutines = context.fetch_subroutines(internal_call_args.classname.as_str(), internal_call_args.fn_name.as_str());
 
        for (sub_rt) in subroutines.into_iter(){
            let fn_result = self.execute_subroutine( sub_rt );
         }

         Ok(())
    }



    pub fn builtin_precache_sound(&mut self, name:String) -> Result<(), LevelError> {
        // TODO: disable precaching after server is active
        // TODO: precaching doesn't actually load yet
       // let s_id = self.globals.string_id(arg as i16)?;
        self.precache_sound(name);
      /*  self.globals
            .put_string_id(s_id, GLOBAL_ADDR_RETURN as i16)?;*/ //dont need to worry about populating the addr return buffer anymore ! 

        Ok(())
    }

    pub fn builtin_precache_model(&mut self, name:String) -> Result<(), LevelError> {
        println!("builtin_precache_model!!");
        // TODO: disable precaching after server is active
        // TODO: precaching doesn't actually load yet

        //let s_id = self.globals.string_id(arg as i16)?; //this was doing the lookup of the progsdat bytecode value (the memory register basically)


       /* self.globals
            .put_string_id(s_id, GLOBAL_ADDR_RETURN as i16)?;*/ 



        /*
            If the string has not been added to the precache, we should add it and then 
        
        */
        let existing_precache_model_id = self.model_id(name.clone());

        if existing_precache_model_id.is_none() {   //if it is not in the precache 
            let precache_model_name_id = self.precache_model(name.clone());
            self.world.add_model(&self.vfs,  name.clone()   )?;
        }


        Ok(())
    }



/* 
    pub fn think(&mut self, ent_id: EntityId, frame_time: Duration) -> Result<(), ProgsError> {
        let ent = self.world.entity_mut(ent_id)?;
        let think_time = duration_from_f32(ent.load(FieldAddrFloat::NextThink)?);

        if think_time <= Duration::zero() || think_time > self.time + frame_time {
            // Think either already happened or isn't due yet.
            return Ok(());
        }

        // Deschedule next think.
        ent.store(FieldAddrFloat::NextThink, 0.0)?;

        // Call entity's think function.
        let think = ent.load(FieldAddrFunctionId::Think)?;
        self.globals
            .store(GlobalAddrFloat::Time, duration_to_f32(think_time))?;
        self.globals.store(GlobalAddrEntity::Self_, ent_id)?;
        self.globals.store(GlobalAddrEntity::Other, EntityId(0))?;
        self.execute_program(think)?;

        Ok(())
    }

    pub fn physics(
        &mut self,
        clients: &ClientSlots,
        frame_time: Duration,
    ) -> Result<(), ProgsError> {
        self.globals.store(GlobalAddrEntity::Self_, EntityId(0))?;
        self.globals.store(GlobalAddrEntity::Other, EntityId(0))?;
        self.globals
            .store(GlobalAddrFloat::Time, duration_to_f32(self.time))?;

        let start_frame = self
            .globals
            .function_id(GlobalAddrFunction::StartFrame as i16)?;
        self.execute_program(start_frame)?;

        // TODO: don't alloc
        let mut ent_ids = Vec::new();

        self.world.list_entities(&mut ent_ids);

        for ent_id in ent_ids {
            if self.globals.load(GlobalAddrFloat::ForceRetouch)? != 0.0 {
                // Force all entities to touch triggers, even if they didn't
                // move. This is required when e.g. creating new triggers, as
                // stationary entities typically do not get relinked, and so
                // will ignore new triggers even when touching them.
                //
                // TODO: this may have a subtle ordering bug. If entity 2 has
                // physics run, sets ForceRetouch and spawns entity 1, then
                // entity 1 will not have a chance to touch triggers this frame.
                // Quake solves this by using a linked list and always spawning
                // at the end so that newly spawned entities always have physics
                // run this frame.
                self.link_entity(ent_id, true)?;
            }

            let max_clients = clients.limit();
            if ent_id.0 != 0 && ent_id.0 < max_clients {
                self.physics_player(clients, ent_id)?;
            } else {
                match self.world.entity(ent_id).move_kind()? {
                    MoveKind::Walk => {
                        todo!("MoveKind::Walk");
                    }

                    MoveKind::Push => self.physics_push(ent_id, frame_time)?,
                    // No actual physics for this entity, but still let it think.
                    MoveKind::None => self.think(ent_id, frame_time)?,
                    MoveKind::NoClip => self.physics_noclip(ent_id, frame_time)?,
                    MoveKind::Step => self.physics_step(ent_id, frame_time)?,

                    // all airborne entities have the same physics
                    _ => unimplemented!(),
                }
            }

            match self.globals.load(GlobalAddrFloat::ForceRetouch)? {
                f if f > 0.0 => self.globals.store(GlobalAddrFloat::ForceRetouch, f - 1.0)?,
                _ => (),
            }
        }

        // TODO: increase sv.time by host_frametime
        unimplemented!();
    }

    // TODO: rename arguments when implementing
    pub fn physics_player(
        &mut self,
        clients: &ClientSlots,
        ent_id: EntityId,
    ) -> Result<(), ProgsError> {
        let client_id = ent_id.0.checked_sub(1).ok_or_else(|| {
            ProgsError::with_msg(format!("Invalid client entity ID: {:?}", ent_id))
        })?;

        if clients.get(&(client_id as i32)).is_none() {
            // No client in this slot.
            return Ok(());
        }

        let ent = self.world.entity_mut(ent_id)?;
        ent.limit_velocity(self.cvars.borrow().get_value("sv_maxvelocity").unwrap())?;
        unimplemented!();
    }

    pub fn physics_push(
        &mut self,
        ent_id: EntityId,
        frame_time: Duration,
    ) -> Result<(), ProgsError> {
        let ent = self.world.entity_mut(ent_id)?;

        let local_time = duration_from_f32(ent.load(FieldAddrFloat::LocalTime)?);
        let next_think = duration_from_f32(ent.load(FieldAddrFloat::NextThink)?);

        let move_time = if local_time + frame_time > next_think {
            (next_think - local_time).max(Duration::zero())
        } else {
            frame_time
        };

        drop(ent);
        if !move_time.is_zero() {
            self.move_push(ent_id, frame_time, move_time)?;
        }

        let ent = self.world.entity_mut(ent_id)?;

        let old_local_time = local_time;
        let new_local_time = duration_from_f32(ent.load(FieldAddrFloat::LocalTime)?);

        // Let the entity think if it needs to.
        if old_local_time < next_think && next_think <= new_local_time {
            // Deschedule thinking.
            ent.store(FieldAddrFloat::NextThink, 0.0)?;

            self.globals
                .put_float(duration_to_f32(self.time), GlobalAddrFloat::Time as i16)?;
            self.globals
                .put_entity_id(ent_id, GlobalAddrEntity::Self_ as i16)?;
            self.globals
                .put_entity_id(EntityId(0), GlobalAddrEntity::Other as i16)?;

            let think = ent.function_id(FieldAddrFunctionId::Think as i16)?;
            self.execute_program(think)?;
        }

        Ok(())
    }

    pub fn physics_noclip(
        &mut self,
        ent_id: EntityId,
        frame_time: Duration,
    ) -> Result<(), ProgsError> {
        // Let entity think, then move if it didn't remove itself.
        self.think(ent_id, frame_time)?;

        if let Ok(ent) = self.world.entity_mut(ent_id) {
            let frame_time_f = duration_to_f32(frame_time);

            let angles: Vector3<f32> = ent.load(FieldAddrVector::Angles)?.into();
            let angle_vel: Vector3<f32> = ent.load(FieldAddrVector::AngularVelocity)?.into();
            let new_angles = angles + frame_time_f * angle_vel;
            ent.store(FieldAddrVector::Angles, new_angles.into())?;

            let orig: Vector3<f32> = ent.load(FieldAddrVector::Origin)?.into();
            let vel: Vector3<f32> = ent.load(FieldAddrVector::Velocity)?.into();
            let new_orig = orig + frame_time_f * vel;
            ent.store(FieldAddrVector::Origin, new_orig.into())?;
        }

        Ok(())
    }

    pub fn physics_step(
        &mut self,
        ent_id: EntityId,
        frame_time: Duration,
    ) -> Result<(), ProgsError> {
        let in_freefall = !self
            .world
            .entity(ent_id)
            .flags()?
            .intersects(EntityFlags::ON_GROUND | EntityFlags::FLY | EntityFlags::IN_WATER);

        if in_freefall {
            let sv_gravity = self.cvars.borrow().get_value("sv_gravity").unwrap();
            let vel: Vector3<f32> = self
                .world
                .entity(ent_id)
                .load(FieldAddrVector::Velocity)?
                .into();

            // If true, play an impact sound when the entity hits the ground.
            let hit_sound = vel.z < -0.1 * sv_gravity;

            self.world
                .entity_mut(ent_id)?
                .apply_gravity(sv_gravity, frame_time)?;

            let sv_maxvelocity = self.cvars.borrow().get_value("sv_maxvelocity").unwrap();
            self.world
                .entity_mut(ent_id)?
                .limit_velocity(sv_maxvelocity)?;

            // Move the entity and relink it.
            self.move_ballistic(frame_time, ent_id)?;
            self.link_entity(ent_id, true)?;

            let ent = self.world.entity_mut(ent_id)?;

            if ent.flags()?.contains(EntityFlags::ON_GROUND) && hit_sound {
                // Entity hit the ground this frame.
                todo!("SV_StartSound(demon/dland2.wav)");
            }
        }

        self.think(ent_id, frame_time)?;

        todo!("SV_CheckWaterTransition");

        Ok(())
    }

    pub fn move_push(
        &mut self,
        ent_id: EntityId,
        frame_time: Duration,
        move_time: Duration,
    ) -> Result<(), ProgsError> {
        let ent = self.world.entity_mut(ent_id)?;

        let vel: Vector3<f32> = ent.load(FieldAddrVector::Velocity)?.into();
        if vel.is_zero() {
            // Entity doesn't need to move.
            let local_time = ent.load(FieldAddrFloat::LocalTime)?;
            let new_local_time = local_time + duration_to_f32(move_time);
            ent.store(FieldAddrFloat::LocalTime, new_local_time)?;
            return Ok(());
        }

        let move_time_f = duration_to_f32(move_time);
        let move_vector = vel * move_time_f;
        // TODO let mins =
        todo!()
    }

    const MAX_BALLISTIC_COLLISIONS: usize = 4;

    /// Movement function for freefalling entities.
    pub fn move_ballistic(
        &mut self,
        sim_time: Duration,
        ent_id: EntityId,
    ) -> Result<(CollisionFlags, Option<Trace>), ProgsError> {
        let mut sim_time_f = duration_to_f32(sim_time);

        let mut out_trace = None;
        let mut flags = CollisionFlags::empty();
        let mut touching_planes: ArrayVec<Hyperplane, 5> = ArrayVec::new();

        let init_velocity = self.world.entity(ent_id).velocity()?;
        let mut trace_velocity = init_velocity;

        // Even when the entity collides with something along its path, it may
        // continue moving. This may occur when bouncing or sliding off a solid
        // object, or when moving between media (e.g. from air to water).
        for _ in 0..Self::MAX_BALLISTIC_COLLISIONS {
            let velocity = self.world.entity(ent_id).velocity()?;

            if velocity.is_zero() {
                // Not moving.
                break;
            }

            let orig = self.world.entity(ent_id).origin()?;
            let end = orig + sim_time_f * velocity;
            let min = self.world.entity(ent_id).min()?;
            let max = self.world.entity(ent_id).max()?;

            let (trace, hit_entity) =
                self.world
                    .move_entity(ent_id, orig, min, max, end, CollideKind::Normal)?;

            if trace.all_solid() {
                // Entity is stuck in a wall.
                self.world
                    .entity_mut(ent_id)?
                    .store(FieldAddrVector::Velocity, Vector3::zero().into())?;

                return Ok((CollisionFlags::HORIZONTAL | CollisionFlags::VERTICAL, None));
            }

            if trace.ratio() > 0.0 {
                // If the entity moved at all, update its position.
                self.world
                    .entity_mut(ent_id)?
                    .store(FieldAddrVector::Origin, trace.end_point().into())?;
                touching_planes.clear();

                trace_velocity = self.world.entity(ent_id).velocity()?;
            }

            // Find the plane the entity hit, if any.
            let boundary = match trace.end().kind() {
                // Entity didn't hit anything.
                TraceEndKind::Terminal => break,

                TraceEndKind::Boundary(b) => b,
            };

            // Sanity check to make sure the trace actually hit something.
            let hit_entity = match hit_entity {
                Some(h) => h,
                None => panic!("trace collided with nothing"),
            };

            // TODO: magic constant
            if boundary.plane.normal().z > 0.7 {
                flags |= CollisionFlags::HORIZONTAL;
                if self.world.entity(hit_entity).solid()? == EntitySolid::Bsp {
                    self.world
                        .entity_mut(ent_id)?
                        .add_flags(EntityFlags::ON_GROUND)?;
                    self.world
                        .entity_mut(ent_id)?
                        .store(FieldAddrEntityId::Ground, hit_entity)?;
                }
            } else if boundary.plane.normal().z == 0.0 {
                flags |= CollisionFlags::VERTICAL;
                out_trace = Some(trace.clone());
            }

            self.impact_entities(ent_id, hit_entity)?;
            if !self.world.entity_exists(ent_id) {
                // Entity removed by touch function.
                break;
            }

            sim_time_f -= trace.ratio() * sim_time_f;

            if touching_planes.try_push(boundary.plane.clone()).is_err() {
                // Touching too many planes to make much sense of, so stop.
                self.world
                    .entity_mut(ent_id)?
                    .store(FieldAddrVector::Velocity, Vector3::zero().into())?;
                return Ok((CollisionFlags::HORIZONTAL | CollisionFlags::VERTICAL, None));
            }

            let end_velocity =
                match phys::velocity_after_multi_collision(trace_velocity, &touching_planes, 1.0) {
                    Some(v) => v,
                    None => {
                        // Entity is wedged in a corner, so it simply stops.
                        self.world
                            .entity_mut(ent_id)?
                            .store(FieldAddrVector::Velocity, Vector3::zero().into())?;

                        return Ok((
                            CollisionFlags::HORIZONTAL
                                | CollisionFlags::VERTICAL
                                | CollisionFlags::STOPPED,
                            None,
                        ));
                    }
                };

            if init_velocity.dot(end_velocity) <= 0.0 {
                // Avoid bouncing the entity at a sharp angle.
                self.world
                    .entity_mut(ent_id)?
                    .store(FieldAddrVector::Velocity, Vector3::zero().into())?;
                return Ok((flags, out_trace));
            }

            self.world
                .entity_mut(ent_id)?
                .store(FieldAddrVector::Velocity, end_velocity.into())?;
        }

        Ok((flags, out_trace))
    }

    const DROP_TO_FLOOR_DIST: f32 = 256.0;

    /// Moves an entity straight down until it collides with a solid surface.
    ///
    /// Returns `true` if the entity hit the floor, `false` otherwise.
    ///
    /// ## Notes
    /// - The drop distance is limited to 256, so entities which are more than 256 units above a
    ///   solid surface will not actually hit the ground.
    pub fn drop_entity_to_floor(&mut self, ent_id: EntityId) -> Result<bool, ProgsError> {
        debug!("Finding floor for entity with ID {}", ent_id.0);
        let origin = self.world.entity(ent_id).origin()?;

        let end = Vector3::new(origin.x, origin.y, origin.z - Self::DROP_TO_FLOOR_DIST);
        let min = self.world.entity(ent_id).min()?;
        let max = self.world.entity(ent_id).max()?;

        let (trace, collide_entity) =
            self.world
                .move_entity(ent_id, origin, min, max, end, CollideKind::Normal)?;
        debug!("End position after drop: {:?}", trace.end_point());

        let drop_dist = 256.0;
        let actual_dist = (trace.end_point() - origin).magnitude();

        if collide_entity.is_none() || actual_dist == drop_dist || trace.all_solid() {
            // Entity didn't hit the floor or is stuck.
            Ok(false)
        } else {
            // Entity hit the floor. Update origin, relink and set ON_GROUND flag.
            self.world
                .entity_mut(ent_id)?
                .put_vector(trace.end_point().into(), FieldAddrVector::Origin as i16)?;
            self.link_entity(ent_id, false)?;
            self.world
                .entity_mut(ent_id)?
                .add_flags(EntityFlags::ON_GROUND)?;
            self.world
                .entity_mut(ent_id)?
                .put_entity_id(collide_entity.unwrap(), FieldAddrEntityId::Ground as i16)?;

            Ok(true)
        }
    }

    pub fn touch_triggers(&mut self, ent_id: EntityId) -> Result<(), ProgsError> {
        // TODO: alloc once
        let mut touched = Vec::new();
        self.world.list_touched_triggers(&mut touched, ent_id, 0)?;

        // Save state.
        let restore_self = self.globals.load(GlobalAddrEntity::Self_)?;
        let restore_other = self.globals.load(GlobalAddrEntity::Other)?;

        // Activate the touched triggers.
        for trigger_id in touched {
            let trigger_touch = self
                .world
                .entity(trigger_id)
                .load(FieldAddrFunctionId::Touch)?;

            self.globals.store(GlobalAddrEntity::Self_, trigger_id)?;
            self.globals.store(GlobalAddrEntity::Other, ent_id)?;
            self.execute_program(trigger_touch)?;
        }

        // Restore state.
        self.globals.store(GlobalAddrEntity::Self_, restore_self)?;
        self.globals.store(GlobalAddrEntity::Other, restore_other)?;

        Ok(())
    }

    /// Runs two entities' touch functions.
    pub fn impact_entities(&mut self, ent_a: EntityId, ent_b: EntityId) -> Result<(), ProgsError> {
        let restore_self = self.globals.load(GlobalAddrEntity::Self_)?;
        let restore_other = self.globals.load(GlobalAddrEntity::Other)?;

        self.globals
            .store(GlobalAddrFloat::Time, duration_to_f32(self.time))?;

        // Set up and run Entity A's touch function.
        let touch_a = self.world.entity(ent_a).load(FieldAddrFunctionId::Touch)?;
        let solid_a = self.world.entity(ent_a).solid()?;
        if touch_a.0 != 0 && solid_a != EntitySolid::Not {
            self.globals.store(GlobalAddrEntity::Self_, ent_a)?;
            self.globals.store(GlobalAddrEntity::Other, ent_b)?;
            self.execute_program(touch_a)?;
        }

        // Set up and run Entity B's touch function.
        let touch_b = self.world.entity(ent_b).load(FieldAddrFunctionId::Touch)?;
        let solid_b = self.world.entity(ent_b).solid()?;
        if touch_b.0 != 0 && solid_b != EntitySolid::Not {
            self.globals.store(GlobalAddrEntity::Self_, ent_b)?;
            self.globals.store(GlobalAddrEntity::Other, ent_a)?;
            self.execute_program(touch_b)?;
        }

        self.globals.store(GlobalAddrEntity::Self_, restore_self)?;
        self.globals.store(GlobalAddrEntity::Other, restore_other)?;

        Ok(())
    }
*/





    // QuakeC instructions ====================================================
/* 
    pub fn op_return(&mut self, a: i16, b: i16, c: i16) -> Result<(), ProgsError> {
        let val1 = self.globals.get_bytes(a)?;
        let val2 = self.globals.get_bytes(b)?;
        let val3 = self.globals.get_bytes(c)?;

        self.globals.put_bytes(val1, GLOBAL_ADDR_RETURN as i16)?;
        self.globals
            .put_bytes(val2, GLOBAL_ADDR_RETURN as i16 + 1)?;
        self.globals
            .put_bytes(val3, GLOBAL_ADDR_RETURN as i16 + 2)?;

        self.cx.leave_function(&mut self.globals)?;

        Ok(())
    }

    // LOAD_F: load float field from entity
    pub fn op_load_f(&mut self, e_ofs: i16, e_f: i16, dest_ofs: i16) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(e_ofs)?;

        let fld_ofs = self.globals.get_field_addr(e_f)?;

        let f = self.world.entity(ent_id).get_float(fld_ofs.0 as i16)?;
        self.globals.put_float(f, dest_ofs)?;

        Ok(())
    }

    // LOAD_V: load vector field from entity
    pub fn op_load_v(
        &mut self,
        ent_id_addr: i16,
        ent_vector_addr: i16,
        dest_addr: i16,
    ) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(ent_id_addr)?;
        let ent_vector = self.globals.get_field_addr(ent_vector_addr)?;
        let v = self.world.entity(ent_id).get_vector(ent_vector.0 as i16)?;
        self.globals.put_vector(v, dest_addr)?;

        Ok(())
    }

    pub fn op_load_s(
        &mut self,
        ent_id_addr: i16,
        ent_string_id_addr: i16,
        dest_addr: i16,
    ) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(ent_id_addr)?;
        let ent_string_id = self.globals.get_field_addr(ent_string_id_addr)?;
        let s = self
            .world
            .entity(ent_id)
            .string_id(ent_string_id.0 as i16)?;
        self.globals.put_string_id(s, dest_addr)?;

        Ok(())
    }

    pub fn op_load_ent(
        &mut self,
        ent_id_addr: i16,
        ent_entity_id_addr: i16,
        dest_addr: i16,
    ) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(ent_id_addr)?;
        let ent_entity_id = self.globals.get_field_addr(ent_entity_id_addr)?;
        let e = self
            .world
            .entity(ent_id)
            .entity_id(ent_entity_id.0 as i16)?;
        self.globals.put_entity_id(e, dest_addr)?;

        Ok(())
    }

    pub fn op_load_fnc(
        &mut self,
        ent_id_addr: i16,
        ent_function_id_addr: i16,
        dest_addr: i16,
    ) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(ent_id_addr)?;
        let fnc_function_id = self.globals.get_field_addr(ent_function_id_addr)?;
        let f = self
            .world
            .entity(ent_id)
            .function_id(fnc_function_id.0 as i16)?;
        self.globals.put_function_id(f, dest_addr)?;

        Ok(())
    }

    pub fn op_address(
        &mut self,
        ent_id_addr: i16,
        fld_addr_addr: i16,
        dest_addr: i16,
    ) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(ent_id_addr)?;
        let fld_addr = self.globals.get_field_addr(fld_addr_addr)?;
        self.globals.put_entity_field(
            self.world.ent_fld_addr_to_i32(EntityFieldAddr {
                entity_id: ent_id,
                field_addr: fld_addr,
            }),
            dest_addr,
        )?;

        Ok(())
    }

    pub fn op_storep_f(
        &mut self,
        src_float_addr: i16,
        dst_ent_fld_addr: i16,
        unused: i16,
    ) -> Result<(), ProgsError> {
        if unused != 0 {
            return Err(ProgsError::with_msg("storep_f: nonzero arg3"));
        }

        let f = self.globals.get_float(src_float_addr)?;
        let ent_fld_addr = self
            .world
            .ent_fld_addr_from_i32(self.globals.get_entity_field(dst_ent_fld_addr)?);
        self.world
            .entity_mut(ent_fld_addr.entity_id)?
            .put_float(f, ent_fld_addr.field_addr.0 as i16)?;

        Ok(())
    }

    pub fn op_storep_v(
        &mut self,
        src_vector_addr: i16,
        dst_ent_fld_addr: i16,
        unused: i16,
    ) -> Result<(), ProgsError> {
        if unused != 0 {
            return Err(ProgsError::with_msg("storep_v: nonzero arg3"));
        }

        let v = self.globals.get_vector(src_vector_addr)?;
        let ent_fld_addr = self
            .world
            .ent_fld_addr_from_i32(self.globals.get_entity_field(dst_ent_fld_addr)?);
        self.world
            .entity_mut(ent_fld_addr.entity_id)?
            .put_vector(v, ent_fld_addr.field_addr.0 as i16)?;

        Ok(())
    }

    pub fn op_storep_s(
        &mut self,
        src_string_id_addr: i16,
        dst_ent_fld_addr: i16,
        unused: i16,
    ) -> Result<(), ProgsError> {
        if unused != 0 {
            return Err(ProgsError::with_msg("storep_s: nonzero arg3"));
        }

        let s = self.globals.string_id(src_string_id_addr)?;
        let ent_fld_addr = self
            .world
            .ent_fld_addr_from_i32(self.globals.get_entity_field(dst_ent_fld_addr)?);
        self.world
            .entity_mut(ent_fld_addr.entity_id)?
            .put_string_id(s, ent_fld_addr.field_addr.0 as i16)?;

        Ok(())
    }

    pub fn op_storep_ent(
        &mut self,
        src_entity_id_addr: i16,
        dst_ent_fld_addr: i16,
        unused: i16,
    ) -> Result<(), ProgsError> {
        if unused != 0 {
            return Err(ProgsError::with_msg("storep_ent: nonzero arg3"));
        }

        let e = self.globals.entity_id(src_entity_id_addr)?;
        let ent_fld_addr = self
            .world
            .ent_fld_addr_from_i32(self.globals.get_entity_field(dst_ent_fld_addr)?);
        self.world
            .entity_mut(ent_fld_addr.entity_id)?
            .put_entity_id(e, ent_fld_addr.field_addr.0 as i16)?;

        Ok(())
    }

    pub fn op_storep_fnc(
        &mut self,
        src_function_id_addr: i16,
        dst_ent_fld_addr: i16,
        unused: i16,
    ) -> Result<(), ProgsError> {
        if unused != 0 {
            return Err(ProgsError::with_msg("storep_fnc: nonzero arg3"));
        }

        let f = self.globals.function_id(src_function_id_addr)?;
        let ent_fld_addr = self
            .world
            .ent_fld_addr_from_i32(self.globals.get_entity_field(dst_ent_fld_addr)?);
        self.world
            .entity_mut(ent_fld_addr.entity_id)?
            .put_function_id(f, ent_fld_addr.field_addr.0 as i16)?;

        Ok(())
    }

    pub fn op_state(
        &mut self,
        frame_id_addr: i16,
        unused_b: i16,
        unused_c: i16,
    ) -> Result<(), ProgsError> {
        if unused_b != 0 {
            return Err(ProgsError::with_msg("storep_fnc: nonzero arg2"));
        } else if unused_c != 0 {
            return Err(ProgsError::with_msg("storep_fnc: nonzero arg3"));
        }

        let self_id = self.globals.entity_id(GlobalAddrEntity::Self_ as i16)?;
        let self_ent = self.world.entity_mut(self_id)?;
        let next_think_time = self.globals.get_float(GlobalAddrFloat::Time as i16)? + 0.1;

        self_ent.put_float(next_think_time, FieldAddrFloat::NextThink as i16)?;

        let frame_id = self.globals.get_float(frame_id_addr)?;
        self_ent.put_float(frame_id, FieldAddrFloat::FrameId as i16)?;

        Ok(())
    }

    // QuakeC built-in functions ==============================================

    pub fn builtin_set_origin(&mut self) -> Result<(), ProgsError> {
        let e_id = self.globals.entity_id(GLOBAL_ADDR_ARG_0 as i16)?;
        let origin = self.globals.get_vector(GLOBAL_ADDR_ARG_1 as i16)?;
        self.set_entity_origin(e_id, Vector3::from(origin))?;

        Ok(())
    }

    pub fn builtin_set_model(&mut self) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(GLOBAL_ADDR_ARG_0 as i16)?;
        let model_name_id = self.globals.string_id(GLOBAL_ADDR_ARG_1 as i16)?;
        self.set_entity_model(ent_id, model_name_id)?;

        Ok(())
    }

    pub fn builtin_set_size(&mut self) -> Result<(), ProgsError> {
        let e_id = self.globals.entity_id(GLOBAL_ADDR_ARG_0 as i16)?;
        let mins = self.globals.get_vector(GLOBAL_ADDR_ARG_1 as i16)?;
        let maxs = self.globals.get_vector(GLOBAL_ADDR_ARG_2 as i16)?;
        self.world.set_entity_size(e_id, mins.into(), maxs.into())?;

        Ok(())
    }

    // TODO: move to Globals
    pub fn builtin_random(&mut self) -> Result<(), ProgsError> {
        self.globals
            .put_float(rand::random(), GLOBAL_ADDR_RETURN as i16)?;

        Ok(())
    }

    pub fn builtin_spawn(&mut self) -> Result<(), ProgsError> {
        let ent_id = self.spawn_entity()?;
        self.globals
            .put_entity_id(ent_id, GLOBAL_ADDR_RETURN as i16)?;

        Ok(())
    }

    pub fn builtin_remove(&mut self) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(GLOBAL_ADDR_ARG_0 as i16)?;
        self.world.remove_entity(ent_id)?;

        Ok(())
    }

   

    pub fn builtin_dprint(&mut self) -> Result<(), ProgsError> {
        let strs = self.string_table.borrow();
        let s_id = self.globals.string_id(GLOBAL_ADDR_ARG_0 as i16)?;
        let string = strs.get(s_id).unwrap();
        debug!("DPRINT: {}", string);

        Ok(())
    }

    pub fn builtin_drop_to_floor(&mut self) -> Result<(), ProgsError> {
        let ent_id = self.globals.entity_id(GlobalAddrEntity::Self_ as i16)?;
        let hit_floor = self.drop_entity_to_floor(ent_id)?;
        self.globals
            .put_float(hit_floor as u32 as f32, GLOBAL_ADDR_RETURN as i16)?;

        Ok(())
    }

    pub fn builtin_light_style(&mut self) -> Result<(), ProgsError> {
        let index = match self.globals.get_float(GLOBAL_ADDR_ARG_0 as i16)? as i32 {
            i if i < 0 => return Err(ProgsError::with_msg("negative lightstyle ID")),
            i => i as usize,
        };
        let val = self.globals.string_id(GLOBAL_ADDR_ARG_1 as i16)?;
        self.set_lightstyle(index, val);

        Ok(())
    }

    pub fn builtin_cvar(&mut self) -> Result<(), ProgsError> {
        let s_id = self.globals.string_id(GLOBAL_ADDR_ARG_0 as i16)?;
        let strs = self.string_table.borrow();
        let s = strs.get(s_id).unwrap();
        let f = self.cvars.borrow().get_value(s).unwrap();
        self.globals.put_float(f, GLOBAL_ADDR_RETURN as i16)?;

        Ok(())
    }

    pub fn builtin_cvar_set(&mut self) -> Result<(), ProgsError> {
        let strs = self.string_table.borrow();

        let var_id = self.globals.string_id(GLOBAL_ADDR_ARG_0 as i16)?;
        let var = strs.get(var_id).unwrap();
        let val_id = self.globals.string_id(GLOBAL_ADDR_ARG_1 as i16)?;
        let val = strs.get(val_id).unwrap();

        self.cvars.borrow_mut().set(var, val).unwrap();

        Ok(())
    }

    pub fn builtin_ambient_sound(&mut self) -> Result<(), ProgsError> {
        let _pos = self.globals.get_vector(GLOBAL_ADDR_ARG_0 as i16)?;
        let name = self.globals.string_id(GLOBAL_ADDR_ARG_1 as i16)?;
        let _volume = self.globals.get_float(GLOBAL_ADDR_ARG_2 as i16)?;
        let _attenuation = self.globals.get_float(GLOBAL_ADDR_ARG_3 as i16)?;

        let _sound_index = match self.sound_id(name) {
            Some(i) => i,
            None => return Err(ProgsError::with_msg("sound not precached")),
        };

        // TODO: write to server signon packet
        Ok(())
    }

    */
}
