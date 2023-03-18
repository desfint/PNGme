use clap::ArgMatches;
use std::convert::TryFrom;
use std::fs::{read, write};
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(args: &ArgMatches) {
    let image_path = args.get_one::<String>("PATH").unwrap();
    let image_bytes = read(image_path).unwrap();
    let mut image = Png::try_from(&image_bytes[..]).unwrap();
    let chunk_type = ChunkType::from_str(args.get_one::<String>("CHUNK_TYPE").unwrap()).unwrap();
    let message = args.get_one::<String>("MESSAGE").unwrap().bytes().collect();

    let chunk = Chunk::new(chunk_type, message);
    image.append_chunk(chunk);

    match args.get_one::<String>("output") {
        Some(path) => {
            write(path, image.as_bytes()).unwrap();
        }
        None => {
            write(image_path, image.as_bytes()).unwrap();
        }
    }
}

pub fn decode(args: &ArgMatches) {
    let image_bytes = read(args.get_one::<String>("PATH").unwrap()).unwrap();
    let image = Png::try_from(&image_bytes[..]).unwrap();
    let chunk_type = args.get_one::<String>("CHUNK_TYPE").unwrap();

    match image.chunk_by_type(&chunk_type[..]) {
        Some(chunk) => println!("{chunk}"),
        None => println!("Chunk not found.")
    }
}

pub fn remove(args: &ArgMatches) {
    let image_path = args.get_one::<String>("PATH").unwrap();
    let image_bytes = read(image_path).unwrap();
    let mut image = Png::try_from(&image_bytes[..]).unwrap();
    let chunk_type = args.get_one::<String>("CHUNK_TYPE").unwrap();

    image.remove_chunk(&chunk_type[..]).unwrap();

    match args.get_one::<String>("output") {
        Some(path) => {
            write(path, image.as_bytes()).unwrap();
        }
        None => {
            write(image_path, image.as_bytes()).unwrap();
        }
    }
}

pub fn print_chunks(args: &ArgMatches) {
    let image_bytes = read(args.get_one::<String>("PATH").unwrap()).unwrap();
    let image = Png::try_from(&image_bytes[..]).unwrap();

    println!("{image}");
}
