use nusb::MaybeFuture;
use nusb::transfer::{ControlOut, ControlType, Recipient};
use std::io;
use std::time::Duration;

const VENDOR_ID: u16 = 0x05c6;
const PRODUCT_ID: u16 = 0xf626;

const RNDIS_INTERFACE: u8 = 1;

fn main() -> Result<(), io::Error> {
    let timeout = Duration::from_secs(1);
    let enable_command_mode = ControlOut {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: 0xa0,
        value: 0,
        index: 0,
        data: &[],
    };

    let device = nusb::list_devices()
        .wait()?
        .find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Device not found!"))?
        .open()
        .wait()?;
    let interface = device.detach_and_claim_interface(RNDIS_INTERFACE).wait()?;

    interface
        .control_out(enable_command_mode, timeout)
        .wait()
        .or_else(|e| match e {
            // The device rebooting here indicates success but may give us a pipe error
            nusb::transfer::TransferError::Stall => Ok(()),
            other => Err(other),
        })?;
    println!("ADB should now be enabled!");
    Ok(())
}
