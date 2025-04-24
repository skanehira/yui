#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Type {
    Null = 0,                // Program header table entry unused
    Load = 1,                // Loadable program segment
    Dynamic = 2,             // Dynamic linking information
    Interp = 3,              // Program interpreter
    Note = 4,                // Auxiliary information
    ShLib = 5,               // Reserved
    Phdr = 6,                // Entry for header table itself
    Tls = 7,                 // Thread-local storage segment
    Num = 8,                 // Number of defined types
    LoOs = 0x60000000,       // Start of OS-specific
    GnuEhFrame = 0x6474e550, // GCC .eh_frame_hdr segment
    GnuStack = 0x6474e551,   // Indicates stack executability
    GnuRelro = 0x6474e552,   // Read-only after relocation
    SunwBss = 0x6ffffffa,    // Sun Specific segment
    SunwStack = 0x6ffffffb,  // Stack segment
    HiOs = 0x6fffffff,       // End of OS-specific
    LoProc = 0x70000000,     // Start of processor-specific
    HiProc = 0x7fffffff,     // End of processor-specific
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Flag {
    Executable = 0x1,      // Segment is executable
    Writable = 0x2,        // Segment is writable
    Readable = 0x4,        // Segment is readable
    MaskOS = 0x0ff00000,   // OS-specific
    MaskProc = 0xf0000000, // Processor-specific
}

// TODO
// .text
// .rodata
// .hash
// .dynsym
// .dynstr
// .plt
// .rel.got
#[derive(Debug, PartialEq, Eq)]
pub struct TextSegument {}

// TODO
// .data
// .dynamic
// .got
// .bss
#[derive(Debug, PartialEq, Eq)]
pub struct DataSegument {}

#[derive(Debug, PartialEq, Eq)]
pub enum Segument {
    Text(TextSegument),
    Data(DataSegument),
}
