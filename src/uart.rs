use crate::{reg::*, LcrConfig, DLAB};

use lego_device::{
    read_reg, write_reg, CharDevInfo, CharDevice, Device, DeviceError, DeviceStatus, DeviceType,
};
#[derive(Debug)]
pub struct Uart {
    base_address: usize,
    status: DeviceStatus,
    baud_rate: u64,
    clk_hz: u64,
}

impl Uart {
    pub const fn new(base_address: usize, clk_hz: u64, baud_rate: u64) -> Self {
        Self {
            base_address,
            status: DeviceStatus::Uninitialized,
            baud_rate,
            clk_hz,
        }
    }
}

impl Device for Uart {
    fn init(&mut self) -> Result<(), DeviceError> {
        write_reg::<u8>(self.base_address, IER, 0);
        let config = LcrConfig::default_config();
        write_reg::<u8>(self.base_address, LCR, config.get_value(DLAB::Enable));
        let divisor = self.clk_hz / (self.baud_rate << 4);
        write_reg::<u8>(self.base_address, DLL, (divisor & 0xff) as u8);
        write_reg::<u8>(self.base_address, DLH, ((divisor >> 8) & 0xff) as u8);
        write_reg::<u8>(self.base_address, LCR, config.get_value(DLAB::Disable));
        write_reg::<u8>(
            self.base_address,
            FCR,
            FCR_FIFO | FCR_FIFO_8 | FCR_RCVRCLR | FCR_XMITCLR,
        );
        let mcr = read_reg::<u8>(self.base_address, MCR) & !0x1F;
        write_reg::<u8>(self.base_address, MCR, mcr | 0b11);
        write_reg::<u8>(self.base_address, IER, 0x1);
        Ok(())
    }

    fn close(&mut self) -> Result<(), DeviceError> {
        self.status = DeviceStatus::Suspended;
        Ok(())
    }

    fn status(&self) -> DeviceStatus {
        DeviceStatus::Transfer
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Char
    }

    fn error_handle(&self) -> DeviceStatus {
        DeviceStatus::Transfer
    }

    fn reinit(&mut self) -> Result<(), DeviceError> {
        self.init()
    }
}

impl CharDevice for Uart {
    fn get_char(&self) -> core::result::Result<u8, DeviceError> {
        let lsr = Lsr::from_bits(read_reg::<u8>(self.base_address, LSR)).unwrap();
        if lsr.contains(Lsr::data_ready) {
            Ok(read_reg::<u8>(self.base_address, RBR))
        } else {
            Err(DeviceError::DeviceBusy)
        }
    }

    fn put_char(&self, ch: u8) -> core::result::Result<(), DeviceError> {
        let lsr = Lsr::from_bits(read_reg::<u8>(self.base_address, LSR)).unwrap();
        if lsr.contains(Lsr::thre) {
            write_reg::<u8>(self.base_address, THR, ch);
            Ok(())
        } else {
            Err(DeviceError::DeviceBusy)
        }
    }

    fn information(&self) -> &dyn CharDevInfo {
        todo!()
    }
}

unsafe impl Sync for Uart {}
