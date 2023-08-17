use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use syxpack;
use kompress;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: kompress infile outfile");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    if let Some(buffer) = syxpack::read_file(&Path::new(&input_file)) {
        let result = kompress::kompress(buffer);
        let mut f = fs::File::create(&output_file).expect("to create file");
        f.write_all(&result).expect("to write to file");
    }
}
