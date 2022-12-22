use crate::{args::{Args, Commands}, chunk, chunk_type, png};
use std::{fs, str::FromStr, io, io::Write};
use crate::Result;

/// Runs the specified command corresponding to the argument configuration 
pub fn run(args: Args) -> Result<()> {
    match args.command {
        Commands::encode{file_path , chunk_type, message, output_file} 
        => {
            return encode(file_path, chunk_type, message, output_file);
        }, 
        Commands::decode{file_path, chunk_type} => {
            return decode(file_path, chunk_type);
        },
        Commands::remove { file_path, chunk_type } => {
            return remove(file_path, chunk_type);
        },
        Commands::print{file_path} => {
            return print(file_path);
        }
    }
}

fn get_png(fp: String) -> Result<png::Png> {
    let bytes: Vec<u8> = fs::read(fp)?;
    png::Png::try_from(&bytes[..])
}

fn encode(fp: String, ct: String, msg: String, of: Option<String>) -> Result<()> {

    // get PNG struct from file path
    let mut png = get_png(fp)?;

    // get chunk_type from specified chunk type string
    let chunk_type = chunk_type::ChunkType::from_str(&ct)?;

    //convert chunk type and message into new chunk to be appended
    let secret_chunk = chunk::Chunk::new(chunk_type, msg.into_bytes());

    png.append_chunk(secret_chunk);

    if let Some(ofp) = of {
        fs::write(ofp, png.as_bytes())?;
    };

    Ok(())
}

fn decode(fp: String, ct: String) -> Result<()> {
    
    let mut png = get_png(fp)?;

    //Search the PNG for the specified chunk type
    match png.chunk_by_type(&ct) {

        // Print the chunk message as a string if found
        // Otherwise, return a ChunkNotFoundError
        Some(chunk) => {
            println!("{}", chunk.data_as_string()?);
            Ok(())
        }, 
        None => {
            Err(Box::new(png::ChunkNotFoundError))
        }
    }
}

fn remove(fp: String, ct: String) -> Result<()> {
    
    let ofp = fp.clone();

    let mut png = get_png(fp)?;

    //remove chunk corresponding to some chunk type
    png.remove_chunk(&ct)?;

    fs::write(ofp, png.as_bytes())?;

    Ok(())
}

fn print(fp: String) -> Result<()> {

    let mut png = get_png(fp)?;

    let ancilliary_chunks = png.ancillary_chunks();

    match ancilliary_chunks.len() {
        0 => {
            println!("No searchable PNG chunks available!");
        }
        _ => {
            print!("Searchable PNG chunks (by chunk type): ");
            for i in 0..ancilliary_chunks.len() {
                // print the specified chunk
                print!("{}", ancilliary_chunks[i]);

                // add comma seperation and newline spacing at the end
                if i != ancilliary_chunks.len() -1 {
                    print!(", ");
                } else {
                    print!("\n");
                }
            }
            io::stdout().flush().unwrap();
        }
    }
    Ok(())
}