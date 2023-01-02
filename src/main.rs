use std::{fs::{File, OpenOptions}, io::{Read, Write}, str::FromStr};

use args::*;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = MyArgs::parse();

    // You can check the value provided by positional arguments, or option arguments
    match &cli.commands {
        Commands::Encode(params) => encode_msg(&params),
        Commands::Decode(params) => decode_msg(&params),
        Commands::Remove(params) => remove(&params),
        Commands::Print(params) => print(&params.image_path)
    }

    Ok(())
}

fn encode_msg(params: &EncodeCommand) {
    let mut bytes = Vec::new();
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(&params.image_path)
        .unwrap();

    file.read_to_end(&mut bytes).expect("Error while reading file");
    let mut png = Png::try_from(&bytes[..]).unwrap();

    png.append_chunk(Chunk::new(ChunkType::from_str(params.chunk_type.as_str()).unwrap(), params.message.as_bytes().to_vec()));

    if let Some(output_file) = &params.output_file {
        let mut f = File::create(output_file).unwrap();
        f.write_all(&png.as_bytes()).expect("Something went wrong opening the file");
    } else {
        file.write_all(&png.as_bytes()).unwrap();
    }
}

fn decode_msg(params: &DecodeCommand) {
    let mut bytes = Vec::new();
    let mut file = File::open(&params.image_path).unwrap();
    file.read_to_end(&mut bytes).expect("Error while reading file");
    let png = Png::try_from(&bytes[..]).unwrap();
    let chunk = png.chunk_by_type(&params.chunk_type);
    if let Some(data) = &chunk {
        println!("Encoded message is: \"{}\"", data.data_as_string().unwrap())
    } else {
        println!("There is no hidden message in this file.")
    }
}

fn remove(params: &RemoveCommand) {
    let mut bytes = Vec::new();
    let mut file = File::open(&params.image_path).unwrap();
    file.read_to_end(&mut bytes).expect("Error while reading file");
    let mut png = Png::try_from(&bytes[..]).unwrap();

    let removed_chunk = png.remove_chunk(&params.chunk_type).unwrap();

    println!("Chunk with type {} was removed", removed_chunk.chunk_type())
}

fn print(file_path: &String) {
    let mut bytes = Vec::new();
    let mut file = File::open(&file_path).unwrap();
    file.read_to_end(&mut bytes).expect("Error while reading file");
    let png = Png::try_from(&bytes[..]).unwrap();
    println!("{png}")
}