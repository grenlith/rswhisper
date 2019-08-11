use std::io;
use std::io::BufReader;
use std::env;
use std::fs::File;
use std::process;

use rswhisper::{metadata, archives, datapoint};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} source", args[0]);
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let meta = metadata::read(&mut reader)?;
    let archives = archives::read_all(&mut reader, &meta)?;
    let datapoint = datapoint::read_seq(&mut reader)?;

    println!("{:?}", meta);
    println!("{:?}", archives);
    println!("{:?}", datapoint);
    Ok(())
}