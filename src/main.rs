#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


extern crate reqwest;
extern crate serde_json;

#[path = "./utils/args.rs"]
mod args;
use args::ArgumentsParser;


fn main()
{
    let _args = ArgumentsParser::new();
    std::process::exit( match _args.parse() {
        Ok(_) => 0,
        Err(err) => { eprintln!("Program Error: {:?}", err); 1 }
    });
}