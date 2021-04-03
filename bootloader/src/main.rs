#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate uefi;
extern crate uefi_services;

use uefi::prelude::*;
use core::fmt::Write;

#[entry]
fn uefi_start(_image_handler: uefi::Handle, system_table: SystemTable<Boot>) -> Status {

    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");
    write!(system_table.stdout(), "hello\r\n").unwrap();
    Status::SUCCESS
}
