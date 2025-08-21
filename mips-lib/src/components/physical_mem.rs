use elf::{endian::AnyEndian, ElfBytes};
use std::{
    any::Any,
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
};
use syncrim::{
    common::{Component, Ports, Simulator},
    signal::Id,
};

use serde::{Deserialize, Serialize};

/// Used to contain the "physical memory" which instruction memory and data memory uses
#[derive(Serialize, Deserialize, Clone)]
pub struct PhysicalMem {
    pub id: String,
    pub pos: (f32, f32),
    #[serde(skip)]
    pub mem: RefCell<MipsMem>,
    #[serde(skip)]
    pub history: RefCell<HashMap<usize, MemWriteReturn>>,
    // used for the un_clock(), this is because the simulator is not passed in un clock and we dont know what cycle we un clock to
    #[serde(skip)]
    pub cycle: RefCell<usize>,
}

impl PhysicalMem {
    pub fn new(id: impl Into<String>, pos: (f32, f32)) -> Self {
        Self {
            id: id.into(),
            pos,
            mem: RefCell::default(),
            history: RefCell::default(),
            cycle: RefCell::default(),
        }
    }

    pub fn load_file(&self, path: &PathBuf) -> Result<(), MemLoadError> {
        let data = fs::read(path)?;
        self.mem.replace(MipsMem::from_sections(&data)?);
        self.history.borrow_mut().clear();
        Ok(())
    }

    pub fn get_data(&self, start: u32, array: &mut [u8]) {
        let mem = &self.mem.borrow().data;
        for (i, b) in array.iter_mut().enumerate() {
            *b = *mem.get(&(i as u32 + start)).unwrap_or(&0)
        }
    }

    pub fn get_str_at_symbol(&self, symbol: &str) -> String {
        let sym_indx = &self.mem.borrow().symbols.iter().find_map(|(idx, sym)| {
            if sym == symbol {
                Some(*idx)
            } else {
                None
            }
        }).unwrap();
        let mem = &self.mem.borrow().data;
        // this is ugly
        let mut byte_vec: Vec<u8> = Vec::new();
        let mut i = 0;
        loop {
            let b =  *mem.get(&(sym_indx + i)).unwrap_or(&0);
            byte_vec.push(b);
            if b == 0 {
                break;
            }
            i += 1;
        };

        String::from_utf8_lossy(&byte_vec).to_string()
    }
}

#[typetag::serde]
impl Component for PhysicalMem {
    #[doc = " returns the (id, Ports) of the component"]
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(vec![], syncrim::common::OutputType::Combinatorial, vec![]),
        )
    }

    #[doc = " any"]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn un_clock(&self, _: &Simulator) {
        *self.cycle.borrow_mut() -= 1;
        if let Some(op) = self.history.borrow_mut().remove(&*self.cycle.borrow()) {
            self.mem.borrow_mut().revert(op);
        };
    }

    fn reset(&self) {
        // dont need to reset cycle, since cycle is updated in clock

        let mut hist_vec: Vec<(usize, MemWriteReturn)> =
            self.history.borrow_mut().drain().collect();
        // sort vec with largest first
        hist_vec.sort_by(|(a, _), (b, _)| a.cmp(b).reverse());
        let mut mem = self.mem.borrow_mut();
        for (_, op) in hist_vec {
            mem.revert(op);
        }
    }
}

/// A men contains three fields. One with the memory mapped data in a BTreeMap<u32,u8>,
/// hashmap with symbols and a hashmap with sections
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MipsMem {
    symbols: HashMap<u32, String>,
    sections: HashMap<u32, String>,
    data: BTreeMap<u32, u8>,
}
#[derive(Clone)]
pub enum MemOpSize {
    Byte,
    Half,
    Word,
}

/// An error type which describes different error that can occur while load a file
#[derive(Debug)]
pub enum MemLoadError {
    ParseError(elf::ParseError),
    FileReadError(std::io::Error),
    NoSections(),
    NoStrTab(),
}

impl std::fmt::Display for MemLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemLoadError::ParseError(parse_error) => write!(
                f,
                "An error occurred while parsing the efl file: {}",
                parse_error
            ),
            MemLoadError::NoSections() => write!(f, "Can't find any sections in this elf file"),
            MemLoadError::NoStrTab() => write!(f, "Can't find the String table"),
            MemLoadError::FileReadError(error) => {
                write!(f, "Error while reading the file: {}", error)
            }
        }
    }
}

impl From<elf::ParseError> for MemLoadError {
    fn from(value: elf::ParseError) -> Self {
        MemLoadError::ParseError(value)
    }
}

impl From<std::io::Error> for MemLoadError {
    fn from(value: std::io::Error) -> Self {
        MemLoadError::FileReadError(value)
    }
}

/// This struct is not ment to be cloned
#[derive(Clone)]
pub struct MemWriteReturn {
    address: u32,
    op_size: MemOpSize,
    bytes: [u8; 4],
}
impl MemWriteReturn {
    /// return the address the bytes comes from
    pub fn address(&self) -> u32 {
        self.address
    }
    pub fn op_size(&self) -> MemOpSize {
        self.op_size.clone()
    }
    /// return the bytes before the write where [0] is at address, [1] is at address + 1 and [N] is at address + N
    pub fn before_bytes(&self) -> Vec<u8> {
        match self.op_size {
            MemOpSize::Byte => vec![self.bytes[0]],
            MemOpSize::Half => vec![self.bytes[0], self.bytes[1]],
            MemOpSize::Word => vec![self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]],
        }
    }
}

impl MipsMem {
    /// This function constructs a Mem struct using the elf sections to load the data.
    /// This may be un-reliable as the Elf might not always contain the sections,
    /// or contain un-relevant sections and no relevant ones
    pub fn from_sections(elf_bytes: &[u8]) -> Result<MipsMem, MemLoadError> {
        let mut mem: MipsMem = MipsMem {
            symbols: HashMap::new(),
            data: BTreeMap::new(),
            sections: HashMap::new(),
        };

        let file = ElfBytes::<AnyEndian>::minimal_parse(elf_bytes)?;
        let sect_head_str_tab = file.section_headers_with_strtab()?;
        let sections = sect_head_str_tab.0.ok_or(MemLoadError::NoSections())?;
        let str_tab = sect_head_str_tab.1.ok_or(MemLoadError::NoStrTab())?;
        // for each section in elf
        for sect in sections {
            // if the section has flag alloc(0x2), aka lives in memory
            // if the section has a size larger than zero
            if sect.sh_flags & 0x2 == 0x2 && sect.sh_size != 0 {
                let v_address = sect.sh_addr as u32;

                // if the section has flag alloc(0x2), aka lives in memory
                // if the section has a size larger than zero
                if true {
                    let elf_address = sect.sh_offset; // offset into elf file where data is stored (note inside of elf Segment)
                    let elf_end_address = elf_address + sect.sh_size; // end address of data
                    let sect_data = &elf_bytes[elf_address as usize..elf_end_address as usize];
                    for (i, byte) in sect_data.iter().enumerate() {
                        mem.data.insert(v_address + i as u32, byte.to_owned());
                    }
                }

                // add section to section hashmap
                mem.sections.insert(
                    v_address,
                    // try to demangle the section name if possible
                    match str_tab.get(sect.sh_name as usize)?.rsplit_once(".") {
                        Some((a, b)) => {
                            format!("{}.{:#}", a, rustc_demangle::demangle(b)).to_string()
                        }
                        None => str_tab.get(sect.sh_name as usize).unwrap().to_string(),
                    },
                );
            };
        }
        mem.get_symbols(&file)?;
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
            MemOpSize::Word =>
            {
                #[allow(clippy::collapsible_else_if)]
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
    pub fn write(
        &mut self,
        address: u32,
        data: u32,
        size: MemOpSize,
        big_endian: bool,
    ) -> MemWriteReturn {
        match size {
            MemOpSize::Byte => {
                let b = self.data.insert(address, data as u8).unwrap_or(0);
                MemWriteReturn {
                    address,
                    op_size: size,
                    bytes: [b, 0, 0, 0],
                }
            }
            MemOpSize::Half => {
                let uint_16 = data as u16;
                let bytes = if big_endian {
                    uint_16.to_be_bytes()
                } else {
                    uint_16.to_le_bytes()
                };
                let b0 = self.data.insert(address, bytes[0]).unwrap_or(0);
                let b1 = self.data.insert(address + 1, bytes[1]).unwrap_or(0);
                MemWriteReturn {
                    address,
                    op_size: size,
                    bytes: [b0, b1, 0, 0],
                }
            }
            MemOpSize::Word => {
                let bytes = if big_endian {
                    data.to_be_bytes()
                } else {
                    data.to_le_bytes()
                };
                let mut b: [u8; 4] = [0; 4];
                bytes.iter().enumerate().for_each(|(i, byte)| {
                    b[i] = self
                        .data
                        .insert(address + i as u32, byte.to_owned())
                        .unwrap_or(0);
                });
                MemWriteReturn {
                    address,
                    op_size: size,
                    bytes: b,
                }
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
    ) -> Result<MemWriteReturn, ()> {
        let size_int: u32 = match size {
            MemOpSize::Byte => 1,
            MemOpSize::Half => 2,
            MemOpSize::Word => 4,
        };
        if address % size_int != 0 {
            Err(())
        } else {
            Ok(self.write(address, data, size, big_endian))
        }
    }

    /// Gets the elf symbol table, and set the self hashmap
    fn get_symbols(&mut self, elf_file: &ElfBytes<AnyEndian>) -> Result<(), MemLoadError> {
        if let Some((sym_table, string_table)) = elf_file.symbol_table()? {
            let mut sym_hash_map: HashMap<u32, String> = HashMap::new();

            // for each symbol entry
            for sym_entry in sym_table {
                let sym_name = string_table.get(sym_entry.st_name as usize)?;

                // if the symbol type is NOTYPE, bind is LOCAL and has a string add it
                if sym_entry.st_shndx != 0x0 && !sym_name.is_empty() {
                    sym_hash_map.insert(
                        sym_entry.st_value as u32,
                        format!("{:#}", rustc_demangle::demangle(sym_name)),
                    );
                }
            }
            self.symbols = sym_hash_map;
        }
        Ok(())
    }
    /// consumes undo the passed related mem write operation
    pub fn revert(&mut self, op: MemWriteReturn) {
        match op.op_size {
            MemOpSize::Byte => {
                self.data
                    .insert(op.address, op.bytes[0])
                    .expect("tried to revert an write operation that did not happen");
            }
            MemOpSize::Half => {
                self.data
                    .insert(op.address, op.bytes[0])
                    .expect("tried to revert an write operation that did not happen");
                self.data
                    .insert(op.address + 1, op.bytes[1])
                    .expect("tried to revert an write operation that did not happen");
            }
            MemOpSize::Word => {
                self.data
                    .insert(op.address, op.bytes[0])
                    .expect("tried to revert an write operation that did not happen");
                self.data
                    .insert(op.address + 1, op.bytes[1])
                    .expect("tried to revert an write operation that did not happen");
                self.data
                    .insert(op.address + 2, op.bytes[2])
                    .expect("tried to revert an write operation that did not happen");
                self.data
                    .insert(op.address + 3, op.bytes[3])
                    .expect("tried to revert an write operation that did not happen");
            }
        }
    }

    pub fn get_symbol_table(&self) -> HashMap<u32, String> {
        self.symbols.clone()
    }
    pub fn get_section_table(&self) -> HashMap<u32, String> {
        self.sections.clone()
    }
}
