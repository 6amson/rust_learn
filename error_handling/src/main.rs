//these libraries are enums, and should be treated as such.
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file(path: &str) -> Result<String, Error>{

    //define the varaible to store the file content in as a string
    let mut file_content = String::new();
    
    //open the file using result options error handling
    let _file: Result<File, Error> = match File::open(path){
        Ok(file_final) => Ok(file_final),
        Err(error) => panic!("this error was found: {:?}", error),
    };

    //read the content of the file, pass in "&mut" and the string variable as an argument
    match _file?.read_to_string(&mut file_content){
        Ok(_) => return Ok(file_content),
        Err(err) => panic!("panic at the disco {:?}", err)
    }

}

fn main() {
     let _final = read_file("../carFactory/Cargo.toml");
     println!("File is here: {:?}", _final)
    //  match file {
    //      Ok(file_final) => println!("{:?}", file_final),
    //      Err(error) =>{ println!("error here {}", error)},
    //  }
}
