[![Crates.io](https://img.shields.io/crates/v/hd44780-i2c-nostd.svg)](https://crates.io/crates/hd44780-i2c-nostd)
[![Documentation](https://docs.rs/hd44780-i2c-nostd/badge.svg)](https://docs.rs/hd44780-i2c-nostd)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/license-GPL--2.0--or--later-blue.svg)](https://opensource.org/licenses/GPL-2.0-or-later)


# 📟 hd44780-i2c-nostd
🦅 Version v0.3.2 testée sur la pico 2 et pico 2040,
utilisez la dernière version pour plus de stabilité.

Un pilote HD44780 robuste et haute performance pour Rust (no_std). Optimisé pour Embassy et les systèmes embarqués comme RP2040 (Pico), Pico 2, STM32 et ESP32.

# 📋 Mises à jour

## Version 0.3.2

- Amélioration du README  
- Simplification des exemples  
- Clarté accrue  

Cette version est considérée comme stable.  
Elle introduit également un fichier `CHANGELOG.md` afin d’alléger le README et de conserver uniquement les informations essentielles.  

👉 Pour consulter l’historique complet des changements, n’hésitez pas à vous référer au `CHANGELOG.md`.

----

# 🛠️ Tests et Compatibilité :
Validé sur Pico 2 : Le driver hd44780-i2c-nostd a été testé avec succès sur le matériel RP2350.

Amélioration continue : N'hésitez pas à remonter des bugs ou à suggérer des améliorations. Ce driver évolue grâce à vos retours.

**⚠️ Disclaimer :**
L'électronique est capricieuse. Assurez-vous de vérifier vos tensions (5V) et vos adresses I2C (0x3F ou 0x27). 

----

# NOTE
**Changement d’API :**

Pour garantir cette fiabilité « Always-On », les méthodes publiques comme **write_str**, **set_cursor** et **clear** nécessitent désormais un argument de délai.
Cela permet de respecter les timings matériels lors d’une récupération automatique.

# 🚀 Fonctionnalités principales
**Asynchrone natif :** construit dès le départ pour embedded-hal-async (aucune boucle bloquante, aucun gaspillage CPU).

**Efficacité zero-copy :** transactions I2C optimisées avec regroupement des états High/Low pour saturer efficacement le bus
no_std & bare-metal : parfait pour Embassy, RTIC ou des kernels personnalisés.

**Initialisation anti-glitch :** séquence officielle 4 bits avec délais précis pour garantir un démarrage propre
Layouts flexibles : compatible avec écrans 16x2, 20x4 et autres formats standards.


----


# 🛠️ Utilisation
**Dans votre Cargo.toml**
````
[dependencies]
hd44780-i2c-nostd = "0.3.2"
````

----


# 💡 Démarrage rapide
````rust
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

````

----

# 🎮 Exemple : télémétrie en temps réel
````rust
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

----

# ⚖️ Licence

Ce projet est sous licence GNU GPL v2.0 ou ultérieure.

Vous êtes libre de l’utiliser, mais toute amélioration doit être partagée avec la communauté.


👨‍💻 Créé par Jorge Andre Castro