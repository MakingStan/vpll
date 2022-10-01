mod parser;
mod translater;

extern crate sdl2;

use std::{env, fs};


fn get_program_file_content() -> String
{
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("You have to supply a file name!");

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    return contents;
}

pub fn main() {

    let vpll_json = parser::convert_vpll_to_json(get_program_file_content());

    translater::start_sdl2(vpll_json);
}