// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.

#![no_std] // We use no_std to ensure our library can be used in embedded environments.

//! A robust async driver for HD44780 LCD displays via PCF8574 I2C expander.
//! Optimized for Embassy and designed for reliability in critical embedded systems.

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Standard HD44780 commands
const CMD_CLEAR: u8 = 0x01;
const CMD_RETURN_HOME: u8 = 0x02;
const CMD_ENTRY_MODE: u8 = 0x04;
const CMD_DISPLAY_CONTROL: u8 = 0x08;
const CMD_FUNCTION_SET: u8 = 0x20;

/// PCF8574 Pin Mapping to HD44780
const RS: u8 = 0b0000_0001; // Register Select bit
const EN: u8 = 0b0000_0100; // Enable bit
const BACKLIGHT: u8 = 0b0000_1000; // Backlight control bit

/// Main LCD Controller structure.
pub struct LcdI2c<I2C> {
    i2c: I2C,
    addr: u8,
    backlight_state: u8,
}

impl<I2C: I2c> LcdI2c<I2C> {
    /// Creates a new LCD instance.
    /// # Arguments
    /// * `i2c` - The async I2C instance.
    /// * `addr` - The I2C address of the PCF8574 (usually 0x27 or 0x3F).
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            backlight_state: BACKLIGHT,
        }
    }

    /// Internal helper to send a 4-bit nibble.
    /// This pulses the EN (Enable) pin to latch the data.
    async fn write_nibble(&mut self, nibble: u8, mode: u8) -> Result<(), I2C::Error> {
        let base = nibble | mode | self.backlight_state;
        // Optimized: Send EN High and EN Low in a single I2C transaction
        self.i2c.write(self.addr, &[base | EN, base]).await
    }

    /// Sends a full byte (8-bit) as two 4-bit nibbles.
    async fn send_byte(&mut self, data: u8, mode: u8) -> Result<(), I2C::Error> {
        self.write_nibble(data & 0xF0, mode).await?;
        self.write_nibble((data << 4) & 0xF0, mode).await?;
        Ok(())
    }

    /// Méthode de secours : si l'envoi échoue, on ré-initialise et on retente.
    async fn safe_send(&mut self, data: u8, mode: u8, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        if self.send_byte(data, mode).await.is_err() {
            self.init(delay).await?; // Rebranchement détecté -> Ré-init 4-bit
            self.send_byte(data, mode).await?; // Retente l'envoi
        }
        Ok(())
    }

    /// Initializes the display using the standard 4-bit sequence.
     pub async fn init(&mut self, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        // Stabilisation électrique
        delay.delay_ms(100).await;

        // Force la réinitialisation logicielle du contrôleur (Software Reset)
        for _ in 0..3 {
            self.write_nibble(0x30, 0).await?;
            delay.delay_ms(10).await; 
        }

        // Passage définitif en mode 4-bits
        self.write_nibble(0x20, 0).await?; 
        delay.delay_ms(10).await;

        // Configuration du hardware
        self.send_byte(CMD_FUNCTION_SET | 0x08, 0).await?;     // 2 lignes, 5x8 dots
        self.send_byte(CMD_DISPLAY_CONTROL | 0x0C, 0).await?;  // Display ON, No cursor
        self.send_byte(CMD_ENTRY_MODE | 0x02, 0).await?;       // Auto-increment
        
        self.send_byte(CMD_CLEAR, 0).await?;
        delay.delay_ms(5).await;
        Ok(())
    }

   
    /// Clears the display. Requires a mandatory 2ms delay.
    pub async fn clear(&mut self, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        self.safe_send(CMD_CLEAR, 0, delay).await?; // Changé en safe_send
        delay.delay_ms(2).await;
        Ok(())
    }

    
    /// Sets the cursor to a specific row and column.
    pub async fn set_cursor(&mut self, row: u8, col: u8, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        let offsets = [0x00, 0x40, 0x14, 0x54];
        let addr = 0x80 + offsets[row as usize] + col;
        self.safe_send(addr, 0, delay).await // Utilise safe_send ici
    }

    /// Writes a string to the current cursor position.
    pub async fn write_str(&mut self, s: &str, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        for b in s.as_bytes() {
            self.safe_send(*b, RS, delay).await?; // Utilise safe_send ici
        }
        Ok(())
    }

    /// Toggles the backlight state.
    pub fn set_backlight(&mut self, on: bool) {
        self.backlight_state = if on { BACKLIGHT } else { 0x00 };
    }

    /// Returns the cursor to the home position (0,0) without clearing the display.
    pub async fn return_home(&mut self, delay: &mut impl DelayNs) -> Result<(), I2C::Error> {
        self.safe_send(CMD_RETURN_HOME, 0, delay).await
    }
}