pub use std::io::{BufReader, Bytes, Read, Lines, BufRead};
pub use std::fs::File;

// pub fn bytes_from_file(path: &str) -> Bytes<BufReader<File>> {
//     BufReader::new(
//         File::open(path).expect("File open error")
//     )
//         .bytes()
// }

pub fn lines_from_file(path: &str) -> Lines<BufReader<File>> {
    BufReader::new (
        File::open(path).expect("File open error")
    )
        .lines()
}

pub fn string_iterator(path: &str) -> impl Iterator<Item = String> {
    lines_from_file(path)
        .filter_map(|l| l.ok())
}
