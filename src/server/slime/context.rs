
use toml::Value;
use serde_derive::Deserialize;

 



#[derive(Debug, Deserialize)]
pub struct SlimeContext {
    name: String,
 }

impl SlimeContext {
    pub fn new() -> SlimeContext {
        let context:SlimeContext =  toml::from_str(r#"
                name = 'testt'
 
            "#).unwrap();

        assert_eq!(context.name, "testt");

        return context
    }


    //research type parameters ! s

    //returns the method type and the inputs for the call 
    pub fn fetch_subroutines_for_function(&self, classname: &str, methodname: &str) {


    }

}