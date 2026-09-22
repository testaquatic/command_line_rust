use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};

use crate::args::Args;

pub mod args;

fn open(filename: &str) -> Result<Box<dyn BufRead>, io::Error> {
    if filename == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(args: Args) -> Result<(), anyhow::Error> {
    let mut is_first_file = true;

    for filename in args.files.iter() {
        if !is_first_file {
            println!();
        } else {
            is_first_file = false;
        }
        let mut file = match open(filename) {
            Err(err) => {
                eprintln!("{}: {}", filename, err);
                continue;
            }
            Ok(file) => file,
        };

        if args.files.len() > 1 {
            print_filename(filename);
        }

        if let Some(bytes) = args.bytes {
            print_bytes(&mut file, bytes)?;
        } else {
            print_lines(&mut file, args.lines)?;
        }
    }

    Ok(())
}

fn print_filename(filename: &str) {
    println!("==> {} <==", filename)
}

fn print_lines(reader: &mut dyn BufRead, lines: u64) -> Result<(), std::io::Error> {
    for _ in 0..lines {
        let mut line_string = String::new();
        if reader.read_line(&mut line_string)? == 0 {
            break;
        }
        print!("{}", line_string);
        line_string.clear();
    }

    Ok(())
}

fn print_bytes(reader: &mut dyn BufRead, bytes: u64) -> Result<(), std::io::Error> {
    let bytes_vec = reader
        .bytes()
        .take(bytes as usize)
        .collect::<Result<Vec<u8>, io::Error>>()?;

    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&bytes_vec)?;

    Ok(())
}
