use std::{ops::Range, collections::HashMap};

use arrayvec::{ArrayString, ArrayVec};

/// Maximum permitted length of a precache path.
pub const MAX_PRECACHE_PATH: usize = 64;

pub const MAX_PRECACHE_ENTRIES: usize = 256;

/// A list of resources to be loaded before entering the game.
///
/// This is used by the server to inform clients which resources (sounds and
/// models) they should load before joining. It also serves as the canonical
/// mapping of resource IDs for a given level.
// TODO: ideally, this is parameterized by the maximum number of entries, but
// it's not currently possible to do { MAX_PRECACHE_PATH * N } where N is a
// const generic parameter. In practice both models and sounds have a maximum
// value of 256.
#[derive(Debug)]
pub struct Precache {

   
    items: Vec<String>,
    reverse_id_map: HashMap<String,usize>,

    //str_data: ArrayString<{ MAX_PRECACHE_PATH * MAX_PRECACHE_ENTRIES }>,
    //items: ArrayVec<Range<usize>, MAX_PRECACHE_ENTRIES>,
}

impl Precache {
    /// Creates a new empty `Precache`.
    pub fn new() -> Precache {
        Precache {
            items: Vec::new(),
            reverse_id_map: HashMap::new()            
        }
    }

    /// Retrieves an item from the precache if the item exists.
    /// modelId => modelName
    pub fn get(&self, index: usize) -> Option<&str> {
        
        Some(&self.items.get(index)?.as_str())
    }

    /// Returns the index of the target value if it exists.
    pub fn find<S>(&self, target: S) -> Option<usize>
    where
        S: AsRef<str>,
    {
        //the hashmap speeds this up ! no need to iter
        Some(*self.reverse_id_map.get(target.as_ref())?)
    }
 
   pub fn get_data(&self) -> Vec<String> { 
        return self.items.clone()   //.into_iter().collect() 
    } 


    /// Adds an item to the precache.
    ///
    /// If the item already exists in the precache, this has no effect.
    pub fn precache<S>(&mut self, item: S) -> usize
    where
        S: AsRef<str>,
    {
        let item = item.as_ref();

        if item.len() > MAX_PRECACHE_PATH {
            panic!(
                "precache name (\"{}\") too long: max length is {}",
                item, MAX_PRECACHE_PATH
            );
        }

       
      /*  let start = self.str_data.len();
        self.str_data.push_str(item);
        let end = self.str_data.len();

        self.items.push(start..end);*/ 

        match self.find(item)  {
            Some (i) => return i ,  
            None => {


                let item_id =  self.items.len();

                self.reverse_id_map.insert(  item.to_string() , item_id.clone() );
                self.items.push(item.to_string());
                
        
                return item_id.clone()
        
            }
            
        }



    }

    // Returns an iterator over the values in the precache.
  
  /*  pub fn iter( self) -> impl Iterator<Item=String> + 'static {
        self.items
            .iter()
           // . map( | item | item  )
            
           // .cloned()
           // . map( | item | item.as_str() )
    } */


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precache_one() {
        let mut p = Precache::new();

        p.precache("hello");
        assert_eq!(Some("hello"), p.get(0));
    }

    #[test]
    fn test_precache_several() {
        let mut p = Precache::new();

        let items = &["Quake", "is", "a", "1996", "first-person", "shooter"];

        for item in items {
            p.precache(item);
        }

        // Pick an element in the middle
        assert_eq!(Some("first-person"), p.get(4));

        // Check all the elements
       // for (precached, &original) in p.iter().zip(items.iter()) {
         //   assert_eq!(precached, original);
       //}
    }
}
