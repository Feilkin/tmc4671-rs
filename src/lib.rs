//! Crate for commanding the TMC4671 FOC IC over SPI
use crate::spi::Datagram;
use embedded_hal::spi::{Error, SpiDevice};
use nom::Finish;
use thiserror::Error;

pub struct Tmc4671<SPI: SpiDevice> {
    spi_device: SPI,
}

impl<SPI: SpiDevice> Tmc4671<SPI> {
    pub fn new(spi_device: SPI) -> Self {
        Tmc4671 { spi_device }
    }
}

impl<SPI: SpiDevice> Tmc4671<SPI> {
    pub fn get_chip_info(
        &mut self,
        info: spi::constants::CHIP_INFO_ADDRESS,
    ) -> Result<u32, Tmc4671Error> {
        self.write_register(spi::registers::CHIPINFO_ADDR, info as u32)?;
        self.read_register(spi::registers::CHIPINFO_DATA)
    }
}

impl<SPI: SpiDevice> Tmc4671<SPI> {
    pub fn read_register(&mut self, register: u8) -> Result<u32, Tmc4671Error> {
        let datagram = Datagram {
            write_not_read: false,
            address: register,
            data: 0x00_00_00_00,
        };

        let received_datagram = self.transfer_datagram(datagram)?;

        Ok(received_datagram.data)
    }

    fn transfer_datagram(&mut self, datagram: Datagram) -> Result<Datagram, Tmc4671Error> {
        let mut buffer = datagram.bytes();

        self.spi_device
            .transfer_in_place(&mut buffer)
            .map_err(|err| Tmc4671Error::CommunicationError(err.kind()))?;

        let (_, received_datagram) = Datagram::parse(&buffer)
            .finish()
            .map_err(|_err| Tmc4671Error::ParseError)?;

        // debug_assert_eq!(datagram.address, received_datagram.address);
        Ok(received_datagram)
    }

    pub fn write_register(&mut self, register: u8, data: u32) -> Result<(), Tmc4671Error> {
        let datagram = Datagram {
            write_not_read: true,
            address: register,
            data,
        };

        let _received_datagram = self.transfer_datagram(datagram)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Tmc4671Error {
    #[error("failed to parse data")]
    ParseError,
    #[error("SPI communication failed")]
    CommunicationError(embedded_hal::spi::ErrorKind),
}

pub mod spi {
    use nom::IResult;

    pub const ADDR_WRITE_BIT: u8 = 0b1000_0000;

    #[derive(Debug, Copy, Clone)]
    pub struct Datagram {
        pub write_not_read: bool,
        pub address: u8,
        pub data: u32,
    }

    impl Datagram {
        pub fn parse(input: &[u8]) -> IResult<&[u8], Datagram> {
            let (tail, address_and_write_not_read) = nom::number::streaming::u8(input)?;
            let (tail, data) = nom::number::streaming::be_u32(tail)?;

            let datagram = Datagram {
                write_not_read: (address_and_write_not_read & ADDR_WRITE_BIT) != 0,
                address: address_and_write_not_read & !ADDR_WRITE_BIT,
                data,
            };

            Ok((tail, datagram))
        }

        pub fn bytes(&self) -> [u8; 5] {
            let mut out = [0u8; 5];

            out[0] = self.address;
            if self.write_not_read {
                out[0] |= ADDR_WRITE_BIT;
            }

            out[1..5].copy_from_slice(&self.data.to_be_bytes());

            out
        }
    }

    pub mod registers {
        // Register names and function descriptions taken from TMC4671-LA datasheet.
        // ©2022 TRINAMIC Motion Control GmbH & Co. KG, Hamburg, Germany

        /// This register displays name and version information of the accessed IC.
        /// It can be used for test of communication.
        pub const CHIPINFO_DATA: u8 = 0x00;
        /// This register is used to change displayed information in register CHIPINFO_DATA.
        pub const CHIPINFO_ADDR: u8 = 0x01;
        /// This registers displays ADC values.
        /// The displayed registers can be switched by register ADC_RAW_ADDR.
        pub const ADC_RAW_DATA: u8 = 0x02;
        pub const ADC_RAW_ADDR: u8 = 0x03;
        pub const dsADC_MCFG_B_MCFG_A: u8 = 0x04;
        pub const dsADC_MCLK_A: u8 = 0x05;
        pub const dsADC_MCLK_B: u8 = 0x06;
        pub const dsADC_MDEC_B_MDEC_A: u8 = 0x07;
        pub const ADC_I1_SCALE_OFFSET: u8 = 0x08;
        pub const ADC_I0_SCALE_OFFSET: u8 = 0x09;
        pub const ADC_I_SELECT: u8 = 0x0A;
        pub const ADC_I1_I0_EXT: u8 = 0x0B;
        pub const DS_ANALOG_INPUT_STAGE_CFG: u8 = 0x0C;
        pub const AENC_0_SCALE_OFFSET: u8 = 0x0D;
        pub const AENC_1_SCALE_OFFSET: u8 = 0x0E;
        pub const AENC_2_SCALE_OFFSET: u8 = 0x0F;
        pub const AENC_SELECT: u8 = 0x11;
        pub const ADC_IWY_IUX: u8 = 0x12;
        pub const ADC_IV: u8 = 0x13;
        pub const AENC_WY_UX: u8 = 0x15;
        pub const AENC_VN: u8 = 0x16;
        pub const PWM_POLARITIES: u8 = 0x17;
        pub const PWM_MAXCNT: u8 = 0x18;
        pub const PWM_BBM_H_BBM_L: u8 = 0x19;
        pub const PWM_SV_CHOP: u8 = 0x1A;
        pub const MOTOR_TYPE_N_POLE_PAIRS: u8 = 0x1B;
        pub const PHI_E_EXT: u8 = 0x1C;
        pub const OPENLOOP_MODE: u8 = 0x1F;
        pub const OPENLOOP_ACCELERATION: u8 = 0x20;
        pub const OPENLOOP_VELOCITY_TARGET: u8 = 0x21;
        pub const OPENLOOP_VELOCITY_ACTUAL: u8 = 0x22;
        pub const OPENLOOP_PHI: u8 = 0x23;
        pub const UQ_UD_EXT: u8 = 0x24;
        pub const ABN_DECODER_MODE: u8 = 0x25;
        pub const ABN_DECODER_PPR: u8 = 0x26;
        pub const ABN_DECODER_COUNT: u8 = 0x27;
        pub const ABN_DECODER_COUNT_N: u8 = 0x28;
        pub const ABN_DECODER_PHI_E_PHI_M_OFFSET: u8 = 0x29;
        pub const ABN_DECODER_PHI_E_PHI_M: u8 = 0x2A;
        pub const ABN_2_DECODER_MODE: u8 = 0x2C;
        pub const ABN_2_DECODER_PPR: u8 = 0x2D;
        pub const ABN_2_DECODER_COUNT: u8 = 0x2E;
        pub const ABN_2_DECODER_COUNT_N: u8 = 0x2F;
        pub const ABN_2_DECODER_PHI_M_OFFSET: u8 = 0x30;
        pub const ABN_2_DECODER_PHI_M: u8 = 0x31;
        pub const HALL_MODE: u8 = 0x33;
        pub const HALL_POSITION_060_000: u8 = 0x34;
        pub const HALL_POSITION_180_120: u8 = 0x35;
        pub const HALL_POSITION_300_240: u8 = 0x36;
        pub const HALL_PHI_E_PHI_M_OFFSET: u8 = 0x37;
        pub const HALL_DPHI_MAX: u8 = 0x38;
        pub const HALL_PHI_E_INTERPOLATED_PHI_E: u8 = 0x39;
        pub const HALL_PHI_M: u8 = 0x3A;
        pub const AENC_DECODER_MODE: u8 = 0x3B;
        pub const AENC_DECODER_N_THRESHOLD: u8 = 0x3C;
        pub const AENC_DECODER_PHI_A_RAW: u8 = 0x3D;
        pub const AENC_DECODER_PHI_A_OFFSET: u8 = 0x3E;
        pub const AENC_DECODER_PHI_A: u8 = 0x3F;
        pub const AENC_DECODER_PPR: u8 = 0x40;
        pub const AENC_DECODER_COUNT: u8 = 0x41;
        pub const AENC_DECODER_COUNT_N: u8 = 0x42;
        pub const AENC_DECODER_PHI_E_PHI_M_OFFSET: u8 = 0x45;
        pub const AENC_DECODER_PHI_E_PHI_M: u8 = 0x46;
        pub const CONFIG_DATA: u8 = 0x4D;
        pub const CONFIG_ADDR: u8 = 0x4E;
        pub const VELOCITY_SELECTION: u8 = 0x50;
        pub const POSITION_SELECTION: u8 = 0x51;
        pub const PHI_E_SELECTION: u8 = 0x52;
        pub const PHI_E: u8 = 0x53;
        pub const PID_FLUX_P_FLUX_I: u8 = 0x54;
        pub const PID_TORQUE_P_TORQUE_I: u8 = 0x56;
        pub const PID_VELOCITY_P_VELOCITY_I: u8 = 0x58;
        pub const PID_POSITION_P_POSITION_I: u8 = 0x5A;
        pub const PIDOUT_UQ_UD_LIMITS: u8 = 0x5D;
        pub const PID_TORQUE_FLUX_LIMITS: u8 = 0x5E;
        pub const PID_VELOCITY_LIMIT: u8 = 0x60;
        pub const PID_POSITION_LIMIT_LOW: u8 = 0x61;
        pub const PID_POSITION_LIMIT_HIGH: u8 = 0x61;
        pub const MODE_RAMP_MODE_MOTION: u8 = 0x63;
        pub const PID_TORQUE_FLUX_TARGET: u8 = 0x64;
        pub const PID_TORQUE_FLUX_OFFSET: u8 = 0x65;
        pub const PID_VELOCITY_TARGET: u8 = 0x66;
        pub const PID_VELOCITY_OFFSET: u8 = 0x67;
        pub const PID_POSITION_TARGET: u8 = 0x68;
        pub const PID_TORQUE_FLUX_ACTUAL: u8 = 0x69;
        pub const PID_VELOCITY_ACTUAL: u8 = 0x6A;
        pub const PID_POSITION_ACTUAL: u8 = 0x6B;
        pub const PID_ERROR_DATA: u8 = 0x6C;
        pub const PID_ERROR_ADDR: u8 = 0x6D;
        pub const INTERIM_DATA: u8 = 0x6E;
        pub const INTERIM_ADDR: u8 = 0x6F;
        pub const ADC_VM_LIMITS: u8 = 0x75;
        pub const TMC4671_INPUTS_RAW: u8 = 0x76;
        pub const TMC4671_OUTPUTS_RAW: u8 = 0x77;
        pub const STEP_WIDTH: u8 = 0x78;
        pub const UART_BPS: u8 = 0x79;
        pub const GPIO_dsADCI_CONFIG: u8 = 0x7B;
        pub const STATUS_FLAGS: u8 = 0x7C;
        pub const STATUS_MASK: u8 = 0x7D;
    }

    pub mod bit_masks {}

    pub mod constants {

        #[repr(u32)]
        pub enum CHIP_INFO_ADDRESS {
            SI_TYPE = 0x00_00_00_00,
            SI_VERSION = 0x00_00_00_01,
            SI_DATE = 0x00_00_00_02,
            SI_TIME = 0x00_00_00_03,
            SI_VARIANT = 0x00_00_00_04,
            SI_BUIlD = 0x00_00_00_05,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
