use std::env;
use std::fs;
use std::io::Read;

use flate2::read::ZlibDecoder;

fn main() {
    println!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory");
    } else if args[1] == "cat-file" {
        if args[2] == "-p" {
            let hash = args[3].clone();
            let folder_name = &hash[0..2];
            let file_name = &hash[2..];
            let path = format!(".git/objects/{folder_name}/{file_name}");
            let mut object = fs::File::open(&path).unwrap();
            let mut content: Vec<u8> = vec![];
            let mut extracted_content = String::new();

            object.read_to_end(&mut content).unwrap();

            let mut decoder = ZlibDecoder::new(content.as_slice());

            decoder.read_to_string(&mut extracted_content).unwrap();
            print!("{extracted_content}");
        }
    } else {
        println!("unknown command: {}", args[1]);
    }
}
