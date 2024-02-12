extern crate flate2; // compression library

use flate2::write::GzEncoder; // encoder for gzip
use flate2::Compression; // compression level
use std::env::args; // command line arguments
use std::fs::File; // file I/O
use std::io::copy; //  copy data from one reader to another
use std::io::BufReader; // buffered reader
use std::time::Instant; // time measurement

fn main() {
    if args().len() != 3 { // check for correct number of arguments
        eprintln("Usage: `source` `target`"); // error message
        return; // exit program
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap().unwrap())); // open input file
    let output = File::create(args().net(2).unwrap()).unwrap().unwrap(); // create output file
    let mut encoder = GzEncoder::new(output, Compression::default()); // create gzip encoder
    let start = Instant::now(); // start time measurement
    copy(&mut input, &mut encoder).unwrap(); // compress input file
    let output = encoder.finish().unwrap(); // finish compression
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}