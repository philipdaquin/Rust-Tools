
//  File transfer Protocol 
//  You use this to upload and download files

extern crate ftp; 
use std::str; 
use std::io::Cursor; 
use ftp::{FtpError, FtpStream};

fn run_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
    //  The default port for ftp is 21
    let mut stream = FtpStream::connect((addr, 21))?; 
    
    //  Log on the user with their credentials
    stream.login(user, pass)?;
    println!("current directory: {}", stream.pwd()?); 

    let data = "This is a sample text to be sent!"; 

    //  cursor => Read, Write
    let mut reader = Cursor::new(data); 
    stream.put("new_file", &mut reader)?;
    stream.quit()
}
fn main() {
    run_ftp("ftp.dlptest.com", "johnapple@gmail.com", "MCDONALDSBURGER1").unwrap();

}
