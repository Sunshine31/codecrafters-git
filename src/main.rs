use codecrafters_git_rust::cat_file::cat_file;
use codecrafters_git_rust::hash_object::hash_object;
use codecrafters_git_rust::init::init;
use std::env;

fn main() {
    println!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args[2..]),
        "hash-object" => hash_object(&args[2..]),
        _ => println!("unknown command: {}", args[1]),
    }
}
