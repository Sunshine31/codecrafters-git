use std::{
    fs::DirBuilder,
    io::Write,
    path::{Path, PathBuf},
};

use flate2::{Compression, write::ZlibEncoder};
use hex::ToHex;
use sha1::{Digest, Sha1};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let filename = &args[1];
            let file = std::fs::read(filename).unwrap();
            let header = get_header(&file);
            let mut content = header.into_bytes();

            content.extend(file);

            let sha = get_sha(&content);
            let compressed_file = compress(&content);
            let folder_path = create_folder(&sha);
            print_sha(&sha);
            save_file(&compressed_file, folder_path, get_file_sha(&sha));
        }
        _ => eprintln!("Unknown option"),
    }
}

fn get_sha(file: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(file);
    hasher.finalize().encode_hex::<String>()
}

fn compress(file: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(file).unwrap();
    encoder.finish().unwrap()
}

fn create_folder(sha: &str) -> PathBuf {
    // let path = format!(".git/objects/{}", &sha[0..2]);
    let path = Path::new(".git").join("objects").join(&sha[0..2]);
    DirBuilder::new().recursive(true).create(&path).unwrap();

    path
}

fn print_sha(sha: &str) {
    println!("{sha}");
}

fn save_file(file: &[u8], mut path: PathBuf, file_sha: &str) {
    path.push(file_sha);

    if path.exists() {
        return;
    }
    std::fs::write(path, file).unwrap();
}

fn get_file_sha(sha: &str) -> &str {
    &sha[2..]
}

fn get_header(content: &[u8]) -> String {
    let object_type = "blob";
    let size = content.len();

    format!("{object_type} {size}\0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_provide_file_sha() {
        let sha = "2qy39147jhrspetndhrsiutljgdwf897";
        let expected_file_sha = "y39147jhrspetndhrsiutljgdwf897";
        let result = get_file_sha(sha);

        assert_eq!(result, expected_file_sha);
    }
}
