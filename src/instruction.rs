use self::Instruction::*;

#[derive(Debug)]
#[derive(Clone)]
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
}
