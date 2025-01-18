use crate::{reg::*, LcrConfig};
use core::fmt::Write;

use lego_device::{
    read_reg, write_reg, CharDevInfo, CharDevice, Device, DeviceError, DeviceStatus, DeviceType,
};
#[derive(Debug)]
pub struct Uart {
    base_address: usize,
    status: DeviceStatus,
    config: LcrConfig,
}

impl Uart {
    pub const fn new(base_address: usize, div: u8) -> Self {
        Self {
            base_address,
            status: DeviceStatus::Uninitialized,
            config: LcrConfig::default_config(div),
        }
    }
}

impl Device for Uart {
    fn init(&mut self) -> Result<(), DeviceError> {
        let config = self.config;
        write_reg::<u8>(self.base_address, LCR, config.to_u8(1));
        write_reg::<u8>(self.base_address, FCR, Fcr::enable.bits());
        write_reg::<u16>(self.base_address, DLL, config.divisor as u16);
        write_reg::<u8>(self.base_address, LCR, config.to_u8(0));
        write_reg::<u8>(self.base_address, IER, 1);
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
        if lsr.contains(Lsr::thre) {
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

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ele in s.as_bytes() {
            self.put_char(*ele).unwrap();
        }
        Ok(())
    }
}

unsafe impl Sync for Uart {}
