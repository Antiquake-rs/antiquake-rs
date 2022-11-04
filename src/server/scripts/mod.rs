
/*


        Using rlua !! 


*/


use rlua::{Function, Lua, MetaMethod, Result as LuaResult, UserData, UserDataMethods, Variadic};

use core::fmt::Error;

#[derive(Debug)]
pub struct ScriptingContext {
   /* string_table: Rc<RefCell<StringTable>>,
    functions: Rc<Functions>,
    pc: usize,
    current_function: FunctionId,
    call_stack: Vec<StackFrame>,
    local_stack: Vec<[u8; 4]>,*/ 
}

#[derive(Clone)]
struct VirtualLevel {
    name:String
}

impl ScriptingContext {
    pub fn new() -> ScriptingContext {
        ScriptingContext {} 
    }


  

    pub fn evaluate_build_level(&self) -> Result<(),Error>{

         
         
        // Create scripting engine
        let lua = Lua::new();


        //load vars and load files into the context 

 
      
         //  println!("script says {}", vlevel.name );
      

        Ok(())



    }

    

}

