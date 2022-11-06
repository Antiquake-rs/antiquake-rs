

pub mod context;
 
use toml;

use crate::{
  server::world::{EntityError, EntityTypeDef}, 
 
  server::progs::string_table::{StringTable},
  server::progs::globals::{Globals},
 
};

use crate::server::Vfs;

use std::{
  
  cell::{  RefCell},
  rc::Rc,
  fmt,

  error::Error,
};

pub use self::{
  context::{SlimeContext}
};





pub struct Slime {
  //  pub cx: ExecutionContext,
    pub slime_context: SlimeContext,
    //pub globals: Globals,  //need these anymore ??? 
    pub entity_def: Rc<EntityTypeDef>,
    pub string_table: Rc<RefCell<StringTable>>,
}

#[derive(Debug)]
pub enum SlimeError {
  Io(::std::io::Error),

  Toml( toml::de::Error),

  SlimeLoadingError(String),
  
  Entity(EntityError),
 
  Other(String),
}


impl fmt::Display for SlimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      use self::SlimeError::*;
      match *self {
          Io(ref err) => {
              write!(f, "I/O error: ")?;
              err.fmt(f)
          }

          Toml(ref err) => {
              write!(f, "Toml error: ")?;
              err.fmt(f)
          }
           
          Entity(ref err) => {
              write!(f, "Entity error: ")?;
              err.fmt(f)
          }
         
          SlimeLoadingError(ref msg) => write!(f, "{}", msg),

          Other(ref msg) => write!(f, "{}", msg),
      }
  }
}

impl Error for SlimeError {}

impl From<::std::io::Error> for SlimeError {
  fn from(error: ::std::io::Error) -> Self {
    SlimeError::Io(error)
  }
}

impl From<toml::de::Error> for SlimeError {
  fn from(error: toml::de::Error) -> Self {
    SlimeError::Toml(error)
  }
}
 

impl From<EntityError> for SlimeError {
  fn from(error: EntityError) -> Self {
    SlimeError::Entity(error)
  }
}


impl Slime{
  pub fn load(vfs: &Vfs, slime_file_name:&str) -> Result<Slime,SlimeError> {



    let slime_context = SlimeContext::new( vfs , slime_file_name  ) ?;

    

    //let manifest_contents = manifest_file.read();

    let mut strings = Vec::new();  
    let string_table = Rc::new(RefCell::new(StringTable::new(strings)));
    let mut field_defs = Vec::new(); //what are these 
    let mut globaldefs = Vec::new();
    let mut addrs = Vec::new();

    let ent_addr_count = 105; // is this right 

    //how do we populate ? 
    //parse w serde ?? 

 


    let entity_def = Rc::new(EntityTypeDef::new(
      string_table.clone(),
        ent_addr_count,
        field_defs.into_boxed_slice(),
    )?);


      let globals = Globals::new(
        string_table.clone(),
        globaldefs.into_boxed_slice(),
        addrs.into_boxed_slice(),
    );

    Ok(Slime{slime_context,entity_def,string_table})
  }
}