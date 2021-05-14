use std::error::Error;
use std::fs;
use std::collections::HashMap;

use crossbeam; // 0.8.0

pub fn run(arg: Arg) -> Result<(), Box<dyn Error>> {


    let codes = zip_codes_in(arg.dir)?;

    println!("Results: {:?}", codes);

    Ok(())
}

pub struct Arg {
    pub dir: String,
}

fn zip_codes_in(dir: String) -> Result<Vec<String>, Box<dyn Error>> {
    let files = fs::read_dir(dir)?;

    let codes = files.filter_map(|file| fs::read_to_string(file.ok()?.path()).ok() )
        .collect::<Vec<_>>()
        .join(",\n")
        .split(",\n")
        .map(str::to_string)
        .collect::<Vec<_>>();
    
    Ok(codes)
}

impl Arg {
    pub fn new(args: &[String]) -> Result<Arg, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let dir = args[1].clone();

        Ok(Arg { dir})
    }
}
