/// # file
/// `file` is a module for I/O

use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(str: String, path: String) {
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(str.as_bytes()).expect("Unable to write to file");
    f.sync_all().expect("Unable to sync file");
}
