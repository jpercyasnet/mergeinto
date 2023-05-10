use std::io::BufReader;
use std::fs::File;
use std::path::{Path};

// function called by functions get_tomodel, get_dirmodel, get_strvector (twice)
//  Use to see if file have exif data
// input is the full path filename and output is error number
pub fn dump_file(path: &Path) -> Result<(), exif::Error> {
    let file = File::open(path)?;
    let _reader = exif::Reader::new().read_from_container(
        &mut BufReader::new(&file))?;

    Ok(())
}

