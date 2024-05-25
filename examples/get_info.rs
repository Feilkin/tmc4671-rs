use anyhow::Result;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::{CdevPin, SpidevDevice, SysfsPin};
use std::time::Duration;
use tmc4671_rs::spi::constants::CHIP_INFO_ADDRESS;
use tmc4671_rs::Tmc4671;

fn main() -> Result<()> {
    let mut chip = Chip::new("/dev/gpiochip4")?;
    let eni_line = chip.get_line(23)?;
    let eni_handle = eni_line.request(LineRequestFlags::OUTPUT, 0, "TMC driver")?;
    let eni = CdevPin::new(eni_handle)?;

    let mut spi = SpidevDevice::open("/dev/spidev0.0")?;

    spi.configure(&SpidevOptions {
        bits_per_word: None,
        max_speed_hz: Some(1_000_000),
        lsb_first: None,
        spi_mode: Some(SpiModeFlags::SPI_MODE_3),
    })
    .expect("failed to configure SPI device");

    let mut tmc = Tmc4671::new(spi);

    loop {
        eni.set_value(1)?;
        let si_type = tmc.get_chip_info(CHIP_INFO_ADDRESS::SI_TYPE)?;

        eni.set_value(0)?;

        let type_bytes = si_type.to_be_bytes();
        let si_type_str = String::from_utf8_lossy(&type_bytes);

        println!("SI_TYPE:\t{si_type_str}");

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
