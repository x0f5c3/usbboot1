// Safe tar reading using the tar crate
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use tar::Archive;

pub fn bootfiles_read_tar<P: AsRef<Path>>(archive: P, filename: &str) -> Result<Vec<u8>, String> {
    let file = File::open(&archive).map_err(|e| format!("Failed to open archive: {}", e))?;
    let mut archive = Archive::new(file);
    for entry in archive.entries().map_err(|e| format!("Tar entries error: {}", e))? {
        let mut entry = entry.map_err(|e| format!("Tar entry error: {}", e))?;
        if let Ok(path) = entry.path() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.eq_ignore_ascii_case(filename) {
                    let mut buf = Vec::new();
                    entry.read_to_end(&mut buf).map_err(|e| format!("Read error: {}", e))?;
                    return Ok(buf);
                }
            }
        }
    }
    Err(format!("File {} not found in archive", filename))
}

