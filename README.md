 # 📟 hd44780-i2c-nostd
🦅 Version v0.3.0 testé  sur la pico 2 

Un pilote HD44780 robuste et haute performance pour Rust (no_std). Optimisé pour Embassy et les systèmes embarqués comme RP2040 (Pico), Pico 2, STM32 et ESP32.

# Update la version 0.3.0 introduit le #![forbid(unsafe_code)] 
Choix pour la safety.

# 🚀 Mise à jour v0.2.4  Exemple
Cette version est une étape majeure pour la fiabilité du driver dans l'écosystème Rust embarqué.

📦 Ce qui change :
Exemple "Plug & Play"  Dans la Section exemples : Ajout d'un exemple complet prêt à l'emploi. Il inclut le main.rs, la gestion du clignotement de la LED (Blink) et l'initialisation du LCD, ainsi que les dépendances pour lancer plus vite si vous avez du mal avec l'écosystème des crates .

Linker Pico 2 (RP2350) : Inclusion d'une configuration de Linker optimisée pour la Raspberry Pi Pico 2. C'est une ressource précieuse pour ceux qui migrent vers cette nouvelle puce.


# 🛠️ Tests et Compatibilité :
Validé sur Pico 2 : Le driver hd44780-i2c-nostd a été testé avec succès sur le matériel RP2350.

Appel aux testeurs (Pico 1/RP2040) : Bien que le driver soit conçu pour fonctionner sur tout l'écosystème Embassy, la théorie ne remplace jamais la pratique. Si vous utilisez une Pico 1, vos retours sont les bienvenus !

Amélioration continue : N'hésitez pas à remonter des bugs ou à suggérer des améliorations. Ce driver évolue grâce à vos retours.

⚠️ Disclaimer :
L'électronique est capricieuse. Assurez-vous de vérifier vos tensions (5V) et vos adresses I2C (0x3F ou 0x27). Un grand merci pour votre confiance et pour les nombreux téléchargements !!!!



# 🛡️ Résilience matérielle & auto-réparation (v0.2.2)

La mise à jour la plus importante de la version 0.2.2 est l’introduction d’une couche de communication résiliente, conçue pour les systèmes embarqués fonctionnant sur de longues durées.

Le problème des « données corrompues »

Les pilotes HD44780 classiques souffrent souvent de « corruption LCD » ou d’« hiéroglyphes ».
Cela se produit lorsque l’écran perd son alimentation ou est physiquement déconnecté.

Lors de la reconnexion :

le LCD se réinitialise en mode 8 bits par défaut
tandis que le microcontrôleur continue d’envoyer des données en mode 4 bits

➡️ Ce décalage rend l’affichage illisible et nécessite un redémarrage manuel du système.

La solution : la logique safe_send

Cette crate résout le problème en encapsulant les transactions I2C dans une boucle auto-réparatrice :

Détection : chaque commande surveille le bus I2C pour détecter les erreurs NACK ou les échecs de communication
Récupération automatique : en cas d’erreur, le driver suppose un hot-plug ou une coupure et relance automatiquement la séquence d’initialisation 4 bits
Reprise transparente : les données originales sont renvoyées, garantissant un affichage correct sans intervention manuelle ni reflash
⚠️ Note sur les limitations physiques du matériel

En raison de la décharge capacitive sur la plupart des modules HD44780 I2C, un redémarrage à froid est nécessaire pour une réinitialisation parfaite.

Récupération fiable : si l’écran est déconnecté pendant plus de 5 secondes, le driver restaure automatiquement l’interface complète lors de la reconnexion
Reconnexion instantanée : si la reconnexion est trop rapide (< 1 s), une tension résiduelle dans les condensateurs peut provoquer des problèmes de synchronisation (données corrompues)

➡️ C’est une limitation physique connue du circuit de reset du contrôleur HD44780, et non un défaut logiciel.

# NOTE
Changement d’API :
Pour garantir cette fiabilité « Always-On », les méthodes publiques comme write_str, set_cursor et clear nécessitent désormais un argument de délai.
Cela permet de respecter les timings matériels lors d’une récupération automatique.



hd44780-i2c-nostd fournit un moyen fiable de piloter des écrans LCD classiques via l’expandeur I2C PCF8574.

Cette crate est sous licence GPL-2.0-or-later afin de garantir que les drivers matériels fondamentaux restent un bien commun, et ne soient jamais enfermés dans des solutions propriétaires.

# 🚀 Fonctionnalités principales
Vrai asynchrone natif : construit dès le départ pour embedded-hal-async (aucune boucle bloquante, aucun gaspillage CPU)
Efficacité zero-copy : transactions I2C optimisées avec regroupement des états High/Low pour saturer efficacement le bus
no_std & bare-metal : parfait pour Embassy, RTIC ou des kernels personnalisés
Initialisation anti-glitch : séquence officielle 4 bits avec délais précis pour garantir un démarrage propre
Layouts flexibles : compatible avec écrans 16x2, 20x4 et autres formats standards
# 📋 Changelog & mises à jour
 🦅 Version 0.1.2
Support asynchrone complet via I2c et DelayNs
Gestion du curseur et du rétroéclairage
Optimisation : écriture en une seule transaction pour réduire la charge I2C

# 🛠️ Utilisation
Installation
````
[dependencies]
hd44780-i2c-nostd = "0.1.2"
💡 Démarrage rapide
use hd44780_i2c_nostd::LcdI2c;
use embassy_time::Delay;

// Initialisation I2C (exemple RP2040)
// let i2c = I2c::new(...);

// Création de l'écran
let mut lcd = LcdI2c::new(i2c, 0x27);

// Initialisation avec délai
lcd.init(&mut Delay).await.unwrap();

// Écriture
lcd.set_cursor(0, 0, &mut Delay).await.ok();
lcd.write_str("Project of my life", &mut Delay).await.ok();

// Rétroéclairage
lcd.set_backlight(true);
🎮 Exemple : télémétrie en temps réel
loop {
    let temp = sensor.read_temp().await;
    lcd.set_cursor(1, 0, &mut Delay).await.ok();

    let mut buf = [0u8; 16];
    if let Ok(s) = format_no_std(&mut buf, format_args!("Temp: {:.2}C", temp)) {
        lcd.write_str(s, &mut Delay).await.ok();
    }

    Timer::after_millis(500).await;
}

````
# Exemple
````
[package]
name = "andrew-pico2"
version = "0.1.0"
edition = "2021"

[dependencies]
 On garde embassy-rp 0.10.0
embassy-rp = { version = "0.10.0", features = ["rp235xa", "rt", "critical-section-impl", "time-driver"] }

 On monte embassy-time à la version 0.5.1 pour correspondre à la queue v0.3.0
embassy-time = { version = "0.5.1", features = ["generic-queue-8"] }

On aligne l'exécuteur et le reste
embassy-executor = { version = "0.7.0", features = ["arch-cortex-m", "executor-thread"] }
embassy-sync = { version = "0.6.1" }
cortex-m-rt = "0.7.3"

embassy-embedded-hal = { version = "0.3.0" }
embedded-hal = "1.0"
embedded-hal-async = "1.0"

Mes crates 
rp2350-linker = "0.2.1"
hd44780-i2c-nostd = "0.3.0"
panic-halt = "0.2.0"

[profile.release]
lto = true
opt-level = 'z'
panic = "abort"
strip = true

````
# Le config.toml indispensable :

````
[build]
target = "thumbv8m.main-none-eabihf"

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs -d"

rustflags = [
  "-C", "linker=flip-link",
  "-C", "link-arg=-Tlink.x",
  
  "-C", "link-arg=--nmagic",
   
]

````
si jamais le link vous manque :cargo install flip-link

La flash.sh avec picotool:

#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"

 1. Compilation
cargo build --release || exit 1

2. Conversion 
 On utilise le nom 'Votrenomdeprojet' 
picotool uf2 convert -t elf target/thumbv8m.main-none-eabihf/release/votrenomdeprojet-pico2 Votrenomdeprojet.uf2 --family rp2350-arm-s

 3. Flash
 picotool a besoin d'un accès USB, sudo est souvent nécessaire ici
sudo picotool load Votrenomdeprojet.uf2 -x




# Et l'integration :
````rust
#![no_std]
#![no_main]

use cortex_m_rt as _; 
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_time::{Delay, Duration, Timer};
use hd44780_i2c_nostd::LcdI2c;
use {panic_halt as _, embassy_rp as _};

// 🦅 Signature d'Andrew
use rp2350_linker as _;

// Liaison des interruptions pour l'I2C0
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::I2C0;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 1. Initialisation système
    let p = embassy_rp::init(embassy_rp::config::Config::default());

    // 2. Configuration I2C Robuste (100kHz pour éviter les erreurs de timing)
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 100_000;
    
    // Une seule initialisation de l'I2C (ne pas répéter cette ligne)
    let i2c = I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, i2c_config);
    
    // 3. Initialisation du LCD 
    // ATTENTION : Vérifie ton adresse. 0x27 est la plus commune, 0x3F est la seconde.
    let mut lcd = LcdI2c::new(i2c, 0x3F); 
    
    // 4. Séquence de démarrage du LCD
    // On attend un peu que l'écran soit sous tension
    Timer::after(Duration::from_millis(500)).await;

    if lcd.init(&mut Delay).await.is_ok() {
        let _ = lcd.set_backlight(true);
        let _ = lcd.set_cursor(0, 0, &mut Delay).await;
        let _ = lcd.write_str("Hello Andre!", &mut Delay).await;
        let _ = lcd.set_cursor(1, 0, &mut Delay).await;
        let _ = lcd.write_str("Pico 2 Flying ", &mut Delay).await;
    }

    // 5. LED sur GP25 pour la Pico 2
    let mut led = Output::new(p.PIN_25, Level::Low);

    // Boucle Heartbeat
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;
        led.set_low();
        Timer::after(Duration::from_millis(900)).await;
    }
}
````

**Exemple Gyro et sqrt  gestion du bus pour eviter des bugs de freeze**

````rust 
// Copyright (C) 2026 Jorge Andre Castro
#![no_std]
#![no_main]
#![forbid(unsafe_code)]

use cortex_m_rt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Flex, Pull};
use embassy_rp::i2c::{Config as I2cConfig, I2c, Async};
use embassy_time::{Delay, Duration, Timer, with_timeout};
use hd44780_i2c_nostd::LcdI2c;
use {panic_halt as _, embassy_rp as _};
use heapless::String;
use core::fmt::Write;

use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use static_cell::StaticCell;

use embassy_gy_bmi160::Bmi160;
use embassy_gy_bmi160::signals::{ACCEL_SIGNAL, GYRO_SIGNAL};
use embedded_f32_sqrt::sqrt;

use rp2350_linker as _;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::I2C0; 

bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

static I2C_BUS: StaticCell<Mutex<NoopRawMutex, I2c<'static, I2C0, Async>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut p = embassy_rp::init(embassy_rp::config::Config::default());
    
    // PATCH ANTI-BUG PAR REBORROW 
    {
        // .reborrow() permet de créer un accès temporaire pour Flex
        // sans consommer définitivement p.PIN_5 et p.PIN_4.
        let mut scl = Flex::new(p.PIN_5.reborrow());
        let mut sda = Flex::new(p.PIN_4.reborrow());
        
        scl.set_as_output();
        sda.set_as_input();
        sda.set_pull(Pull::Up);

        for _ in 0..9 {
            scl.set_low();
            Timer::after_micros(10).await;
            scl.set_high();
            Timer::after_micros(10).await;
            if sda.is_high() { break; }
        }
    } 
    
    Timer::after_millis(50).await; 

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 100_000;
    
    // Les pins sont toujours possédées par 'p', donc on peut les donner à l'I2C ici.
    let i2c = I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, i2c_config);
    let i2c_bus = Mutex::<NoopRawMutex, _>::new(i2c);
    let i2c_bus = I2C_BUS.init(i2c_bus);

    let lcd_i2c = I2cDevice::new(i2c_bus);
    let bmi_i2c = I2cDevice::new(i2c_bus);

    let lcd = LcdI2c::new(lcd_i2c, 0x3F); 
    let bmi = Bmi160::new(bmi_i2c, 0x68);

    spawner.spawn(system_task(lcd, bmi)).unwrap();

    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(200).await; 
    }
}

#[embassy_executor::task]
async fn system_task(
    mut lcd: LcdI2c<I2cDevice<'static, NoopRawMutex, I2c<'static, I2C0, Async>>>,
    mut bmi: Bmi160<'static, I2cDevice<'static, NoopRawMutex, I2c<'static, I2C0, Async>>>
) {
    let mut delay = Delay;
    let mut imu_ready = false;
    
    if lcd.init(&mut delay).await.is_ok() {
        let _ = lcd.set_backlight(true);
        let _ = lcd.clear(&mut delay).await;
        let _ = lcd.write_str("THE RUST EAGLE", &mut delay).await;
        Timer::after_secs(1).await;
    }

    match with_timeout(Duration::from_millis(800), bmi.init()).await {
        Ok(Ok(_)) => imu_ready = true,
        _ => {
            bmi.set_address(0x69);
            if let Ok(Ok(_)) = with_timeout(Duration::from_millis(800), bmi.init()).await {
                imu_ready = true;
            }
        }
    }

    loop {
        if imu_ready {
            if let Ok(g) = bmi.read_gyro().await {
                GYRO_SIGNAL.signal(g);
                let _ = lcd.set_cursor(0, 0, &mut delay).await;
                let mut s: String<20> = String::new();
                let _ = write!(s, "G:{:>3},{:>3},{:>3}", g.x/128, g.y/128, g.z/128);
                let _ = lcd.write_str(s.as_str(), &mut delay).await;
            }

            if let Ok(a) = bmi.read_accel().await {
                ACCEL_SIGNAL.signal(a);
                let x = a.x as f32; 
                let y = a.y as f32; 
                let z = a.z as f32;
                let mag = sqrt(x*x + y*y + z*z).unwrap_or(0.0) / 16384.0;

                let _ = lcd.set_cursor(1, 0, &mut delay).await;
                let mut s: String<20> = String::new();
                let _ = write!(s, "ACCEL: {:.2}G  ", mag);
                let _ = lcd.write_str(s.as_str(), &mut delay).await;
            }
        } else {
            let _ = lcd.set_cursor(1, 0, &mut delay).await;
            let _ = lcd.write_str("IMU NOT FOUND", &mut delay).await;
        }
        Timer::after_millis(150).await;
    }
}

````


# ⚖️ Licence

Ce projet est sous licence GNU GPL v2.0 ou ultérieure.

Vous êtes libre de l’utiliser, mais toute amélioration doit être partagée avec la communauté.

# 🦅 Pourquoi l’utiliser ?

Parce que dans le « projet de ta vie », tu ne peux pas te permettre un driver qui freeze ou utilise du code bloquant.

hd44780-i2c-nostd est conçu pour être le pont invisible, robuste et fiable entre ta logique et ton interface utilisateur.

👨‍💻 Créé par Jorge Andre Castro