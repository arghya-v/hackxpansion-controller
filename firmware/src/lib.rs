#![no_std]

use embassy_rp::{
    peripherals::{DMA_CH0, DMA_CH1, SPI0},
    spi,
};

use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::sdcard::spi::SdCard;


use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    interfaces::buttons::{A, B, X, pin_button},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    registry::Registry,
};

pub struct SdCardDriver;

impl DriverMeta for SdCardDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K,
        md1: ModuleDetectResistor::R30K,
    };
}

impl<G: BankPins> Driver<G> for SdCardDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {

        registry.register(
            slot,
            SdCardDriver::ID,
            pin_button::<A>(gpio_bank.gpio5.into()),
        );

        registry.register(
            slot,
            SdCardDriver::ID,
            pin_button::<B>(gpio_bank.gpio6.into()),
        );

        registry.register(
            slot,
            SdCardDriver::ID,
            pin_button::<X>(gpio_bank.gpio7.into()),
        );

    
        let spi = bus_allocator
            .create_spi_hardware::<SPI0, DMA_CH0, DMA_CH1, _>(
                gpio_bank.gpio2,
                gpio_bank.gpio4,
                gpio_bank.gpio3,
                Irqs,
                spi::Config::default(),
            )
            .map_err(|_| DriverError::PeripheralSetupFailed)?;

        let cs = gpio_bank.gpio9;

        // SD card initialization coming soon

        Ok(())
    }
}