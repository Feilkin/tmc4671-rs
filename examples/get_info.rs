use anyhow::Result;
use linux_embedded_hal::SpidevDevice;
use tmc4671_rs::spi::constants::CHIP_INFO_ADDRESS;
use tmc4671_rs::Tmc4671;

#[tokio::main]
async fn main() -> Result<()> {
    let spi = SpidevDevice::open("/dev/spidev0.0")?;

    let mut tmc = Tmc4671::new(spi);

    let si_type = tmc.get_chip_info(CHIP_INFO_ADDRESS::SI_TYPE).await?;

    let si_type_str = String::from_utf8_lossy(&si_type.to_le_bytes());

    println!("SI_TYPE:\t{si_type_str}");

    Ok(())
}
