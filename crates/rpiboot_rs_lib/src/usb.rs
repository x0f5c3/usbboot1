// USB device handling logic for rpiboot_rs_lib

use nusb::Device as NusbDevice;
use nusb::transfer::Bulk;
use nusb::MaybeFuture;
use nusb::transfer::{BulkReader, BulkWriter};
use anyhow::{anyhow, Result};

pub struct RpibootDevice {
    pub device: NusbDevice,
    pub out_ep: u8,
    pub in_ep: u8,
    pub bcm2711: bool,
    pub bcm2712: bool,
    pub serial_number: Option<String>,
    pub product_id: u16,
    pub vendor_id: u16,
}

pub fn open_device_with_vid(vendor_id: u16) -> Result<RpibootDevice> {
    let devices = nusb::list_devices().wait().map_err(|e| anyhow!("nusb list error: {e}"))?;
    for desc in devices {
        if desc.vendor_id() == vendor_id && matches!(desc.product_id(), 0x2763 | 0x2764 | 0x2711 | 0x2712) {
            let mut bcm2711 = false;
            let mut bcm2712 = false;
            match desc.product_id() {
                0x2711 => bcm2711 = true,
                0x2712 => bcm2712 = true,
                _ => {}
            }
            let serial_number = desc.serial_number().and_then(|x| Some(x.to_string()));
            let out_ep = if bcm2711 || bcm2712 { 3 } else { 1 };
            let in_ep = if bcm2711 || bcm2712 { 4 } else { 2 };
            let device = desc.open().wait().map_err(|e| anyhow!("nusb open error: {e}"))?;
            return Ok(RpibootDevice {
                device,
                out_ep,
                in_ep,
                bcm2711,
                bcm2712,
                serial_number,
                product_id: desc.product_id(),
                vendor_id: desc.vendor_id(),
            });
        }
    }
    Err(anyhow!("No device found"))
}

pub fn open_device_with_serialno(serialno: &str) -> Result<RpibootDevice> {
    let devices = nusb::list_devices().wait().map_err(|e| anyhow!("nusb list error: {e}"))?;
    for desc in devices {
        if let Some(sn) = desc.serial_number().map(|x| x.to_string()) {
            if sn == serialno && desc.vendor_id() == 0x0a5c && matches!(desc.product_id(), 0x2763 | 0x2764 | 0x2711 | 0x2712) {
                let mut bcm2711 = false;
                let mut bcm2712 = false;
                match desc.product_id() {
                    0x2711 => bcm2711 = true,
                    0x2712 => bcm2712 = true,
                    _ => {}
                }
                let out_ep = if bcm2711 || bcm2712 { 3 } else { 1 };
                let in_ep = if bcm2711 || bcm2712 { 4 } else { 2 };
                return Ok(RpibootDevice {
                    device: desc.open().wait().map_err(|e| anyhow!("nusb open error: {e}"))?,
                    out_ep,
                    in_ep,
                    bcm2711,
                    bcm2712,
                    serial_number: Some(sn),
                    product_id: desc.product_id(),
                    vendor_id: desc.vendor_id(),
                });
            }
        }
    }
    Err(anyhow!("No device found"))
}

pub fn claim_interface(_device: &mut RpibootDevice, _interface: u8) -> Result<()> {
    // nusb claims interface automatically
    Ok(())
}

pub fn ep_write(device: &mut RpibootDevice, buf: &[u8]) -> Result<usize> {
    let mut writer = device.device.claim_interface().wait()?..map_err(|e| format!("nusb open error: {e}"))?.bulk_writer(device.out_ep).map_err(|e| format!("nusb bulk_writer error: {e}"))?;
    let sent = writer.write(buf).map_err(|e| format!("nusb write error: {e}"))?;
    Ok(sent)
}

pub fn ep_read(device: &mut RpibootDevice, buf: &mut [u8]) -> Result<usize, String> {
    let mut reader = device.device.open().map_err(|e| format!("nusb open error: {e}"))?.bulk_reader(device.in_ep).map_err(|e| format!("nusb bulk_reader error: {e}"))?;
    let read = reader.read(buf).map_err(|e| format!("nusb read error: {e}"))?;
    Ok(read)
}
