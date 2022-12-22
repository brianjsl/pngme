use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser, Debug)]
#[command(name = "PNGMe")]
#[command(author = "Brian Lee <brianjslmit@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Encode/Decode secret messages in your PNGs", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encodes a message string of a given PNG chunk type to a 
    /// specified a PNG file
    encode {
        /// Path to the PNG File
        file_path: String, 

        /// PNG chunk type as a UTF-8 string
        chunk_type: String,

        /// Message to be encoded
        message: String,

        /// Optional Output file for the modified PNG
        output_file: Option<String>
    },

    /// Decodes encoded message strings of a specified PNG chunk 
    /// type from a specified PNG file
    decode {
        /// Path to the PNG File
        file_path: String,

        /// Message to be encoded
        chunk_type: String
    },

    /// Removes encoded messages of a specified PNG chunk type 
    /// from a specified PNG file
    remove {
        /// Path to the PNG File
        file_path: String,

        /// Message to be encoded
        chunk_type: String,
    },

    /// Prints a list of PNG chunks that can be searched for messages
    print {
        /// Path to the PNG File
        file_path: String,
    },
}
