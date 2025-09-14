// File server logic for rpiboot_rs_lib

// This module will handle device file requests, reading files from disk or tar archives,
// and responding to device commands. The actual USB communication will be implemented
// in conjunction with the usb.rs module.

// TODO: Implement file server logic, including:
// - Handling GetFileSize, ReadFile, Done commands
// - Reading files using bootfiles_read or from disk
// - Responding to device over USB
// - Integrating with metadata.rs for metadata output

// Stub for now

use crate::usb::{RpibootDevice, ep_write, ep_read};
use crate::bootfiles::bootfiles_read;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub enum FileCommand {
    GetFileSize,
    ReadFile,
    Done,
}

pub struct FileMessage {
    pub command: FileCommand,
    pub fname: String,
}

pub struct FileServer {
    pub device: RpibootDevice,
    pub directory: Option<String>,
    pub use_tar: bool,
    pub tar_path: Option<String>,
}

impl FileServer {
    pub fn handle_message(&mut self, msg: &FileMessage) -> Result<(), String> {
        match msg.command {
            FileCommand::GetFileSize => {
                let size = self.get_file_size(&msg.fname)?;
                let size_bytes = (size as u32).to_le_bytes();
                ep_write(&mut self.device, &size_bytes).map_err(|e| format!("USB write error: {:?}", e))?;
            }
            FileCommand::ReadFile => {
                let data = self.read_file(&msg.fname)?;
                ep_write(&mut self.device, &data).map_err(|e| format!("USB write error: {:?}", e))?;
            }
            FileCommand::Done => {
                // End session
            }
        }
        Ok(())
    }

    pub fn get_file_size(&self, fname: &str) -> Result<usize, String> {
        if self.use_tar {
            if let Some(ref tar_path) = self.tar_path {
                let data = bootfiles_read(tar_path, fname).map_err(|e| format!("Tar read error: {}", e))?;
                Ok(data.len())
            } else {
                Err("No tar archive specified".to_string())
            }
        } else if let Some(ref dir) = self.directory {
            let path = format!("{}/{}", dir, fname);
            let mut file = File::open(&path).map_err(|e| format!("File open error: {}", e))?;
            let size = file.seek(SeekFrom::End(0)).map_err(|e| format!("Seek error: {}", e))?;
            Ok(size as usize)
        } else {
            Err("No directory specified".to_string())
        }
    }

    pub fn read_file(&self, fname: &str) -> Result<Vec<u8>, String> {
        if self.use_tar {
            if let Some(ref tar_path) = self.tar_path {
                bootfiles_read(tar_path, fname).map_err(|e| format!("Tar read error: {}", e))
            } else {
                Err("No tar archive specified".to_string())
            }
        } else if let Some(ref dir) = self.directory {
            let path = format!("{}/{}", dir, fname);
            let mut file = File::open(&path).map_err(|e| format!("File open error: {}", e))?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).map_err(|e| format!("File read error: {}", e))?;
            Ok(buf)
        } else {
            Err("No directory specified".to_string())
        }
    }
}
