mod usb;
mod bootfiles;
mod duid;
mod metadata;
mod file_server;
mod embedded;
mod bootfiles_tarcrate;

pub use usb::*;
pub use bootfiles::*;
pub use duid::*;
pub use metadata::*;
pub use file_server::*;
// embedded.rs only exposes stubs for now

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
