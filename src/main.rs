#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};

#[macro_use]
mod uart;

use uart::Uart;

const PCI_ECAM_BASE: usize = 0x30000000;
const VGA_VENDOR_ID: u16 = 0x1234;
const VGA_DEVICE_ID: u16 = 0x1111;

const PCI_VENDOR_ID_OFFSET: usize = 0x00;
const PCI_DEVICE_ID_OFFSET: usize = 0x02;
const PCI_BAR0_OFFSET: usize = 0x10;
const PCI_BAR2_OFFSET: usize = 0x18;

// Bochs VBE interface
const DISPI_INDEX_ID: u16 = 0;
const DISPI_INDEX_XRES: u16 = 1;
const DISPI_INDEX_YRES: u16 = 2;
const DISPI_INDEX_BPP: u16 = 3;
const DISPI_INDEX_ENABLE: u16 = 4;

const DISPI_ID_MAGIC: u16 = 0xB0C5;
const DISPI_ENABLED: u16 = 0x01;
const DISPI_LFB_ENABLED: u16 = 0x40;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

static LOGO: &str = r"
 _____              __   ____
/\  __`\          /'__`\/\  _`\
\ \ \/\ \   __  _/\ \/\ \ \,\ \_\
 \ \ \ \ \ /\ \/'\ \ \ \ \/_\__ \
  \ \ \_\ \\/>  </\ \ \_\ \/\ \ \ \
   \ \_____\/\_/\_\\ \____/\ `\____\
    \/_____/\//\/_/ \/___/  \/_____/

------------------------------------
       OxOS - Version 0.0.001
------------------------------------
";

// Function to read a 32-bit word from PCI config space
fn pci_config_read32(bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    let addr: usize = PCI_ECAM_BASE
        | ((bus as usize) << 20)
        | ((device as usize) << 15)
        | ((function as usize) << 12)
        | ((offset as usize) & 0xFFC);
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    // Create & init UART
    let mut my_uart = uart::Uart::new(0x1000_0000);
	my_uart.init();

    // Print kernel logo
    let _ = my_uart.write_str(LOGO);
    
    // Access PCI device 00:01.00
    let (bus, device, function) = (0, 1, 0);
    let vendor_device = pci_config_read32(bus, device, function, 0x00);
    let vendor_id = (vendor_device & 0xFFFF) as u16;
    let device_id = ((vendor_device >> 16) & 0xFFFF) as u16;

    // Check if vendor isn't -1
    if vendor_id == 0xFFFF {
        let _ = my_uart.write_str("[PCI]: No device...\n");
    }
    else{
        // Print bis and device addresses
        let _ = my_uart.write_str("[PCI]: PCI Device found at: 0x");
        uart_hex_fmt!(my_uart, bus as u32, 2);
        let _ = my_uart.write_str(":0x");
        uart_hex_fmt!(my_uart, device as u32, 2);
        let _ = my_uart.write_str(":.");
        uart_hex_fmt!(my_uart, function as u32, 2);
        my_uart.endl();

        // Print vendor & device ID's
        let _ = my_uart.write_str("[PCI]:  Vendor ID: 0x");
        uart_hex_fmt!(my_uart, vendor_id as u32, 4);
        let _ = my_uart.write_str(", Device ID: 0x");
        uart_hex_fmt!(my_uart, device_id as u32, 4);
        my_uart.endl();
        
        // Read PCI class
        let class_reg = pci_config_read32(bus, device, function, 0x08);
        let class_code = ((class_reg >> 24) & 0xFF) as u8;
        let subclass = ((class_reg >> 16) & 0xFF) as u8;
        let prog_if = ((class_reg >> 8) & 0xFF) as u8;

        // Print PCI class
        let _ = my_uart.write_str("[PCI]:  Class: 0x");
        uart_hex_fmt!(my_uart, class_code as u32, 2);
        let _ = my_uart.write_str(", Subclass: 0x");
        uart_hex_fmt!(my_uart, subclass as u32, 2);
        let _ = my_uart.write_str(", Prog IF: 0x");
        uart_hex_fmt!(my_uart, prog_if as u32, 2);
        my_uart.endl();
    }

    // Write decorator...
    write!(my_uart, "Hello OS!\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}