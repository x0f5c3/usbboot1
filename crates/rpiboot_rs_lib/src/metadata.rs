// Metadata file creation and handling for rpiboot_rs_lib
use std::fs::File;
use std::io::{Write, Result as IoResult};
use crate::duid::duid_decode_c40;

pub fn create_metadata_file(path: &str, serial_num: &str) -> IoResult<File> {
    let fname = format!("{}/{}.json", path, serial_num);
    let mut file = File::create(&fname)?;
    writeln!(file, "{{")?;
    Ok(file)
}

pub fn write_metadata_property(file: &mut File, property: &str, value: &str, index: usize) -> IoResult<()> {
    if index != 0 {
        write!(file, ",")?;
    }
    if property == "FACTORY_UUID" {
        if let Ok(decoded) = duid_decode_c40(value) {
            writeln!(file, "\n\t\"{}\" : \"{}\"", property, decoded)?;
        } else {
            writeln!(file, "\n\t\"{}\" : \"{}\"", property, value)?;
        }
    } else {
        writeln!(file, "\n\t\"{}\" : \"{}\"", property, value)?;
    }
    Ok(())
}

pub fn close_metadata_file(file: &mut File) -> IoResult<()> {
    writeln!(file, "\n}}")?;
    Ok(())
}
