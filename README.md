# JH71xx-HAL

`jh71xx-hal` is a HAL (hardware abstraction layer) crate for `JH71xx`-based SoCs from StarFive.

Currently, only the `JH7110` SoC is supported. `JH7100` appears to be discontinued, and the next SoC line from StarFive will be the `JH81xx` series.

Please submit changes if you would like to add support for the `JH7100` SoC!

## Usage

### GPIO

GPIO configuration and access is fully supported:

```rust
use jh71xx_hal::{pac, gpio};
use embedded_hal::digital::{InputPin, OutputPin};

let dp = pac::Peripherals::take().unwrap();
let gpio0 = gpio::get_gpio(dp.SYS_PINCTRL.gpio_0());

// Configure as an enabled output
let mut gpio0_out = gpio0.into_enabled_output();

// Drive pin high
gpio0_out.set_high();
// Drive pin low
gpio0_out.set_low();

// Configure as an enabled input
let gpio0_in = gpio0_out.into_enabled_input();

// Configure as high-impedance input
let gpio0_in_high_z = gpio0_in.into_input_high_z();
// Configure as pull-up input
let gpio0_in_pull_up = gpio0_in_high_z.into_input_pull_up();
// Configure as pull-down input
let mut gpio0_in_pull_down = gpio0_in_pull_up.into_input_pull_down();

// Is pin low?
if gpio0_in_pull_down.is_low() {
    // do interesting GPIO stuff
}

// Is pin high?
if gpio0_in_pull_down.is_high() {
    // do interesting GPIO stuff
}
```

#### WIP: GPIO

`JH7110` SoCs use a pin multiplexer to configure pins for specialized functionality (I2C, SPI, etc.).

Work is on-going to provide high-level interfaces to configure specialized functions for GPIO pins.

Low-level configuration can currently be achieved through the `jh71xx-pac` crate which is re-exported as `jh71xx_hal::pac`.

### I2C

I2C configuration and access is fully supported.

```rust
use embedded_hal::i2c::{I2c as _, Operation};
use jh71xx_hal::{pac, i2c};

let dp = pac::Peripherals::take().unwrap();
let mut i2c0 = i2c::I2c::new(dp.I2C0);

// 7-bit address
let addr: u8 = 1;
let mut read_buf = [0u8; 1];
let ops = Operation::Read(&mut read_buf);

i2c0.transaction(addr, &mut [ops]).unwrap(); 

// 10-bit address
let addr: u16 = 1;

i2c0.transaction(addr, &mut [ops]).unwrap();
```

### SPI

SPI configuration and access is fully supported.

```rust
use embedded_hal::spi::SpiBus;
use jh71xx_hal::{pac, spi};

let dp = pac::Peripherals::take().unwrap();

// 8-bit transactions
let mut spi0 = spi::Spi::<pac::SPI0, 8>::new(dp.SPI0).unwrap();
let mut read_buf = [0u8; 1];
let write_buf = [0u8; 1];

// Read and write as separate transactions
spi0.read(read_buf.as_mut()).unwrap();
spi0.write(write_buf.as_ref()).unwrap();

// Read and write in the same call
spi0.transfer(read_buf.as_mut(), write_buf.as_ref()).unwrap();
// Write and read from the same buffer
// NOTE: writes happen first, since the read overwrites the buffer
spi0.transfer_in_place(read_buf.as_mut()).unwrap();

// Flushes read/write FIFOs, and waits for peripheral to become idle
spi0.flush().unwrap();

// 16-bit transactions
let mut spi1 = spi::Spi::<pac::SPI1, 16>::new(dp.SPI1).unwrap();
let mut read_buf = [0u16; 1];
let write_buf = [0u16; 1];

// Read and write as separate transactions
spi1.read(read_buf.as_mut()).unwrap();
spi1.write(write_buf.as_ref()).unwrap();

// Read and write in the same call
spi1.transfer(read_buf.as_mut(), write_buf.as_ref()).unwrap();
// Write and read from the same buffer
// NOTE: writes happen first, since the read overwrites the buffer
spi1.transfer_in_place(read_buf.as_mut()).unwrap();

// Flushes read/write FIFOs, and waits for peripheral to become idle
spi1.flush().unwrap();
```

#### WIP: SPI

Currently, only 8- and 16-bit transfers are supported. The peripheral in the SoC supports 4- to 16-bit transfers.

TBD: should the interface support:

- packed data transfers for efficiency (but increased complexity)
- unpacked transfers for simplicity (but reduced efficiency)

Either way, the additional data sizes could be supported without breaking changes to the current API.

The [ARM pl022 SSP SPI](https://documentation-service.arm.com/static/5e8e3b2afd977155116a92f7&rut=3d45d778b3f2b62fe659ebfb50905914d913d289f017585fb1c8e07383ea508a) peripheral also supports "Slave" mode, which is outside the `embedded-hal` traits, but could still be useful to `jh71xx-hal` users.

Similarly, the peripheral supports the Texas Instruments Synchronous Serial and Microwire serial frame formats (currently unsupported).

### PWM

PWM configuration and access is fully supported.

```rust
use embedded_hal::pwm::SetDutyCycle;
use jh71xx_hal::{pac, pwm};

let dp = pac::Peripherals::take().unwrap();
let mut pwm0 = pwm::Pwm::new(dp.PWM);

// Gets the maximum duty cycle
let max_cycle = pwm0.max_duty_cycle();
// Sets the PWM peripheral to a ~50% duty cycle
pwm0.set_duty_cycle(max_cycle / 2).unwrap();
```
