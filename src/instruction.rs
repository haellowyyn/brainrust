#[derive(Debug)]
pub enum Instruction {
    // (>) Increment the data pointer.
    Next,
    // (<) Decrement the data pointer.
    Prev,
    // (+) Increment the byte at the data pointer.
    Inc,
    // (-) Decrement the byte at the data pointer.
    Dec,
    // (.) Output the byte at the data pointer.
    Put,
    // (,) Accept one byte of input, store at it the data pointer.
    Get,
    // ([) If the byte at the data pointer is zero, skip forward to the matching Loop.
    Skip,
    // (]) If the byte at the data pointer is nonzero, jump back to the matching Skip.
    Loop,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            b'>' => Some(Instruction::Next),
            b'<' => Some(Instruction::Prev),
            b'+' => Some(Instruction::Inc),
            b'-' => Some(Instruction::Dec),
            b'.' => Some(Instruction::Put),
            b',' => Some(Instruction::Get),
            b'[' => Some(Instruction::Skip),
            b']' => Some(Instruction::Loop),
            _ => None,
        }
    }
}
