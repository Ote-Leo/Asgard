use std::fmt;
use std::mem::size_of;
use std::{ffi::c_void, mem::transmute};

use hex_pp::*;

type HModule = *const c_void;
type FarProc = *const c_void;

extern "stdcall" {
    fn LoadLibraryA(lib: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

const IPHLPAPI: *const u8 = "IPHLPAPI.dll\0".as_ptr();

struct IPAddr([u8; 4]);

impl fmt::Debug for IPAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{a}.{b}.{c}.{d}")
    }
}

#[repr(C)]
#[derive(Debug)]
struct IpOptionInformation {
    ttl: u8,
    tos: u8,
    flags: u8,
    options_size: u8,
    options_data: u32,
}

type Handle = *const c_void;
type IcmpSendEcho = extern "stdcall" fn(
    icmp_handle: Handle,
    destination_address: IPAddr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    time_out: u32,
) -> u32;

type IcmpCreateFile = extern "stdcall" fn() -> Handle;

#[repr(C)]
#[derive(Debug)]
struct IcmpEchoReply {
    address: IPAddr,
    status: u32,
    round_trip_time: u32,
    data_size: u16,
    reserved: u16,
    data: *const u8,
    options: IpOptionInformation,
}

fn main() {
    unsafe {
        let h = LoadLibraryA(IPHLPAPI);
        let icmp_create_file: IcmpCreateFile =
            transmute(GetProcAddress(h, "IcmpCreateFile\0".as_ptr()));
        let icmp_send_echo: IcmpSendEcho = transmute(GetProcAddress(h, "IcmpSendEcho\0".as_ptr()));

        let handle = icmp_create_file();
        let destination_address = IPAddr([8, 8, 8, 8]);
        let request_data = "O Romeo, Romeo, wherefore art thou Romeo?\0";
        let request_size = request_data.len() as u16;
        let request_options = Some(&IpOptionInformation {
            ttl: 128,
            tos: 0,
            flags: 0,
            options_size: 0,
            options_data: 0,
        });

        let reply_size = size_of::<IcmpEchoReply>();
        let reply_buffer_size = reply_size + 8 + request_data.len();
        let mut reply_buffer = vec![0u8; reply_buffer_size];

        let timeout = 4000;

        let reply_count = icmp_send_echo(
            handle,
            destination_address,
            request_data.as_ptr(),
            request_size,
            request_options,
            reply_buffer.as_mut_ptr(),
            reply_buffer_size as u32,
            timeout,
        );

        if reply_count == 0 {
            panic!("`IcmpEchoReply` failed! reply cound: {reply_count}");
        }

        let reply: &IcmpEchoReply = transmute(&reply_buffer);
        println!("{reply:#?}");
        println!("{:?}", reply_buffer.hex_dump());
    }
}
