use core::fmt::Write;

use crate::{reg::*, UartConfig};

use lego_device::{
    read_reg, write_reg, CharDevInfo, CharDevice, Device, DeviceError, DeviceStatus, DeviceType,
};
#[derive(Debug)]
pub struct Uart {
    base_address: usize,
    status: DeviceStatus,
    config: UartConfig,
}

impl Uart {
    pub const fn new(base_address: usize, div: u8) -> Self {
        Self {
            base_address,
            status: DeviceStatus::Uninitialized,
            config: UartConfig::uart8250(div),
        }
    }
}

impl Device for Uart {
    fn init(&mut self) -> Result<(), DeviceError> {
        let lcr = (self.base_address + 3) as *mut u8;
        let config = self.config;
        unsafe {
            lcr.write_volatile(
                config.word_len as u8
                    | ((config.stop_bits as u8) << 2)
                    | ((config.parity_bit as u8) << 3)
                    | ((config.parity_select as u8) << 4)
                    | ((config.brk as u8) << 6)
                    | 1 << 7,
            );
        }

        let dll = (self.base_address) as *mut u8;
        unsafe {
            dll.write_volatile(13);
        }
        unsafe {
            lcr.write_volatile(
                config.word_len as u8
                    | ((config.stop_bits as u8) << 2)
                    | ((config.parity_bit as u8) << 3)
                    | ((config.parity_select as u8) << 4)
                    | ((config.brk as u8) << 6),
            );
        }
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
            let ch = read_reg::<u8>(self.base_address, RBR);
            Ok(ch)
        } else {
            Err(DeviceError::DeviceBusy)
        }
    }

    fn put_char(&self, ch: u8) -> core::result::Result<(), DeviceError> {
        while read_reg::<u8>(self.base_address, LSR) & 0x20 == 0 {}
        write_reg::<u8>(self.base_address, THR, ch);
        Ok(())
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
