use crate::error;

use crate::pe::optional_header;
use scroll::Pread;

/// DOS header present in all PE binaries
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct DosHeader {
    /// Magic number: 5a4d
    pub signature: u16,
    /// Pointer to PE header, always at offset 0x3c
    pub pe_pointer: u32,
}

pub const DOS_MAGIC: u16 = 0x5a4d;
pub const PE_POINTER_OFFSET: u32 = 0x3c;

impl DosHeader {
    pub fn parse(bytes: &[u8]) -> error::Result<Self> {
        let signature = bytes.pread_with(0, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse DOS signature (offset {:#x})", 0)))?;
        let pe_pointer = bytes.pread_with(PE_POINTER_OFFSET as usize, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse PE header pointer (offset {:#x})", PE_POINTER_OFFSET)))?;
        Ok (DosHeader { signature: signature, pe_pointer: pe_pointer })
    }
}

/// COFF Header
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct CoffHeader {
    /// COFF Magic: PE\0\0, little endian
    pub signature: u32,
    /// The machine type
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbol_table: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

pub const SIZEOF_COFF_HEADER: usize = 24;
/// PE\0\0, little endian
pub const COFF_MAGIC: u32 = 0x00004550;
pub const COFF_MACHINE_X86: u16 = 0x14c;
pub const COFF_MACHINE_X86_64: u16 = 0x8664;

impl CoffHeader {
    pub fn parse(bytes: &[u8], offset: &mut usize) -> error::Result<Self> {
        let mut coff = CoffHeader::default();
        coff.signature = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF signature (offset {:#x})", offset)))?;
        coff.machine = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF machine (offset {:#x})", offset)))?;
        coff.number_of_sections = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF number of sections (offset {:#x})", offset)))?;
        coff.time_date_stamp = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF time date stamp (offset {:#x})", offset)))?;
        coff.pointer_to_symbol_table = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF pointer to symbol table (offset {:#x})", offset)))?;
        coff.number_of_symbol_table = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF number of symbol (offset {:#x})", offset)))?;
        coff.size_of_optional_header = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF size of optional header (offset {:#x})", offset)))?;
        coff.characteristics = bytes.gread_with(offset, scroll::LE)
            .map_err(|_| error::Error::Malformed(format!("cannot parse COFF characteristics (offset {:#x})", offset)))?;
        Ok(coff)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Header {
    pub dos_header: DosHeader,
    pub coff_header: CoffHeader,
    pub optional_header: Option<optional_header::OptionalHeader>,
}

impl Header {
    pub fn parse(bytes: &[u8]) -> error::Result<Self> {
        let dos_header = DosHeader::parse(&bytes)?;
        let mut offset = dos_header.pe_pointer as usize;
        let coff_header = CoffHeader::parse(&bytes, &mut offset)?;
        let optional_header =
            if coff_header.size_of_optional_header > 0 {
                Some (bytes.pread::<optional_header::OptionalHeader>(offset)?)
            }
        else { None };
        Ok( Header { dos_header: dos_header, coff_header: coff_header, optional_header: optional_header })
    }
}

#[cfg(test)]
mod tests {
    use super::{DOS_MAGIC, COFF_MAGIC, COFF_MACHINE_X86, Header};

    const CRSS_HEADER: [u8; 688] =
        [0x4d, 0x5a, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00,
         0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0x00, 0x00, 0x00,
         0x0e, 0x1f, 0xba, 0x0e, 0x00, 0xb4, 0x09, 0xcd, 0x21, 0xb8, 0x01, 0x4c, 0xcd, 0x21, 0x54, 0x68,
         0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x20, 0x63, 0x61, 0x6e, 0x6e, 0x6f,
         0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x44, 0x4f, 0x53, 0x20,
         0x6d, 0x6f, 0x64, 0x65, 0x2e, 0x0d, 0x0d, 0x0a, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0xaa, 0x4a, 0xc3, 0xeb, 0xee, 0x2b, 0xad, 0xb8, 0xee, 0x2b, 0xad, 0xb8, 0xee, 0x2b, 0xad, 0xb8,
         0xee, 0x2b, 0xac, 0xb8, 0xfe, 0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x66, 0xb8, 0xeb, 0x2b, 0xad, 0xb8,
         0x33, 0xd4, 0x63, 0xb8, 0xea, 0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x7a, 0xb8, 0xed, 0x2b, 0xad, 0xb8,
         0x33, 0xd4, 0x64, 0xb8, 0xef, 0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x61, 0xb8, 0xef, 0x2b, 0xad, 0xb8,
         0x52, 0x69, 0x63, 0x68, 0xee, 0x2b, 0xad, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x50, 0x45, 0x00, 0x00, 0x4c, 0x01, 0x05, 0x00, 0xd9, 0x8f, 0x15, 0x52, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0xe0, 0x00, 0x02, 0x01, 0x0b, 0x01, 0x0b, 0x00, 0x00, 0x08, 0x00, 0x00,
         0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
         0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
         0x06, 0x00, 0x03, 0x00, 0x06, 0x00, 0x03, 0x00, 0x06, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x60, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0xe4, 0xab, 0x00, 0x00, 0x01, 0x00, 0x40, 0x05,
         0x00, 0x00, 0x04, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x10, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x3c, 0x30, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0xb8, 0x22, 0x00, 0x00,
         0x00, 0x50, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x10, 0x10, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x68, 0x10, 0x00, 0x00, 0x5c, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x00, 0x00,
         0x24, 0x06, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x60,
         0x2e, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x3c, 0x03, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00,
         0x00, 0x02, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0xc0, 0x2e, 0x69, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00,
         0xf8, 0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40,
         0x2e, 0x72, 0x73, 0x72, 0x63, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
         0x00, 0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x42, 0x2e, 0x72, 0x65, 0x6c, 0x6f, 0x63, 0x00, 0x00,
         0x86, 0x01, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x42,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    #[test]
    fn crss_header () {
        let header = Header::parse(&&CRSS_HEADER[..]).unwrap();
        assert!(header.dos_header.signature == DOS_MAGIC);
        assert!(header.coff_header.signature == COFF_MAGIC);
        assert!(header.coff_header.machine == COFF_MACHINE_X86);
        println!("header: {:?}", &header);
    }
}
