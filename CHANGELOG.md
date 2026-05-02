# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Ce projet suit (globalement) le format de [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/)
et le versionning s’inspire de [Semantic Versioning](https://semver.org/lang/fr/).


## Version 0.3.2

- Amélioration du README  
- Simplification des exemples  
- Clarté accrue  

Cette version est considérée comme stable.  
Elle introduit également un fichier `CHANGELOG.md` afin d’alléger le README et de conserver uniquement les informations essentielles.  


# Update Version 0.3.1, Résolution du problème d'affichage (RP2040) 
Le projet ne fonctionnait pas initialement à cause d'une désynchronisation entre la configuration logicielle et les contraintes matérielles du RP2040. Voici les corrections apportées :

1. Gestion des Opérations Atomiques (CAS)
Le processeur Cortex-M0+ de la Pico 1 ne possède pas d'instructions atomiques natives. Pour utiliser les bibliothèques modernes (embedded-hal-bus, embassy-rp), nous avons dû forcer le compilateur à simuler ces opérations.

Fichier : .cargo/config.toml

Action : Ajout du flag --cfg portable_atomic_unsafe_assume_single_core dans les rustflags. Cela permet de compiler les dépendances qui exigent des garanties de synchronisation.

2. Alignement des Dépendances (Cargo.toml)
L'utilisation de versions disparates d'Embassy empêchait la bonne communication sur le bus I2C.

Action : Unification des versions sur Embassy 0.6.0 et HAL 1.0.0.

Impact : Cela garantit que tous les traits (I2C, Delay, etc.) sont compatibles entre le driver de l'écran et le reste du système.

3. Correction de la Matrice I2C (Hardware Mapping)
Le RP2040 a un câblage fixe pour ses périphériques I2C.

Erreur : Tentative d'inversion logicielle de SDA et SCL sur les pins 8 et 9.

Correction : Respect de la datasheet du RP2040 :

GP8 = SDA (Data)

GP9 = SCL (Clock)

Code : I2c::new_async(p.I2C0, p.PIN_9, p.PIN_8, Irqs, config)

4. Stabilisation du Driver (Timing)
Les écrans LCD HD44780 sont beaucoup plus lents que le processeur de la Pico.

Action : Injection de délais de 500µs entre l'activation et la désactivation du signal Enable (EN) dans le driver local.

Résultat : L'écran a maintenant le temps de "lire" chaque bit envoyé avant que le suivant n'arrive.


# Update la version 0.3.0 introduit le #![forbid(unsafe_code)] 
Choix pour la safety.


# 🚀 Mise à jour v0.2.4  Exemple
Cette version est une étape majeure pour la fiabilité du driver dans l'écosystème Rust embarqué.

**📦 Ce qui change :**
Exemple "Plug & Play"  Dans la Section exemples : Ajout d'un exemple complet prêt à l'emploi. Il inclut le main.rs, la gestion du clignotement de la LED (Blink) et l'initialisation du LCD, ainsi que les dépendances pour lancer plus vite si vous avez du mal avec l'écosystème des crates .

Linker Pico 2 (RP2350) : Inclusion d'une configuration de Linker optimisée pour la Raspberry Pi Pico 2. C'est une ressource précieuse pour ceux qui migrent vers cette nouvelle puce.


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

**⚠️ Note sur les limitations physiques du matériel**

En raison de la décharge capacitive sur la plupart des modules HD44780 I2C, un redémarrage à froid est nécessaire pour une réinitialisation parfaite.

Récupération fiable : si l’écran est déconnecté pendant plus de 5 secondes, le driver restaure automatiquement l’interface complète lors de la reconnexion

Reconnexion instantanée : si la reconnexion est trop rapide (< 1 s), une tension résiduelle dans les condensateurs peut provoquer des problèmes de synchronisation (données corrompues)

➡️ C’est une limitation physique connue du circuit de reset du contrôleur HD44780, et non un défaut logiciel.


## Version 0.1.3
- Amélioration du README
- Simplification des exemples
- Clarté accrue


## 🦅 Version 0.1.2
- Support asynchrone complet via I2c et DelayNs
- Gestion du curseur et du rétroéclairage
- Optimisation : écriture en une seule transaction pour réduire la charge I2C
