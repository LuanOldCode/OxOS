//! A simple VGA Mode 13h graphics driver implementation for embedded systems
//!
//! This module provides functionality to initialize and interact with VGA Mode 13h
//! (320x200 with 256 colors) graphics mode.
//!
//! [en] This driver allows drawing text and primitive shapes on a VGA display
//! [pt-br] Este driver permite desenhar texto e formas primitivas em um display VGA
use OxOS::*;
use core::{convert::Infallible, fmt::Write};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_7X14},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    text::Text,
};
use drivers::sbi;

/// Main entry point for the application
///
/// Initializes the VGA graphics mode and draws text and shapes
///
/// [en] This function never returns and handles all VGA display operations
/// [pt-br] Esta função nunca retorna e lida com todas as operações do display VGA
arch_entry!(init);
pub fn init() -> ! {
    let fb_base = 0x5000_0000 as *mut u8;

    unsafe {
        // NOTE: In qemu, `pci_base` is 0x3000_0000 and `dev` is 1, so `dev_base` is 0x3000_8000.
        let dev_base = 0x3000_8000 as *mut u32;

        // I want VGA
        let pci_class = dev_base.add(2).read_volatile() >> 8;
        if pci_class != 0x030000 {
            panic!("virtio-vga was not found");
        }
        // PCI configuration
        let io_base = {
            dev_base.add(4).write_volatile(0xFFFF_FFFF);
            let fb_size = (!dev_base.add(4).read_volatile() | 0xF) as usize + 1;
            let io_base = fb_base.add(fb_size);
            dev_base.add(6).write_volatile(0xFFFF_FFFF);
            let io_size = (!dev_base.add(6).read_volatile() | 0xF) as usize + 1;

            dev_base.add(4).write_volatile(fb_base as u32 | 8);
            dev_base.add(6).write_volatile(io_base as u32 | 8);

            let cmd = dev_base.add(1);
            cmd.write_volatile(cmd.read_volatile() | 0x0002);

            println!(
                "╔════════════════════════════════════════════════════════════╗"
            );
            println!(
                "║   VGA fb_base {:08x} size {}M io_base {:08x} size {:4} ║",
                fb_base as usize,
                (fb_size + 0x80000) >> 20,
                io_base as usize,
                io_size,
            );
            println!(
                "╚════════════════════════════════════════════════════════════╝"
            );

            io_base
        };

        vga::set_mode13(io_base);

        // Increase the display size to 640x480
        fb_base.write_bytes(0, 640 * 480);
    };

    let mut display = unsafe { Mode13Display::new(fb_base) };

    // Draw OxOS banner
    let logo_start_y = 20;
    let ascii_logo = [
        r"  _____              __   ____",
        r" /\  __`\          /'__`\/\  _`\",
        r" \ \ \/\ \   __  _/\ \/\ \ \,\ \_\",
        r"  \ \ \ \ \ /\ \/'\ \ \ \ \/_\__ \",
        r"   \ \ \_\ \\/>  </\ \ \_\ \/\ \ \ \",
        r"    \ \_____\/\_/\_\\ \____/\ `\____\",
        r"     \/_____/\//\/_/ \/___/  \/_____/",
        "",
        "  ____________________________________",
        r"/\                                    \",
        r"\ \       OxOS - Version 0.0.001       \",
        r" \ \     Paulo-D2000 & LuanOldCode      \",
        r"  \ \____________________________________\",
        r"   \/____________________________________/",
    ];

    for (i, line) in ascii_logo.iter().enumerate() {
        Text::new(
            line,
            Point::new(20, logo_start_y + (i as i32) * 12),
            MonoTextStyleBuilder::new()
                .font(&FONT_7X14)
                .text_color(Rgb888::WHITE)
                .build(),
        )
        .draw(&mut display)
        .unwrap();
    }

    loop {
        let c = sbi::getchar();
        if let Some(c) = c {
            if c != 0 {
                sbi::putchar(c);
            }
        }
    }
}

/// Represents a display using VGA Mode 13h (320x200, 256 colors)
///
/// [en] Handles pixel drawing operations in Mode 13h
/// [pt-br] Manipula operações de desenho de pixels no Modo 13h
pub struct Mode13Display {
    base: *mut u8,
}

impl Mode13Display {
    /// Creates a new Mode13Display from a base memory address
    ///
    /// [en] This function is unsafe because it requires valid memory access
    /// [pt-br] Esta função é insegura porque requer acesso válido à memória
    #[inline]
    pub unsafe fn new(base: *mut u8) -> Self {
        Self { base }
    }

    /// Converts a color component to a value compatible with the VGA palette
    ///
    /// [en] Maps 8-bit color values to the limited VGA color space
    /// [pt-br] Mapeia valores de cor de 8 bits para o espaço de cor limitado do VGA
    #[inline(always)]
    fn _color_component_to_safe_color(c: u8) -> u8 {
        const TABLE: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ];
        TABLE[c as usize]
    }

    /// Sets a single pixel at the specified coordinates with the given color
    ///
    /// [en] Returns None if the coordinates are out of bounds
    /// [pt-br] Retorna None se as coordenadas estiverem fora dos limites
    pub fn set_pixel(&mut self, coord: Point, color: Rgb888) -> Option<()> {
        if let Ok((x @ 0..=319, y @ 0..=199)) = coord.try_into() {
            let index = x as usize + y as usize * 320;

            let r = Self::_color_component_to_safe_color(color.r());
            let g = Self::_color_component_to_safe_color(color.g());
            let b = Self::_color_component_to_safe_color(color.b());
            let color = 16 + r + g * 6 + b * 36;

            unsafe {
                self.base.add(index).write_volatile(color);
            }

            Some(())
        } else {
            None
        }
    }
}

impl DrawTarget for Mode13Display {
    type Color = Rgb888;
    type Error = Infallible;

    /// Draws a collection of pixels to the display
    ///
    /// [en] Processes an iterator of pixels and draws each one
    /// [pt-br] Processa um iterador de pixels e desenha cada um
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_pixel(coord, color);
        }
        Ok(())
    }
}

impl OriginDimensions for Mode13Display {
    /// Returns the size of the display (320x200)
    ///
    /// [en] Provides dimensions for the embedded-graphics system
    /// [pt-br] Fornece dimensões para o sistema embedded-graphics
    fn size(&self) -> Size {
        Size::new(320, 200)
    }
}
