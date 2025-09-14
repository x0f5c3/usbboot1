// Tar archive reading and boot file selection for rpiboot_rs_lib
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

const BLOCK_SIZE: usize = 512;

#[repr(C, packed)]
struct TarHeader {
    filename: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    csum: [u8; 8],
    link: [u8; 1],
    lname: [u8; 100],
}

pub fn bootfiles_read<P: AsRef<Path>>(archive: P, filename: &str) -> Result<Vec<u8>, String> {
    let mut fp = File::open(&archive).map_err(|e| format!("Failed to open archive: {}", e))?;
    let archive_size = fp.seek(SeekFrom::End(0)).map_err(|e| format!("Seek error: {}", e))?;
    fp.seek(SeekFrom::Start(0)).map_err(|e| format!("Seek error: {}", e))?;
    loop {
        let mut hdr_buf = [0u8; std::mem::size_of::<TarHeader>()];
        if fp.read_exact(&mut hdr_buf).is_err() {
            break;
        }
        let hdr: TarHeader = unsafe { std::ptr::read(hdr_buf.as_ptr() as *const _) };
        fp.seek(SeekFrom::Current((BLOCK_SIZE - std::mem::size_of::<TarHeader>()) as i64)).ok();
        let offset = fp.seek(SeekFrom::Current(0)).unwrap();
        if offset == archive_size {
            break;
        }
        let size = match std::str::from_utf8(&hdr.size) {
            Ok(s) => s.trim_matches(char::from(0)).trim(),
            Err(_) => "0",
        };
        let size = usize::from_str_radix(size, 8).unwrap_or(0);
        if offset + size as u64 > archive_size {
            return Err("Corrupted archive".to_string());
        }
        let fname = match std::str::from_utf8(&hdr.filename) {
            Ok(s) => s.trim_matches(char::from(0)),
            Err(_) => "",
        };
        if fname.eq_ignore_ascii_case(filename) {
            let mut data = vec![0u8; size];
            fp.read_exact(&mut data).map_err(|e| format!("Read error: {}", e))?;
            return Ok(data);
        } else {
            fp.seek(SeekFrom::Current(((size + BLOCK_SIZE - 1) & !(BLOCK_SIZE - 1)) as i64)).ok();
        }
    }
    Err(format!("File {} not found in archive", filename))
}
