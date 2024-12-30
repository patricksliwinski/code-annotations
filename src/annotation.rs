use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ffi::OsString;

pub struct Annotation {
    
}

pub fn read_annotation_file(path: OsString) -> std::io::Result<Vec<Annotation>> {
    let f = File::open(path)?;
    let lines = BufReader::new(f).lines();
    let mut annotations = Vec::new();

    return Ok(annotations);
}

fn parse_annotation(lines: &mut std::io::Lines<BufReader<File>>) -> std::io::Result<Annotation> {
    let annotation = Annotation {};
    lines.next();
    return Ok(annotation)
}
