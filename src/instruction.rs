use self::Instruction::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Instruction {
    /// (>) Increment the data pointer.
    Next,
    /// (<) Decrement the data pointer.
    Prev,
    /// (+) Increment the byte at the data pointer.
    Inc,
    /// (-) Decrement the byte at the data pointer.
    Dec,
    /// (.) Output the byte at the data pointer.
    Put,
    /// (,) Accept one byte of input, store at it the data pointer.
    Get,
    /// ([) If the byte at the data pointer is zero, skip forward to the matching Loop.
    Skip,
    /// (]) If the byte at the data pointer is nonzero, jump back to the matching Skip.
    Loop,

    // The following instructions are only used for optimization and have no source code
    // representation.
    /// Increment the data pointer by n.
    Fwd(usize),
    /// Decrement the data pointer by n.
    Bwd(usize),
    /// Add n to the byte at the data pointer.
    Add(u8),
    /// Subtract n from the byte at the data pointer.
    Sub(u8),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            b'>' => Some(Next),
            b'<' => Some(Prev),
            b'+' => Some(Inc),
            b'-' => Some(Dec),
            b'.' => Some(Put),
            b',' => Some(Get),
            b'[' => Some(Skip),
            b']' => Some(Loop),
            _ => None,
        }
    }

    pub fn is_accumulatable(&self) -> bool {
        match *self {
            Next | Prev | Inc | Dec => true,
            _ => false,
        }
    }

    pub fn to_accumulated(&self, count: usize) -> Option<Instruction> {
        match *self {
            Next => Some(Fwd(count)),
            Prev => Some(Bwd(count)),
            Inc => Some(Add(count as u8)),
            Dec => Some(Sub(count as u8)),
            _ => None,
        }
    }
}
