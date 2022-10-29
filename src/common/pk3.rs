// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Quake PK3 archive manipulation.
//! 
//! 

use std::{
    collections::{hash_map::Iter, HashMap},
    fs,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};


use std::io::prelude::*;

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;
 

//import pak and pakerror 

mod pak; 

use pak::{Pak,PakError};

 
impl Pk3  {
    // overrides new to open a zip pak (slime)

    //https://docs.rs/zip/0.6.3/zip/
    pub fn new<P>(path: P) -> Result<Pak, PakError>
    where
        P: AsRef<Path>,
    {
        debug!("Opening {}", path.as_ref().to_str().unwrap());

        let mut infile = fs::File::open(path).unwrap();
       
        let mut archive = zip::ZipArchive::new(file).unwrap();


        let file_names = archive.file_names();



        let mut map = HashMap::new();


        /*
            Somehow turn the files into the hashmap 
         -- may need to do kind of what slade does 

            Need to load files into hashmap 
        */

       /*let mut file = match archive.by_name("test/lorem_ipsum.txt") {
            Ok(file) => file,
            Err(..) => {
                println!("File test/lorem_ipsum.txt not found");
                return 2;
            }
        };
    
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);





        // Locate the file table
        let table_offset = match infile.read_i32::<LittleEndian>()? {
            o if o <= 0 => Err(PakError::InvalidTableOffset(o))?,
            o => o as u32,
        };

        let table_size = match infile.read_i32::<LittleEndian>()? {
            s if s <= 0 || s as usize % PAK_ENTRY_SIZE != 0 => Err(PakError::InvalidTableSize(s))?,
            s => s as u32,
        };

       

        for i in 0..(table_size as usize / PAK_ENTRY_SIZE) {
            let entry_offset = table_offset as u64 + (i * PAK_ENTRY_SIZE) as u64;
            infile.seek(SeekFrom::Start(entry_offset))?;

            let mut path_bytes = [0u8; 56];
            infile.read(&mut path_bytes)?;

            let file_offset = match infile.read_i32::<LittleEndian>()? {
                o if o <= 0 => Err(PakError::InvalidFileOffset(o))?,
                o => o as u32,
            };

            let file_size = match infile.read_i32::<LittleEndian>()? {
                s if s <= 0 => Err(PakError::InvalidFileSize(s))?,
                s => s as u32,
            };

            let last = path_bytes
                .iter()
                .position(|b| *b == 0)
                .ok_or(PakError::FileNameTooLong(
                    String::from_utf8_lossy(&path_bytes).into_owned(),
                ))?;
            let path = String::from_utf8(path_bytes[0..last].to_vec())?;
            infile.seek(SeekFrom::Start(file_offset as u64))?;

            let mut data: Vec<u8> = Vec::with_capacity(file_size as usize);
            (&mut infile)
                .take(file_size as u64)
                .read_to_end(&mut data)?;

            map.insert(path, data.into_boxed_slice());
        }

         */ 


        Ok(Pak(map))
    }

   
    fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
        let mut zip = zip::ZipArchive::new(reader)?;

        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            println!("Filename: {}", file.name());
            std::io::copy(&mut file, &mut std::io::stdout());
        }

        Ok(())
    }


 
}
