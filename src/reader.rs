use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};

use crate::color::Scheme;
use crate::errors::*;

pub fn parse(path: &str) -> Result<Scheme, SchemeReaderError> {
    let path = Path::new(path);

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let scheme_name = match lines.next() {
        Some(header) => header.map_err(|err| SchemeReaderError::IOError(err, "".into())),
        None => Err(SchemeReaderError::NoLinesError),
    }?;

    println!("Scheme name read as {}", &scheme_name);

    todo!()
}
