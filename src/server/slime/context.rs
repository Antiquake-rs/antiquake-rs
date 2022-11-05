
use toml::Value;
use serde_derive::{Deserialize,Serialize};

 



#[derive(Debug, Deserialize, Serialize)]
pub struct SlimeManifest {
    name: String,
 }

 #[derive(Debug, Deserialize, Serialize)]
 pub struct SlimeContext {
    manifest: SlimeManifest,
 }


impl SlimeContext {
    pub fn new() -> SlimeContext {
        let manifest:SlimeManifest =  toml::from_str(r#"
                name = 'testt'
 
            "#).unwrap();

        assert_eq!(manifest.name, "testt");

        return SlimeContext {  

            manifest

        }
    }


    //research type parameters ! s

    //returns the method type and the inputs for the call 
    pub fn fetch_subroutines_for_function(&self, classname: &str, methodname: &str) {


    }

}