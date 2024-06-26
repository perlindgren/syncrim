use num_enum::IntoPrimitive;

use serde::{Deserialize, Serialize};
use std::{
    convert::{From, TryFrom},
    fmt,
};

pub type Id = String;

pub type SignalUnsigned = u32;
pub type SignalSigned = i32;
pub type SignalBool = bool;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Signal {
    data: SignalValue,
    fmt: SignalFmt,
}

impl Signal {
    /// set value field
    pub fn set_value(&mut self, data: SignalValue) {
        self.data = data
    }
    /// set fmt field
    pub fn set_fmt(&mut self, fmt: SignalFmt) {
        self.fmt = fmt
    }
    /// get value field
    pub fn get_value(&self) -> SignalValue {
        self.data
    }
    /// get fmt field
    pub fn get_fmt(&self) -> SignalFmt {
        self.fmt
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignalValue {
    Uninitialized,
    Unknown,
    DontCare,
    Data(SignalUnsigned), // Maybe we should have something even more generic here
}

impl TryFrom<Signal> for bool {
    type Error = String;

    fn try_from(signal: Signal) -> Result<Self, Self::Error> {
        if let SignalValue::Data(data) = signal.data {
            Ok(data == 1)
        } else {
            Err(format!("Could not convert {:?} into bool", signal))
        }
    }
}

impl TryFrom<SignalValue> for bool {
    type Error = String;

    fn try_from(data: SignalValue) -> Result<Self, Self::Error> {
        if let SignalValue::Data(data) = data {
            Ok(data == 1)
        } else {
            Err(format!("Could not convert {:?} into bool", data))
        }
    }
}

impl TryFrom<SignalValue> for usize {
    type Error = String;

    fn try_from(data: SignalValue) -> Result<Self, Self::Error> {
        if let SignalValue::Data(data) = data {
            Ok(data as usize)
        } else {
            Err(format!("Could not convert {:?} into usize", data))
        }
    }
}

impl TryFrom<Signal> for SignalUnsigned {
    type Error = String;

    fn try_from(signal: Signal) -> Result<Self, Self::Error> {
        if let SignalValue::Data(data) = signal.data {
            Ok(data)
        } else {
            Err(format!(
                "Could not convert {:?} into SignalUnsigned",
                signal
            ))
        }
    }
}

impl TryFrom<SignalValue> for SignalUnsigned {
    type Error = String;

    fn try_from(data: SignalValue) -> Result<Self, Self::Error> {
        if let SignalValue::Data(data) = data {
            Ok(data)
        } else {
            Err(format!("Could not convert {:?} into SignalUnsigned", data))
        }
    }
}

impl From<SignalValue> for Signal {
    fn from(data: SignalValue) -> Signal {
        Signal {
            data,
            fmt: SignalFmt::Hex(SignalSize::_32, false),
        }
    }
}

impl From<(SignalUnsigned, SignalFmt)> for Signal {
    fn from((data, fmt): (SignalUnsigned, SignalFmt)) -> Signal {
        Signal {
            data: data.into(),
            fmt,
        }
    }
}

impl From<SignalUnsigned> for Signal {
    fn from(data: u32) -> Signal {
        Signal {
            data: SignalValue::Data(data),
            fmt: SignalFmt::Hex(SignalSize::_32, false),
        }
    }
}

impl From<bool> for Signal {
    fn from(b: bool) -> Signal {
        Signal {
            data: SignalValue::Data(b as SignalUnsigned),
            fmt: SignalFmt::Bool,
        }
    }
}

impl From<SignalUnsigned> for SignalValue {
    fn from(data: u32) -> SignalValue {
        SignalValue::Data(data)
    }
}

impl From<bool> for SignalValue {
    fn from(b: bool) -> SignalValue {
        SignalValue::Data(b as SignalUnsigned)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum SignalFmt {
    Ascii(SignalSize),
    Unsigned(SignalSize),
    Signed(SignalSize),
    Hex(SignalSize, bool), // bool == true for padding
    Binary(u8),            // just to set a limit to the number of bits
    Bool,                  // treats it as true/false
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
pub enum SignalSize {
    _8 = 1,
    _16 = 2,
    _32 = 4,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data {
            SignalValue::Data(value) => match self.fmt {
                SignalFmt::Ascii(signal_size) => {
                    let s: u8 = signal_size.into();

                    let bytes = &value.to_ne_bytes()[0..s as usize];
                    let s: String = bytes
                        .iter()
                        .map(|b| {
                            let c = *b as char;
                            if c.is_ascii_graphic() || c == ' ' {
                                c
                            } else {
                                '¤'
                            }
                        })
                        .rev()
                        .collect();

                    write!(f, "{}", s)
                }
                SignalFmt::Binary(size) => {
                    write!(f, "0b{}", &format!("{:032b}", value)[32 - size as usize..])
                }
                SignalFmt::Unsigned(size) => write!(
                    f,
                    "{}",
                    match size {
                        SignalSize::_8 => format!("{}", value as u8),
                        SignalSize::_16 => format!("{}", value as u16),
                        SignalSize::_32 => format!("{}", value),
                    }
                ),
                SignalFmt::Signed(size) => write!(
                    f,
                    "{}",
                    match size {
                        SignalSize::_8 => format!("{}", value as i8),
                        SignalSize::_16 => format!("{}", value as i16),
                        SignalSize::_32 => format!("{}", value as i32),
                    }
                ),
                SignalFmt::Hex(size, true) => write!(
                    f,
                    "{}",
                    match size {
                        SignalSize::_8 => format!("{:#04x}", value as u8),
                        SignalSize::_16 => format!("{:#06x}", value as u16),
                        SignalSize::_32 => format!("{:#010x}", value),
                    }
                ),
                SignalFmt::Hex(size, false) => write!(
                    f,
                    "{}",
                    match size {
                        SignalSize::_8 => format!("{:#x}", value as u8),
                        SignalSize::_16 => format!("{:#x}", value as u16),
                        SignalSize::_32 => format!("{:#x}", value),
                    }
                ),
                SignalFmt::Bool => write!(f, "{}", value != 0),
            },
            _ => write!(f, "{:?}", self.data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bool_fmt() {
        let mut signal: Signal = false.into();

        // test bool
        signal.set_fmt(SignalFmt::Bool);
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "false");

        signal.set_value(true.into());
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "true");
    }

    #[test]
    fn test_hex_pad_fmt() {
        let mut signal: Signal = 0x0234_0608.into();

        // test hex
        signal.set_fmt(SignalFmt::Hex(SignalSize::_32, true));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0x02340608");

        signal.set_fmt(SignalFmt::Hex(SignalSize::_16, true));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0x0608");

        signal.set_fmt(SignalFmt::Hex(SignalSize::_8, true));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0x08");
    }

    #[test]
    fn test_hex_fmt() {
        let mut signal: Signal = 0x0234_0608.into();

        // test hex
        signal.set_fmt(SignalFmt::Hex(SignalSize::_32, false));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0x2340608");

        signal.set_fmt(SignalFmt::Hex(SignalSize::_16, false));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0x608");

        signal.set_fmt(SignalFmt::Hex(SignalSize::_8, false));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0x8");
    }

    #[test]
    fn test_unsigned_fmt() {
        let mut signal: Signal = 0xF000_0000.into();

        // test unsigned
        signal.set_fmt(SignalFmt::Unsigned(SignalSize::_32));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "4026531840");

        signal.set_fmt(SignalFmt::Unsigned(SignalSize::_16));
        signal.set_value(0xF000_E000.into());
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "57344");

        signal.set_fmt(SignalFmt::Unsigned(SignalSize::_8));
        signal.set_value(0xF000_E0D0.into());
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "208");
    }

    #[test]
    fn test_signed_fmt() {
        let mut signal: Signal = 0xF000_0000.into();

        // test signed
        signal.set_fmt(SignalFmt::Signed(SignalSize::_32));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "-268435456");

        signal.set_fmt(SignalFmt::Signed(SignalSize::_16));
        signal.set_value(0xF000_E000.into());
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "-8192");

        signal.set_fmt(SignalFmt::Signed(SignalSize::_8));
        signal.set_value(0xF000_E0D0.into());
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "-48");
    }

    #[test]
    fn test_binary_fmt() {
        let mut signal: Signal = 0b0000_0000_0000_0001_0001_0000_0000_0010.into();

        // test binary
        signal.set_fmt(SignalFmt::Binary(32));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0b00000000000000010001000000000010");

        signal.set_fmt(SignalFmt::Binary(31));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0b0000000000000010001000000000010");

        signal.set_fmt(SignalFmt::Binary(2));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0b10");

        signal.set_fmt(SignalFmt::Binary(1));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(&s, "0b0");
    }

    #[test]
    fn test_ascii_fmt() {
        let text = 0x4142_4300;
        let mut signal: Signal = text.into();

        // test ascii
        signal.set_fmt(SignalFmt::Ascii(SignalSize::_32));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(s, "ABC¤");

        signal.set_fmt(SignalFmt::Ascii(SignalSize::_16));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(s, "C¤");

        signal.set_fmt(SignalFmt::Ascii(SignalSize::_8));
        let s = format!("{}", signal);
        println!("{}", s);
        assert_eq!(s, "¤");
    }
}
