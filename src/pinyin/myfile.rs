extern crate native;
extern crate rustrt;

/// Note: this file is a pure hack that will not be needed
/// when the std::io::File  will be able to work without
/// runtime

use self::native::io::file::open;
use self::rustrt::rtio;
use std::io::IoResult;
use std::io;
use std::io::standard_error;


pub struct MyFile {
    fd: Box<rtio::RtioFileStream + Send>,
    last_nread: int,
}

impl Writer for MyFile {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        match self.fd.write(buf) {
            Ok(_) => Ok(()),
            Err(_) =>  Err(standard_error(io::EndOfFile))
        }
    }
}

impl Reader for MyFile {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        let result = self.fd.read(buf);

        match result {                        
            Ok(read) => {
                self.last_nread = read;
                match read {
                    0 => Err(standard_error(io::EndOfFile)),
                    _ => Ok(read as uint)
                } 
            },  
            Err(_) => Err(standard_error(io::EndOfFile))
        }           
    }                   
}

   
pub fn open_read_only(path: &Path) -> MyFile {

    let result = open(
        &path.to_c_str(),
        rtio::Open,
        rtio::Read
    );

    match result {
        Ok(filedesc) => {
            MyFile{
                fd: box filedesc,
                last_nread: -1
            }
        },
        Err(e) => {
            panic!(e);
        }
    }
}

pub fn open_write_only(path: &Path) -> MyFile {

    let result = open(
        &path.to_c_str(),
        rtio::Open,
        rtio::Write
    );

    match result {
        Ok(filedesc) => {
            MyFile{
                fd: box filedesc,
                last_nread: -1
            }
        },
        Err(e) => {
            panic!(e);
        }
    }
}
