
use toml::Value;
use serde_derive::{Deserialize,Serialize};

use std::collections::HashMap; 

use crate::server::Vfs;
use crate::server::slime::SlimeError;

#[derive(Debug, Deserialize, Serialize)]
pub struct SlimeManifest {
    name: String,
    version:String,
    cvars: Vec<String>,
    entity_files: Vec<String>,
    component_files: Vec<String>
 }



 #[derive(Debug )]
 pub struct SlimeContext {
    manifest: SlimeManifest,
    entity_slimes: HashMap<String,EntitySlime>,
 }



 #[derive(Debug, Deserialize, Serialize)]
 pub struct EntitySlimeEntry {
    classname: String,
    scripts: HashMap<String, Vec< SlimeFuncEntry >>
   // fn_prepare: Vec< SlimeFuncEntry >,
   // fn_spawn: Vec< SlimeFunc >
 }


 #[derive(Debug, Deserialize, Serialize)]
 pub struct SlimeFuncEntry {
    call: String,
    conditions: Option<Vec< HashMap<String, String> >>,
    inputs:  Option<HashMap<String, String>>
 }


 #[derive(Debug )]
pub struct EntitySlime {
    classname: String,
    scripts: HashMap<String, Vec< SlimeFunc >>

    //fn_prepare: Vec< SlimeFunc >,
   // fn_spawn: Vec< SlimeFunc >
 }

 
 #[derive(Debug )]
 pub enum SlimeFunc {
    PrecacheModel { name:String },

    PrecacheSound { name:String }

 }


impl SlimeContext {
    pub fn new( vfs: &Vfs, slime_file_name:&str ) ->  Result<SlimeContext,SlimeError> {

            
        //could i also try to load a special custom progs dat file that i design myself ?
        let mut manifest_virtual_file = vfs.open( slime_file_name ).or_else(
            |error| { Err(SlimeError::SlimeLoadingError(format!("Error opening manifest file {}", slime_file_name) )) }
        )?;
        
        
 
        let manifest_contents = Vfs::read_to_end( manifest_virtual_file  ).or_else(
            |error| { Err(SlimeError::SlimeLoadingError(format!("Error reading manifest file"))) }
        )?;
     
        println!("reading {}",manifest_contents);
        
        let manifest:SlimeManifest = toml::from_str(  &manifest_contents )?;


        let mut entity_slimes:HashMap<String,EntitySlime> = HashMap::new();

        for entity_file in manifest.entity_files.iter(){         

            let entity_slime_entry:EntitySlimeEntry = SlimeContext::read_parse_toml(vfs, entity_file )?;

            let entity_slime:EntitySlime = SlimeContext::build_entity_slime( entity_slime_entry )? ;

            entity_slimes.insert(entity_slime.classname.to_string(), entity_slime);
        }

       

        return Ok(SlimeContext {  

            manifest,
            entity_slimes

        })
    }


    pub fn read_parse_toml<'a,T >( vfs: &Vfs , file_name: &String  )  -> Result<T,SlimeError>
    where
       for<'de> T: serde::Deserialize<'de> + 'a
     {

        let mut file = vfs.open( file_name ).or_else(
            |error| { Err(SlimeError::SlimeLoadingError(format!("Error opening file {}", file_name) )) }
        )?;

        let contents = Vfs::read_to_end( file  ).or_else(
            |error| { Err(SlimeError::SlimeLoadingError(format!("Error reading file {}", file_name) )) }
        )?;

        let contents_ref = &contents;

        let output:T = toml::from_str( contents_ref  ).or_else(
            |error| { 
                Err(SlimeError::SlimeLoadingError(format!("Error parsing file {} - {}", file_name, error) )) }
        )?;

        Ok(output)

    }

    pub fn build_entity_slime( entry:EntitySlimeEntry ) -> Result<EntitySlime,SlimeError> {


        let scripts_output:HashMap<String, Vec< SlimeFunc >> = HashMap::new();

        for (key, entry) in map.into_iter() {

            let slime_funcs = entry.into_iter().map( |x:SlimeFuncEntry| SlimeContext::build_slime_func( x ).unwrap()  ).collect::<Vec<SlimeFunc>>() ;
            scripts_output.insert(key, slime_funcs  );
     
        };


        Ok(EntitySlime {
            classname: entry.classname,
            scripts: scripts_output

        })
    }

    pub fn build_slime_func( func_entry:SlimeFuncEntry ) -> Result<SlimeFunc,SlimeError> {

        match func_entry.call.as_str() {

            "precache_model" => {
                Ok( SlimeFunc::PrecacheModel { name: func_entry.inputs.unwrap().get("name").unwrap().to_string()  }   )
            }

            "precache_sound" => {
                Ok( SlimeFunc::PrecacheSound { name: func_entry.inputs.unwrap().get("name").unwrap().to_string()  }   )
            }

            _ => Err( SlimeError::SlimeLoadingError(format!("Could not parse slime func {}", func_entry.call))  )

        }
 
    }



    //research type parameters ! s

    //returns the method type and the inputs for the call 
    pub fn fetch_subroutines(&self, classname: &str, methodname: &str) -> Option<Vec<SlimeFunc>>{

        let entity_slime = self.entity_slimes.get(classname)


        
    }

}