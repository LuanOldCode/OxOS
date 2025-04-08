#include <stdint.h>
#include <stddef.h>
#include <stdarg.h>

#define UART        0x10000000
#define UART_THR    (uint8_t*)(UART+0x00) // THR:transmitter holding register
#define UART_LSR    (uint8_t*)(UART+0x05) // LSR:line status register
#define UART_LSR_EMPTY_MASK 0x40          // LSR Bit 6: Transmitter empty; both the THR and LSR are empty

int lib_putc(char ch) {
	while ((*UART_LSR & UART_LSR_EMPTY_MASK) == 0);
	return *UART_THR = ch;
}

void lib_puts(char *s) {
	while (*s) lib_putc(*s++);
}

int lib_vsnprintf(char * out, size_t n, const char* s, va_list vl)
{
    int format = 0;
    int longarg = 0;
    size_t pos = 0;
    for( ; *s; s++) {
        if (format) {
            switch(*s) {
            case 'l': {
                longarg = 1;
                break;
            }
            case 'p': {
                longarg = 1;
                if (out && pos < n) {
                    out[pos] = '0';
                }
                pos++;
                if (out && pos < n) {
                    out[pos] = 'x';
                }
                pos++;
            }
            case 'x': {
                long num = longarg ? va_arg(vl, long) : va_arg(vl, int);
                int hexdigits = 2*(longarg ? sizeof(long) : sizeof(int))-1;
                for(int i = hexdigits; i >= 0; i--) {
                    int d = (num >> (4*i)) & 0xF;
                    if (out && pos < n) {
                        out[pos] = (d < 10 ? '0'+d : 'a'+d-10);
                    }
                    pos++;
                }
                longarg = 0;
                format = 0;
                break;
            }
            case 'd': {
                long num = longarg ? va_arg(vl, long) : va_arg(vl, int);
                if (num < 0) {
                    num = -num;
                    if (out && pos < n) {
                        out[pos] = '-';
                    }
                    pos++;
                }
                long digits = 1;
                for (long nn = num; nn /= 10; digits++)
                    ;
                for (int i = digits-1; i >= 0; i--) {
                    if (out && pos + i < n) {
                        out[pos + i] = '0' + (num % 10);
                    }
                    num /= 10;
                }
                pos += digits;
                longarg = 0;
                format = 0;
                break;
            }
            case 's': {
                const char* s2 = va_arg(vl, const char*);
                while (*s2) {
                    if (out && pos < n) {
                        out[pos] = *s2;
                    }
                    pos++;
                    s2++;
                }
                longarg = 0;
                format = 0;
                break;
            }
            case 'c': {
                if (out && pos < n) {
                    out[pos] = (char)va_arg(vl,int);
                }
                pos++;
                longarg = 0;
                format = 0;
                break;
            }
            default:
                break;
            }
        }
        else if(*s == '%') {
          format = 1;
        }
        else {
          if (out && pos < n) {
            out[pos] = *s;
          }
          pos++;
        }
    }
    if (out && pos < n) {
        out[pos] = 0;
    }
    else if (out && n) {
        out[n-1] = 0;
    }
    return pos;
}

static char out_buf[1000]; // buffer for lib_vprintf()

int lib_vprintf(const char* s, va_list vl)
{
    int res = lib_vsnprintf(NULL, -1, s, vl);
    if (res+1 >= sizeof(out_buf)) {
        lib_puts("error: lib_vprintf() output string size overflow\n");
        while(1) {}
    }
    lib_vsnprintf(out_buf, res + 1, s, vl);
    lib_puts(out_buf);
    return res;
}

int lib_printf(const char* s, ...)
{
    int res = 0;
    va_list vl;
    va_start(vl, s);
    res = lib_vprintf(s, vl);
    va_end(vl);
    return res;
}

#define PCI_ECAM_BASE 0x30000000UL
#define TARGET_VENDOR_ID 0x1234  // Example vendor ID (Bochs SVGA vendor ID)
#define TARGET_DEVICE_ID 0x1111  // Example device ID (Bochs SVGA device ID)

// VBE Registers
#define VBE_DISPI_INDEX_ID (0)
#define VBE_DISPI_INDEX_XRES (1)
#define VBE_DISPI_INDEX_YRES (2)
#define VBE_DISPI_INDEX_BPP (3)
#define VBE_DISPI_INDEX_ENABLE (4)
#define VBE_DISPI_INDEX_BANK (5)
#define VBE_DISPI_INDEX_VIRT_WIDTH (6)
#define VBE_DISPI_INDEX_VIRT_HEIGHT (7)
#define VBE_DISPI_INDEX_X_OFFSET (8)
#define VBE_DISPI_INDEX_Y_OFFSET (9)

// VBE Types
#define VBE_DISPI_DISABLED (0x00) 
#define VBE_DISPI_ENABLED (0x01) 
#define VBE_DISPI_LFB_ENABLED (0x40)
#define VBE_DISPI_NOCLEARMEM (0x80)
#define VBE_DISPI_BPP_4 (0x04)
#define VBE_DISPI_BPP_8 (0x08)
#define VBE_DISPI_BPP_15 (0x0F)
#define VBE_DISPI_BPP_16 (0x10)
#define VBE_DISPI_BPP_24 (0x18)
#define VBE_DISPI_BPP_32 (0x20)

static inline uint32_t pci_config_read32(uint8_t bus, uint8_t device, uint8_t function, uint16_t offset) {
    uintptr_t addr = PCI_ECAM_BASE
                   | ((uintptr_t)bus << 20)
                   | ((uintptr_t)device << 15)
                   | ((uintptr_t)function << 12)
                   | (offset & 0xFFC);
    return *((volatile uint32_t *)addr);
}

static int pci_config_write32(uint8_t bus, uint8_t device, uint8_t function, uint16_t offset, uint32_t data) {
    uintptr_t addr = PCI_ECAM_BASE
                   | ((uintptr_t)bus << 20)
                   | ((uintptr_t)device << 15)
                   | ((uintptr_t)function << 12)
                   | (offset & 0xFFC);
    *((volatile uint32_t *)addr) = data;

    // read back to test (ignore all ones return)
    return (data == 0xFFFFFFFF) ? 0 : (*((volatile uint32_t *)addr) != data);
}

static int pci_config_write16(uint8_t bus, uint8_t device, uint8_t function, uint16_t offset, uint16_t data) {
    uintptr_t addr = PCI_ECAM_BASE
                   | ((uintptr_t)bus << 20)
                   | ((uintptr_t)device << 15)
                   | ((uintptr_t)function << 12)
                   | (offset & 0xFFC);
    *((volatile uint16_t *)addr) = data;

    // read back to test (ignore all ones return)
    return (data == 0xFFFFFFFF) ? 0 : (*((volatile uint16_t *)addr) != data);
}

typedef struct PciAddress
{
    uint8_t bus;
    uint8_t device;
}PciAddress;

PciAddress pci_find_device(uint16_t target_vendor_id, uint16_t target_device_id) {
    lib_printf("Scanning for PCI Devices...\r\n");
    for (uint8_t bus = 0; bus < 256; ++bus) {
        for (uint8_t device = 0; device < 32; ++device) {
            for (uint8_t function = 0; function < 8; ++function) {
                uint32_t vendor_device = pci_config_read32(bus, device, function, 0x00);
                uint16_t vendor_id = vendor_device & 0xFFFF;
                uint16_t device_id = (vendor_device >> 16) & 0xFFFF;

                if (vendor_id == 0xFFFF) continue; // No device

                uint32_t class_reg = pci_config_read32(bus, device, function, 0x08);
                uint8_t class_code = (class_reg >> 24) & 0xFF;
                uint8_t subclass = (class_reg >> 16) & 0xFF;
                uint8_t prog_if = (class_reg >> 8) & 0xFF;
                
                lib_printf("[PCI Device found at %02x:%02x.%x\r\n", bus, device, function);
                lib_printf("  Vendor ID: 0x%04x, Device ID: 0x%04x\r\n", vendor_id, device_id);
                lib_printf("  Class: 0x%02x, Subclass: 0x%02x, Prog IF: 0x%02x\r\n", class_code, subclass, prog_if);
            
                if(vendor_id == target_vendor_id && device_id == target_device_id){
                    lib_printf("PCI Device with Vendor ID: 0x%04x and Device ID: 0x%04x was found!\r\n", target_vendor_id, target_device_id);
                    return (PciAddress){bus, device};
                }
            }
        }
    }

    lib_printf("PCI Device with Vendor ID: 0x%04x and Device ID: 0x%04x not found.\r\n", target_vendor_id, target_device_id);
    return (PciAddress){0xFF, 0xFF};
}

static inline uint32_t pci_reg_read32(PciAddress pci, uint8_t reg) {
    return pci_config_read32(pci.bus, pci.device, 0, reg * 4);
}

static inline int pci_reg_write32(PciAddress pci, uint8_t reg, uint32_t data) {
    pci_config_write32(pci.bus, pci.device, 0, reg * 4, data);
}

typedef struct PCI_BAR{
    PciAddress pci_address;
    uint8_t bar_num;
    uint32_t base_addr;
    uint32_t size;
    uint8_t io_rsv_mem_type;
    int is_io;
    int mapped;
}PCI_BAR;

PCI_BAR pci_bar_map(PciAddress pci_address, uint8_t bar_num){
    PCI_BAR bar = {pci_address, bar_num, 0, 0, -1, 0};

    lib_printf("pci_bar_map: ");

    // save bar0
    uint32_t bar_addr, saved_bar;
    bar_addr =saved_bar = pci_reg_read32(bar.pci_address, bar.bar_num+4);
    //lib_printf("[BAR Mapping Debug]\r\n saving [BAR%d]: %08x\r\n", bar.bar_num, bar_addr);
 
     // write all ones to bar0
    if(pci_reg_write32(bar.pci_address, bar.bar_num+4, 0xFFFFFFFF)){
       lib_printf("Error writing all ones to [BAR%d]!\r\n", bar.bar_num);
       return bar;
    }
 
    // read bar 0
    bar_addr = pci_reg_read32(bar.pci_address, bar.bar_num+4);
    //lib_printf("read [BAR%d]: %08x\r\n", bar.bar_num, bar_addr);

    bar.is_io = (bar_addr & 0x03) == 1;
    if(bar.is_io){
        bar.base_addr = bar_addr >> 2;
        bar.io_rsv_mem_type = bar_addr & 0b10;
    }
    else{
        bar.base_addr = bar_addr >> 4;
        bar.io_rsv_mem_type = bar_addr & 0b110;
    }
    
    // get size
    bar_addr &= 0xFFFFFFFD;
    bar_addr = ~bar_addr;
    bar_addr++;
    bar.size = bar_addr;

    // restore
    if(pci_reg_write32(bar.pci_address, 4+bar.bar_num, saved_bar)){
        lib_printf("Error restoring [BAR%d]!\r\n", bar.bar_num);
    }
    else{
        bar.mapped = 1;
    }

    if(bar.is_io){
        lib_printf("[BAR%d]: 0x%08x-0x%08x is IO space (size: %d bytes, reserved: %d)\r\n", bar.bar_num, bar.base_addr, bar.base_addr+bar.size, bar.size, bar.io_rsv_mem_type);
    }
    else{
        lib_printf("[BAR%d]: 0x%08x-0x%08x is memory space (size: %d bytes, type: %d)\r\n", bar.bar_num, bar.base_addr, bar.base_addr+bar.size, bar.size, bar.io_rsv_mem_type);
    }
    
    return bar;
}

static inline uint32_t pci_bar_read32(PCI_BAR bar, uint32_t offset) {
    if(offset > bar.size || !bar.mapped){
        lib_printf("Error reading from [BAR%d]: 0x%08x (range: 0x%08x-0x%08x, mapped: %d)\r\n",bar.bar_num, bar.base_addr+offset, bar.base_addr, bar.base_addr+bar.size, bar.mapped);
        return -1;
    }
    return pci_config_read32(bar.pci_address.bus, bar.pci_address.device, 0, bar.base_addr + offset);
}

static inline int pci_bar_write32(PCI_BAR bar, uint32_t offset, uint32_t data) {
    if(offset > bar.size || !bar.mapped){
        lib_printf("Error writing to [BAR%d]: 0x%08x (range: 0x%08x-0x%08x, mapped: %d)\r\n",bar.bar_num, bar.base_addr+offset, bar.base_addr, bar.base_addr+bar.size, bar.mapped);
        return -1;
    }
    pci_config_write32(bar.pci_address.bus, bar.pci_address.device, 0, bar.base_addr + offset, data);
}

static inline int pci_bar_write16(PCI_BAR bar, uint32_t offset, uint16_t data) {
    if(offset > bar.size || !bar.mapped){
        lib_printf("Error writing to [BAR%d]: 0x%08x (range: 0x%08x-0x%08x, mapped: %d)\r\n",bar.bar_num, bar.base_addr+offset, bar.base_addr, bar.base_addr+bar.size, bar.mapped);
        return -1;
    }
    pci_config_write16(bar.pci_address.bus, bar.pci_address.device, 0, bar.base_addr + offset, data);
}

uint32_t 

int os_main(void) {
    lib_puts("Initializing PCIE Interface...\r\n");

    PciAddress vga_address = pci_find_device(TARGET_VENDOR_ID, TARGET_DEVICE_ID);
    lib_printf("[PCIE Info]\r\n Bus: 0x%02x, Device: 0x%02x\r\n", vga_address.bus, vga_address.device);

    uint8_t vga_header_type = (pci_reg_read32(vga_address, 3) >> 16) & 0xFF;
    lib_printf(" Header_type: 0x%02x\r\n", vga_header_type);

    if(vga_header_type != 0){
        lib_printf("Error! Idk what to do with header_type != 0\r\n");
    }

    // FrameBuffer bar, 16MB
    PCI_BAR bar0 = pci_bar_map(vga_address, 0);

    // MMIO bar, 4096 bytes in size 
    PCI_BAR bar2 = pci_bar_map(vga_address, 2);

    lib_puts("Initializing Bochs VBE MMIO...\r\n");

    // TODO: Configure MMIO (Bochs VBE)
    // Width, Height, Bpp
    uint32_t Width = 320;
    uint32_t Height = 240;
    uint32_t BitDepth = VBE_DISPI_BPP_32;

    // Flags ? 
    int UseLinearFrameBuffer = 1;
    int ClearVideoMemory = 0;

    // TODO: move to BgaSetVideoMode function?
    pci_bar_write32(bar2, 0x0400, VBE_DISPI_INDEX_ENABLE | (VBE_DISPI_DISABLED << 16));

    pci_bar_write16(bar2, ,VBE_DISPI_INDEX_XRES, Width);
    pci_bar_write16(bar2, ,VBE_DISPI_INDEX_XRES, Width);

    pci_bar_write16(bar2, ,VBE_DISPI_INDEX_YRES);
    pci_bar_write16(bar2, , Height);

    pci_bar_write16(bar2, ,VBE_DISPI_INDEX_BPP);
    pci_bar_write16(bar2, ,BitDepth);
    
    pci_bar_write16(bar2, ,VBE_DISPI_INDEX_ENABLE);
    pci_bar_write16(bar2,  ,VBE_DISPI_ENABLED |
            (UseLinearFrameBuffer ? VBE_DISPI_LFB_ENABLED : 0) |
            (ClearVideoMemory ? 0 : VBE_DISPI_NOCLEARMEM));

    // Write data to FB ?
    pci_bar_write32(bar0, 0, 0xFFFFFFFF);
    
    lib_puts("Hello OS!\r\n");
    while (1) {}
    return 0;
}