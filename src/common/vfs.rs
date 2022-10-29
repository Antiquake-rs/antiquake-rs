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



use std::{
    fs::{self,File,DirEntry},
    io::{self, BufReader, Cursor, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
   
};

use crate::common::pak::{Pak, PakError};


use thiserror::Error;

#[derive(Error, Debug)]
pub enum VfsError {
    #[error("Couldn't load pakfile: {0}")]
    Pak(#[from] PakError),
    #[error("File does not exist: {0}")]
    NoSuchFile(String),
}



pub enum PakExtType {
    PakType,
    Pk3Type 
}

 

#[derive(Debug)]
enum VfsComponent {
    Pak(Pak),
    Directory(PathBuf),
}

#[derive(Debug)]
pub struct Vfs {
    components: Vec<VfsComponent>,
}

 

impl Vfs {
    pub fn new() -> Vfs {
        Vfs {
            components: Vec::new(),
        }
    }

    /// Initializes the virtual filesystem using a base directory.
    pub fn with_base_dir(base_dir: PathBuf) -> Vfs {
        let mut vfs = Vfs::new();

        let mut game_dir = base_dir;
        game_dir.push("id1");

        if !game_dir.is_dir() {
            log::error!(concat!(
                "`id1/` directory does not exist! Use the `--base-dir` option with the name of the",
                " directory which contains `id1/`."
            ));

            std::process::exit(1);
        }

        vfs.add_directory(&game_dir).unwrap();

        let subfiles = fs::read_dir(game_dir).unwrap();


        let mut num_paks = 0;


        for entryResult in subfiles {


            match(entryResult){
                Ok(entry) => {
                    let addResult = Self::try_add_as_pakfile( &mut vfs,  entry );

                    match addResult {
                        Ok(added) => {
                            num_paks += 1;
                        }
                        Err(_) => {continue;}
                    }
                }
                Err(_) => {
                    continue;
                }
            }

           

          
        }


      

        if num_paks == 0 {
            log::warn!("No PAK files found.");
        }

        info!("Found {} pakfiles.", num_paks);
        
        vfs
    }

    pub fn try_add_as_pakfile(vfs:&mut Vfs, entry:DirEntry) -> Result<(),VfsError>{

        let file_path = entry.path();
            
    
        let metadata = fs::metadata(&file_path);
      //  let last_modified = metadata?.modified()?.elapsed().as_secs();

        let file_path_string = file_path.display().to_string();
        let last_period_pos = file_path_string.rfind('.');

        match last_period_pos {
            Some(pos) => {

                let file_ext = &file_path_string[pos..];

                match file_ext.to_lowercase().as_str() {
                     ".pak" => {
                        vfs.add_pakfile(file_path, PakExtType::PakType).unwrap();
                        Ok(())
                    }
                    ".pk3" => {
                        vfs.add_pakfile(file_path, PakExtType::Pk3Type).unwrap();
                        Ok(())
                    }
                    default => {
                        info!("Could not add pak with extension {}", file_ext);
                        Ok(())
                    }
                } 

            }
            None => {
                info!("Could not add pak with path {}", file_path_string);
                Ok(())
              }
        }
         
    }

    pub fn add_pakfile<P>(&mut self, path: P, ext_type: PakExtType) -> Result<(), VfsError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        self.components.push(VfsComponent::Pak(Pak::new(path,PakExtType::PakType)?));
         

        Ok(())
    }

    pub fn add_directory<P>(&mut self, path: P) -> Result<(), VfsError>
    where
        P: AsRef<Path>,
    {
        self.components
            .push(VfsComponent::Directory(path.as_ref().to_path_buf()));
        Ok(())
    }

    pub fn open<S>(&self, virtual_path: S) -> Result<VirtualFile, VfsError>
    where
        S: AsRef<str>,
    {
        let vp = virtual_path.as_ref();

        // iterate in reverse so later PAKs overwrite earlier ones
        for c in self.components.iter().rev() {
            match c {
                VfsComponent::Pak(pak) => {
                    if let Ok(f) = pak.open(vp) {
                        return Ok(VirtualFile::PakBacked(Cursor::new(f)));
                    }
                }

                VfsComponent::Directory(path) => {
                    let mut full_path = path.to_owned();
                    full_path.push(vp);

                    if let Ok(f) = File::open(full_path) {
                        return Ok(VirtualFile::FileBacked(BufReader::new(f)));
                    }
                }
            }
        }

        Err(VfsError::NoSuchFile(vp.to_owned()))
    }
}

pub enum VirtualFile<'a> {
    PakBacked(Cursor<&'a [u8]>),
    FileBacked(BufReader<File>),
}

impl<'a> Read for VirtualFile<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            VirtualFile::PakBacked(curs) => curs.read(buf),
            VirtualFile::FileBacked(file) => file.read(buf),
        }
    }
}

impl<'a> Seek for VirtualFile<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            VirtualFile::PakBacked(curs) => curs.seek(pos),
            VirtualFile::FileBacked(file) => file.seek(pos),
        }
    }
}
