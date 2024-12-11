use bitflags::bitflags;
use lego_device::reg_map;
reg_map!(
    RBR 0,
    DLL 0,
    THR 0,
    DLH 0x4,
    IER 0x4,
    FCR 0x8,
    IIR 0x8,
    LCR 0xC,
    MCR 0x10,
    LSR 0x14,
    MSR 0x18,
    SCR 0x1C
);

bitflags! {
    #[derive(Debug,Clone, Copy)]
    pub struct Lcr:u8{
        const word_len =0x11;
        const stop_bits = 0x1 << 2;
        const parity = 0x1 << 3;
        const parity_select = 0x1 << 4;
        const stick_parity = 0x1 << 5;
        const brk = 0x1 << 6;
        const div_latch = 0x1 << 7;
    }
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

    #[derive(Debug,Clone, Copy)]
    pub struct Fcr:u8{
        const enable =0x1;
        const rc_reset = 0x1 << 1;
        const ts_reset = 0x1 << 2;
        const dma_mode = 0x1 << 3;
        const ts_trigger = 0b11 << 4;
        const rc_trigger = 0b11 << 6;
    }
}
