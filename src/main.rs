#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};

#[macro_use]
mod uart;
use uart::Uart;

mod vga;

const PCI_ECAM_BASE: usize = 0x30000000;
const VGA_VENDOR_ID: u16 = 0x1234;
const VGA_DEVICE_ID: u16 = 0x1111;

const PCI_VENDOR_ID_OFFSET: usize = 0x00;
const PCI_DEVICE_ID_OFFSET: usize = 0x02;
const PCI_BAR0_OFFSET: u16 = 0x10;
const PCI_BAR2_OFFSET: u16 = 0x18;

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

// Function to read a 32-bit word from PCI config space
fn pci_config_write32(bus: u8, device: u8, function: u8, offset: u16, data: u32) {
    let addr: usize = PCI_ECAM_BASE
        | ((bus as usize) << 20)
        | ((device as usize) << 15)
        | ((function as usize) << 12)
        | ((offset as usize) & 0xFFC);

    let ptr = addr as *mut u32;
    unsafe { ptr.write_volatile(data) }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    let fb_base = 0x5000_0000 as *mut u8;

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

        unsafe {
            let io_base = {
                // read bar0 size & restore
                pci_config_write32(bus, device, function, PCI_BAR0_OFFSET, 0xFFFF_FFFF);
                let fb_size = !(pci_config_read32(bus, device, function, PCI_BAR0_OFFSET) & !0xF) as usize + 1;

                // read bar2 size & reset
                let io_base = fb_base.add(fb_size);
                pci_config_write32(bus, device, function, PCI_BAR2_OFFSET, 0xFFFF_FFFF);
                let io_size = !(pci_config_read32(bus, device, function, PCI_BAR2_OFFSET) & !0xF) as usize + 1;
                let io_base = fb_base.add(fb_size);

                // bar0
                pci_config_write32(bus, device, function, PCI_BAR0_OFFSET, fb_base as u32 | 8);

                // bar1
                pci_config_write32(bus, device, function, PCI_BAR2_OFFSET, io_base as u32 | 8);
                
                // header command Memory Space enable 
                let cmd = pci_config_read32(bus, device, function, 4);
                pci_config_write32(bus, device, function, 4, cmd | 0x0002);

                // Print vendor & device ID's
                let _ = my_uart.write_str("[VGA]: fb_base: 0x");
                uart_hex_fmt!(my_uart, fb_base as u32, 8);
                let _ = my_uart.write_str(" size 0x");
                uart_hex_fmt!(my_uart, fb_size as u32, 8);
                let _ = my_uart.write_str(" io_base: 0x");
                uart_hex_fmt!(my_uart, io_base as u32, 8);
                let _ = my_uart.write_str(" size 0x");
                uart_hex_fmt!(my_uart, io_size as u32, 8);
                my_uart.endl();

                io_base
            };

            vga::set_mode13(io_base);

            fb_base.write_bytes(0, 320 * 200);
        };

    }

    // Write decorator...
    write!(my_uart, "Hello OS!\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}