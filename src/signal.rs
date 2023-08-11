use crate::common::{Input, Simulator};
use num_enum::IntoPrimitive;

use serde::{Deserialize, Serialize};
use std::{
    convert::{From, TryFrom},
    fmt,
};

pub type Id = String;

pub type SignalUnsigned = u32;
pub type SignalSigned = i32;

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum SignalExpr {
    BinOp(BinOp, Box<SignalExpr>, Box<SignalExpr>),
    Not(Box<SignalExpr>),
    Input(Input),
    Constant(Signal),
}

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

#[cfg_attr(feature = "gui-vizia", derive(Data))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum BinOp {
    BoolOp(BoolOp),
    CmpOp(CmpOp),
}

#[cfg_attr(feature = "gui-vizia", derive(Data))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum BoolOp {
    And,
    Or,
}

#[cfg_attr(feature = "gui-vizia", derive(Data))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum CmpOp {
    Eq,
    GtUnsigned,
    GtSigned,
}

impl SignalExpr {
    fn signal(&self, simulator: &Simulator) -> Result<Signal, String> {
        match self {
            SignalExpr::Input(input) => Ok(simulator.get_input_signal(input)),
            SignalExpr::Constant(constant) => Ok(*constant),
            _ => Err(format!("expected Signal, found {:?}", self)),
        }
    }
    pub fn eval(&self, simulator: &Simulator) -> Result<bool, String> {
        match self {
            Self::BinOp(op, lhs, rhs) => match op {
                BinOp::BoolOp(bool_op) => {
                    let lhs = lhs.eval(simulator)?;
                    let rhs = rhs.eval(simulator)?;
                    Ok(match bool_op {
                        BoolOp::And => lhs && rhs,
                        BoolOp::Or => lhs || rhs,
                    })
                }
                BinOp::CmpOp(cmp_op) => {
                    let lhs = TryInto::<SignalUnsigned>::try_into(lhs.signal(simulator)?)?;
                    let rhs = TryInto::<SignalUnsigned>::try_into(rhs.signal(simulator)?)?;
                    Ok(match cmp_op {
                        CmpOp::Eq => lhs == rhs,
                        CmpOp::GtSigned => lhs as SignalSigned > rhs as SignalSigned,
                        CmpOp::GtUnsigned => lhs > rhs,
                    })
                }
            },

            SignalExpr::Not(expr) => {
                let expr = expr.eval(simulator)?;
                Ok(!expr)
            }
            _ => {
                let s: SignalUnsigned = self.signal(simulator)?.try_into()?;
                Ok(s != 0)
            }
        }
    }
}

impl fmt::Display for SignalExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BinOp(bin_op, lhs, rhs) => format!("{}{}{}", lhs, bin_op, rhs),
                Self::Not(e) => format!("!{}", e),
                Self::Constant(c) => format!("{}", c),
                Self::Input(Input { id, field }) => format!("{}.{}", id, field),
            }
        )
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BoolOp(bool_op) => format!("{}", bool_op),
                Self::CmpOp(cmp_op) => format!("{}", cmp_op),
            }
        )
    }
}

impl fmt::Display for BoolOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "&&",
                Self::Or => "||",
            }
        )
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Eq => "==",
                Self::GtSigned => ">i",
                Self::GtUnsigned => ">u",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{common::ComponentStore, components::ProbeOut};
    #[test]
    fn test_expr() {
        let cs = ComponentStore {
            store: vec![ProbeOut::rc_new("probe_out")],
        };

        let mut simulator = Simulator::new(&cs);
        // output
        let out = Input::new("probe_out", "out");

        // check reset state
        println!("<reset>");
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(&out), 0.into());

        // check constant false
        println!("<check constant false>");
        let expr = SignalExpr::Constant(false.into());
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(false), expr.eval(&simulator));

        // check constant true
        println!("<check constant true>");
        let expr = SignalExpr::Constant(true.into());
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(true), expr.eval(&simulator));

        // check not constant true
        println!("<check not constant true>");
        let expr = SignalExpr::Not(Box::new(SignalExpr::Constant(true.into())));
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(false), expr.eval(&simulator));

        // check not constant false
        println!("<check not constant false>");
        let expr = SignalExpr::Not(Box::new(SignalExpr::Constant(false.into())));
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(true), expr.eval(&simulator));

        // check "probe_out" "out"
        println!("<check probe out>");
        let expr = SignalExpr::Input(out.clone());
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(false), expr.eval(&simulator));

        // check eq "probe_out" "out" against constant
        println!("<check probe out against constant>");
        let expr = SignalExpr::BinOp(
            BinOp::CmpOp(CmpOp::Eq),
            Box::new(SignalExpr::Input(out.clone())),
            Box::new(SignalExpr::Constant(false.into())),
        );
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(true), expr.eval(&simulator));

        // check input 0 greater than neg signed
        println!("<check greater than>");
        let expr = SignalExpr::BinOp(
            BinOp::CmpOp(CmpOp::GtSigned),
            Box::new(SignalExpr::Input(out.clone())),
            Box::new(SignalExpr::Constant(u32::MAX.into())),
        );
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(true), expr.eval(&simulator));

        // check input 0 greater than 1 unsigned
        println!("<check greater than>");
        let expr = SignalExpr::BinOp(
            BinOp::CmpOp(CmpOp::GtUnsigned),
            Box::new(SignalExpr::Input(out.clone())),
            Box::new(SignalExpr::Constant(1.into())),
        );
        println!("expr {:?}", expr);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(false), expr.eval(&simulator));

        // update simulator value
        println!("<check update simulator>");
        simulator.set_out_value("probe_out", "out", 2);
        println!("eval.expr {:?}", expr.eval(&simulator));
        assert_eq!(Ok(true), expr.eval(&simulator));

        // check and
        println!("<check and>");
        let expr2 = SignalExpr::BinOp(
            BinOp::BoolOp(BoolOp::And),
            Box::new(expr.clone()),
            Box::new(SignalExpr::Constant(false.into())),
        );
        println!("eval.expr2 {:?}", expr2.eval(&simulator));
        assert_eq!(Ok(false), expr2.eval(&simulator));

        // check or
        println!("<check or>");
        let expr2 = SignalExpr::BinOp(
            BinOp::BoolOp(BoolOp::Or),
            Box::new(expr.clone()),
            Box::new(SignalExpr::Constant(false.into())),
        );
        println!("eval.expr2 {:?}", expr2.eval(&simulator));
        assert_eq!(Ok(true), expr2.eval(&simulator));

        // check complex
        println!("<check complex>");
        let expr2 = SignalExpr::BinOp(
            BinOp::BoolOp(BoolOp::And),
            Box::new(expr.clone()), // true
            Box::new(SignalExpr::BinOp(
                BinOp::BoolOp(BoolOp::Or),
                Box::new(SignalExpr::Constant(false.into())), // false
                Box::new(expr.clone()),                       // true
            )),
        );
        println!("eval.expr2 {:?}", expr2.eval(&simulator));
        assert_eq!(Ok(true), expr2.eval(&simulator));

        // check complex
        println!("<check complex>");
        let expr2 = SignalExpr::BinOp(
            BinOp::BoolOp(BoolOp::And),
            Box::new(SignalExpr::Not(Box::new(expr.clone()))), // false
            Box::new(SignalExpr::BinOp(
                BinOp::BoolOp(BoolOp::Or),
                Box::new(SignalExpr::Constant(false.into())), // false
                Box::new(expr.clone()),                       // true
            )),
        );
        println!("eval.expr2 {:?}", expr2.eval(&simulator));
        assert_eq!(Ok(false), expr2.eval(&simulator));
    }

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
