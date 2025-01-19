use bitflags::bitflags;
use lego_device::reg_map;
reg_map!(
    RBR 0,
    THR 0,
    DLL 0,
    DLH 0x4,
    IER 0x4,
    IIR 0x8,
    FCR 0x8,
    LCR 0xC,
    MCR 0x10,
    LSR 0x14,
    MSR 0x18,
    SCR 0x1C
);

bitflags! {
    #[derive(Debug,Clone, Copy)]
    pub struct Lsr:u8{
        const data_ready =0x1;
        const overrun = 0x1 << 1;
        const parity_error = 0x1 << 2;
        const frame_error = 0x1 << 3;
        const brk_interrupt = 0x1 << 4;
        const thre = 0x1 << 5;
        const temt = 0x1 << 6;
        const rcvr_fifo_error = 0x1 <<7;
    }

    #[derive(Debug,Clone, Copy)]
    pub struct Ier:u8{
        const rdai =0x1;
        const threi = 0x1 << 1;
        const rlsi = 0x1 << 2;
        const msi = 0x1 << 3;
        const alc = 0x1 << 4;
        const ptime = 0x1 << 7;
    }

    #[derive(Debug,Clone, Copy)]
    pub struct Iir:u8{
        const id = 0xF;
        const fifo_enabled = 0b11 << 6;
    }
}

pub(crate) const FCR_FIFO: u8 = 0x01;
pub(crate) const FCR_RCVRCLR: u8 = 0x02;
pub(crate) const FCR_XMITCLR: u8 = 0x04;
pub(crate) const FCR_FIFO_8: u8 = 0x80;
