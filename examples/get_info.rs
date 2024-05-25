use anyhow::Result;
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::SpidevDevice;
use tmc4671_rs::spi::constants::CHIP_INFO_ADDRESS;
use tmc4671_rs::Tmc4671;

#[tokio::main]
async fn main() -> Result<()> {
    let mut spi = SpidevDevice::open("/dev/spidev0.0")?;

    spi.configure(&SpidevOptions {
        bits_per_word: None,
        max_speed_hz: Some(8_000_000),
        lsb_first: None,
        spi_mode: Some(SpiModeFlags::SPI_MODE_3),
    })
    .except("failed to configure SPI device");

    let mut tmc = Tmc4671::new(spi);

    let si_type = tmc.get_chip_info(CHIP_INFO_ADDRESS::SI_TYPE)?;

    let type_bytes = si_type.to_le_bytes();
    let si_type_str = String::from_utf8_lossy(&type_bytes);

    println!("SI_TYPE:\t{si_type_str}");

    Ok(())
}
