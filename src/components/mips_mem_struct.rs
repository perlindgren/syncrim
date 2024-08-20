use elf::{endian::AnyEndian, ElfBytes};
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
/// A men contains two fields. One with the memory mapped data in a BTreeMap<u32,u8>.
/// And a hashmap with symbols.
#[derive(Default, Serialize, Deserialize)]
pub struct MipsMem {
    symbols: HashMap<u32, String>,
    sections: HashMap<u32, String>,
    data: BTreeMap<u32, u8>,
}
pub enum MemOpSize {
    Byte,
    Half,
    Word,
}

impl MipsMem {
    /// This function constructs a Mem struct using the elf sections to load the data.
    /// This may be un-reliable as the Elf might not always contain the sections,
    /// or contain un-relevant sections and no relevant ones
    /// # TODO fix result error. currently panics
    pub fn from_sections(elf_bytes: &[u8]) -> Result<MipsMem, ()> {
        let mut mem: MipsMem = MipsMem {
            symbols: HashMap::new(),
            data: BTreeMap::new(),
            sections: HashMap::new(),
        };

        let file = ElfBytes::<AnyEndian>::minimal_parse(elf_bytes).unwrap();
        // will crash if three is no str table
        let sect_head_str_tab = file.section_headers_with_strtab().unwrap();
        let sections = sect_head_str_tab.0.unwrap();
        let str_tab = sect_head_str_tab.1.unwrap();
        // for each section in elf
        for sect in sections {
            // if section is PROG BITS and size is none zero
            if sect.sh_type == 0x1 && sect.sh_size != 0 {
                let v_address = sect.sh_addr as u32;

                // if the section has flag alloc(0x2), aka lives in memory
                // if the section has a size larger than zero
                if sect.sh_flags & 0x2 == 0x2 && sect.sh_size != 0 {
                    let elf_address = sect.sh_offset; // offset into elf file where data is stored (note inside of elf Segment)
                    let elf_end_address = elf_address + sect.sh_size; // end address of data
                    let sect_data = &elf_bytes[elf_address as usize..elf_end_address as usize];
                    for (i, byte) in sect_data.into_iter().enumerate() {
                        mem.data.insert(v_address + i as u32, byte.to_owned());
                    }
                };

                // add section to section hashmap
                mem.sections.insert(
                    v_address,
                    str_tab.get(sect.sh_name as usize).unwrap().to_string(),
                );
            }
        }
        mem.get_symbols(&file);
        Ok(mem)
    }

    /// This function gets the data at that location
    pub fn get_unaligned(
        &self,
        address: u32,
        size: MemOpSize,
        sign_extend: bool,
        big_endian: bool,
    ) -> u32 {
        let size_int: usize = match size {
            MemOpSize::Byte => 1,
            MemOpSize::Half => 2,
            MemOpSize::Word => 4,
        };
        let bytes: Vec<u8> = (0..size_int)
            .map(|i| *self.data.get(&(address + i as u32)).unwrap_or(&0))
            .collect();

        match size {
            MemOpSize::Byte => {
                if sign_extend {
                    // first make byte an i8
                    // then when cast to i32 to sign extends
                    // convert to u32 as final return
                    bytes[0] as i8 as i32 as u32
                } else {
                    bytes[0] as u32
                }
            }
            MemOpSize::Half => {
                if sign_extend {
                    let int_16 = if big_endian {
                        i16::from_be_bytes(bytes.try_into().unwrap())
                    } else {
                        i16::from_le_bytes(bytes.try_into().unwrap())
                    };
                    // as i32 to get sign extended i32, and as u32 to get return type
                    int_16 as i32 as u32
                } else {
                    let uint_16: u16 = if big_endian {
                        u16::from_be_bytes(bytes.try_into().unwrap())
                    } else {
                        u16::from_le_bytes(bytes.try_into().unwrap())
                    };
                    uint_16 as u32
                }
            }
            MemOpSize::Word => {
                if sign_extend {
                    let int_32 = if big_endian {
                        i32::from_be_bytes(bytes.try_into().unwrap())
                    } else {
                        i32::from_le_bytes(bytes.try_into().unwrap())
                    };
                    int_32 as u32
                } else {
                    if big_endian {
                        u32::from_be_bytes(bytes.try_into().unwrap())
                    } else {
                        u32::from_le_bytes(bytes.try_into().unwrap())
                    }
                }
            }
        }
    }

    /// This function gets the data at that location, return error on miss aligned address
    pub fn get(
        &self,
        address: u32,
        size: MemOpSize,
        sign_extend: bool,
        big_endian: bool,
    ) -> Result<u32, ()> {
        let size_int: u32 = match size {
            MemOpSize::Byte => 1,
            MemOpSize::Half => 2,
            MemOpSize::Word => 4,
        };
        if address % size_int != 0 {
            Err(())
        } else {
            Ok(self.get_unaligned(address, size, sign_extend, big_endian))
        }
    }

    /// Will truncate the data to the given size and write the data to memory
    pub fn write(&mut self, address: u32, data: u32, size: MemOpSize, big_endian: bool) {
        match size {
            MemOpSize::Byte => {
                self.data.insert(address, data as u8);
            }
            MemOpSize::Half => {
                let uint_16 = data as u16;
                let bytes = if big_endian {
                    uint_16.to_be_bytes()
                } else {
                    uint_16.to_le_bytes()
                };
                self.data.insert(address, bytes[0]);
                self.data.insert(address + 1, bytes[1]);
            }
            MemOpSize::Word => {
                let bytes = if big_endian {
                    data.to_be_bytes()
                } else {
                    data.to_le_bytes()
                };
                bytes.iter().enumerate().for_each(|(i, byte)| {
                    self.data.insert(address + i as u32, byte.to_owned());
                });
            }
        }
    }
    /// will return error if the data is not aligned
    /// will truncate the data to size and write to memory
    pub fn write_aligned(
        &mut self,
        address: u32,
        data: u32,
        size: MemOpSize,
        big_endian: bool,
    ) -> Result<(), ()> {
        let size_int: u32 = match size {
            MemOpSize::Byte => 1,
            MemOpSize::Half => 2,
            MemOpSize::Word => 4,
        };
        if address % size_int != 0 {
            Err(())
        } else {
            self.write(address, data, size, big_endian);
            Ok(())
        }
    }

    /// Gets the elf symbol table, and set the self hashmap
    fn get_symbols(&mut self, elf_file: &ElfBytes<AnyEndian>) {
        match elf_file.symbol_table().unwrap() {
            Some((sym_table, string_table)) => {
                let mut sym_hash_map: HashMap<u32, String> = HashMap::new();
                let mut _hash_map: HashMap<u32, String> = HashMap::new();

                // for each symbol entry
                for sym_entry in sym_table {
                    let sym_name = string_table.get(sym_entry.st_name as usize).unwrap();

                    // if the symboltype is NOTYPE, bind is LOCAL and has a string add it
                    if sym_entry.st_symtype() == 0x0 && sym_entry.st_bind() == 0x0 && sym_name != ""
                    {
                        sym_hash_map.insert(sym_entry.st_value as u32, sym_name.to_string());
                    }
                }
                self.symbols = sym_hash_map
            }
            None => (),
        }
    }

    pub fn get_symbol_table(&self) -> HashMap<u32, String> {
        self.symbols.clone()
    }
    pub fn get_section_table(&self) -> HashMap<u32, String> {
        self.sections.clone()
    }
}
