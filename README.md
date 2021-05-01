# RAB
Rusty Armor Builds - Monster Hunter Rise Armor Set Creation Tool

Armor files used by RAB here: https://github.com/itytophile/monster-hunter-rise-armors

Localization files here: https://github.com/itytophile/rab-locale
## Installation
WARNING: The binary (the .exe file) must be next to the "armors" folder for it to work!!!!

If you use Windows or Linux, you can download the binary here https://github.com/itytophile/rab/releases

FOR WINDOWS USER: If RAB doesn't work, maybe you need to install [Visual C++](https://support.microsoft.com/en-us/topic/the-latest-supported-visual-c-downloads-2647da03-1eea-4433-9aff-95f26a218cc0) (check the exe files under "Visual Studio 2015, 2017 and 2019").
## Screenshots
Choose the wished skills and RAB searches compatible builds for you:

![Main RAB page](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/rab_main.png)

You can add your own talismans to RAB:

![Talisman menu](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/talisman_menu.png)
![Talisman edition](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/talisman_edition.png)

## Building
You have to install Rust first. Instructions here https://rustup.rs/
And you need git.

Then open a shell:
```sh
git clone https://github.com/itytophile/rab.git
```
Go to the rab folder:
```sh
cd rab
```
clone the armors files repo (the armor files must be in a folder named "armors"):
```sh
git clone https://github.com/itytophile/monster-hunter-rise-armors.git armors
```
and clone the localization files repo (must be in a folder named "locale"):
```sh
git clone https://github.com/itytophile/rab-locale.git locale
```
then you can run the software (execute the command directly in the rab folder):
```sh
cargo run --release
```
If you want to move the binary somewhere else, it is located here: `rab/target/release/rab(.exe)`

WARNING: The binary must be next to the "armors" folder for it to work!!!!
