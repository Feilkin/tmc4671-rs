#![cfg(unix)]
use embedded_hal_async::spi::Operation::DelayNs;
use embedded_hal_bus::spi::AtomicDevice;
use embedded_hal_bus::util::AtomicCell;
use rppal::gpio;
use rppal::spi;
use tmc4671_rs::spi::constants::CHIP_INFO_ADDRESS;
use tmc4671_rs::Tmc4671;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let spi_bus = spi::Spi::new(
        spi::Bus::Spi0,
        spi::SlaveSelect::Ss15,
        8_000_000,
        spi::Mode::Mode3,
    )?;
    let bus_in_cell = AtomicCell::new(spi_bus);

    let gpio = gpio::Gpio::new()?;
    let mut cs_pin = gpio.get(24)?.into_output();
    cs_pin.set_high();

    let mut tmc = Tmc4671::new(AtomicDevice::new(&bus_in_cell, cs_pin, DelayNs(500))?);

    let si_type = tmc.get_chip_info(CHIP_INFO_ADDRESS::SI_TYPE).await?;

    let si_type_str = String::from_utf8_lossy(&si_type.to_le_bytes());

    println!("SI_TYPE:\t{si_type_str}");

    Ok(())
}
