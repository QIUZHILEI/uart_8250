#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Word length
pub enum WordLength {
    FIVE = 0,
    SIX = 1,
    SEVEN = 2,
    EIGHT = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Number of stop bits
pub enum StopBits {
    ONE = 0,
    TWO = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity bits
pub enum Parity {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity select
pub enum ParitySelect {
    EVEN = 0,
    ODD = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Stick parity
pub enum StickParity {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Break
pub enum Break {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct UartConfig {
    pub word_len: WordLength,
    pub stop_bits: StopBits,
    pub parity_bit: Parity,
    pub parity_select: ParitySelect,
    pub brk: Break,
    pub divisor: u8,
}

impl UartConfig {
    pub const fn uart8250(div: u8) -> Self {
        Self {
            word_len: WordLength::EIGHT,
            stop_bits: StopBits::ONE,
            parity_bit: Parity::DISABLE,
            parity_select: ParitySelect::EVEN,
            brk: Break::DISABLE,
            divisor: div,
        }
    }
}
