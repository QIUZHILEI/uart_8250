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
pub struct LcrConfig {
    word_len: WordLength,
    stop_bits: StopBits,
    parity_bit: Parity,
    parity_select: ParitySelect,
    stick_parity: StickParity,
    brk: Break,
    pub(crate) divisor: u8,
}

impl LcrConfig {
    pub const fn default_config(div: u8) -> Self {
        Self {
            word_len: WordLength::EIGHT,
            stop_bits: StopBits::ONE,
            parity_bit: Parity::DISABLE,
            parity_select: ParitySelect::EVEN,
            stick_parity: StickParity::DISABLE,
            brk: Break::DISABLE,
            divisor: div,
        }
    }

    pub(crate) fn to_u8(&self, dlab: u8) -> u8 {
        self.word_len as u8
            | ((self.stop_bits as u8) << 2)
            | ((self.parity_bit as u8) << 3)
            | ((self.parity_select as u8) << 4)
            | ((self.stick_parity as u8) << 5)
            | ((self.brk as u8) << 6)
            | dlab << 7
    }
}
