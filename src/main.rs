use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::exit;
use md5;
use sha1::Sha1;
use sha2;
use crc32fast::Hasher;
use sha2::{Digest, Sha256};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [<checksum1> <checksum2> ...]", args[0]);
        exit(1);
    }
    let filename = &args[1];
    let mut file = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            exit(1)
        }
    };
    println!("Checking file: {}", filename);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    print_hash("MD5", &md5::compute(&buffer).0, &args[2..]);
    print_hash("SHA1", &Sha1::digest(&buffer), &args[2..]);
    print_hash("SHA256", &Sha256::digest(&buffer), &args[2..]);
    print_hash("CRC32", &crc32(&buffer).to_be_bytes(), &args[2..]);
    println!();
    Ok(())
}

fn print_hash(algorithm: &str, hash: &[u8], checksums: &[String]) {
    let hash_hex = hex::encode(hash);
    let mut match_found = false;
    if !checksums.is_empty() {
        match_found = false;
        for checksum in checksums {
            if hash_hex == *checksum.to_lowercase() {
                match_found = true;
                break;
            }
        }
    }
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color_spec = ColorSpec::new();
    if match_found {
        color_spec.set_fg(Some(Color::Green));
    }
    stdout.set_color(&color_spec).unwrap();
    println!("{}\t: {}", algorithm, hash_hex);
    stdout.set_color(&ColorSpec::new()).unwrap();
}

fn crc32(buffer: &[u8]) -> u32 {
    let mut crc32 = Hasher::new();
    crc32.update(buffer);
    crc32.finalize()
}
