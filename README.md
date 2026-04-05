📟 hd44780-i2c-nostd
🦅 Version 0.1.0 
A Robust, High-Performance HD44780 Driver for Rust (no_std). Optimized for Embassy and embedded systems like RP2040 (Pico), Pico 2, STM32, and ESP32.

Created by Jorge Andre Castro.

🛡️ The Mission
hd44780-i2c-nostd provides a reliable way to drive classic LCD displays via the PCF8574 I2C expander. This crate is licensed under GPL-2.0-or-later to ensure that fundamental hardware drivers remain a common good and are never locked away in proprietary blobs.

![alt text](image.png)

🚀 Key Features
True Async Native: Built from the ground up for embedded-hal-async. No blocking loops, no CPU wastage.

Zero-Copy Efficiency: Optimized I2C transactions. We pulse the Enable pin by grouping High/Low states in a single buffer to saturate the bus efficiently.

no_std & Bare-Metal: Perfect for Embassy, RTIC, or custom kernels. Zero dependency on the standard library.

Anti-Glitch Initialization: Implements the official HD44780 4-bit initialization sequence with precise hardware delays to ensure a "Clean Boot" every time.

Flexible Layouts: Supports 16x2, 20x4, and other standard character LCD geometries.

📋 Changelog & Updates
🦅 Version 0.1.0 - The "Async Foundation"
Feature: Full asynchronous support via I2c and DelayNs traits.

Feature: Integrated Cursor Management and Backlight control.

Optimization: Single-transaction nibble writing to reduce I2C overhead.

🛠️ Usage
Installation

Ini, TOML
[dependencies]
hd44780-i2c-nostd = "0.1.0"


💡 Quick Start
Rust
use hd44780_i2c_nostd::LcdI2c;
use embassy_time::Delay;

// 1. Initialize your I2C peripheral (Embassy RP2040 example)
// let i2c = I2c::new(p.I2C0, p.PIN_1, p.PIN_0, Irqs, Config::default());

// 2. Create the LCD instance (Address 0x27 is common)
let mut lcd = LcdI2c::new(i2c, 0x27);

// 3. Initialisation with a delay provider
lcd.init(&mut Delay).await.unwrap();

// 4. Write your data
lcd.set_cursor(0, 0).await.ok();
lcd.write_str("Project of my life").await.ok();

// 5. Toggle Backlight
lcd.set_backlight(true);
🎮 Example: Real-time Telemetry
Rust
// In your main loop, display PID data or sensor values
loop {
    let temp = sensor.read_temp().await;
    lcd.set_cursor(1, 0).await.ok();
    
    // Pro-tip: use core::fmt with a small buffer for dynamic strings
    let mut buf = [0u8; 16];
    if let Ok(s) = format_no_std(&mut buf, format_args!("Temp: {:.2}C", temp)) {
        lcd.write_str(s).await.ok();
    }
    
    Timer::after_millis(500).await;
}
⚖️ License
This project is licensed under the GNU General Public License v2.0 or later.

You are free to use it, but the freedom of the code must be respected. Any improvements made to this driver MUST be shared back with the community.

🦅 Why use this?
Because in the "Project of your life", you cannot afford a driver that hangs or uses legacy blocking code. hd44780-i2c-nostd is designed to be the invisible, robust bridge between your logic and your user interface.