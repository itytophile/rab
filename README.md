# RAB
Rusty Armor Builds - Monster Hunter Rise Armor Set Creation Tool
## Installation
WARNING: The binary (the .exe file) must be next to the "armors" folder for it to work!!!!

If you use Windows or Linux, you can download the binary here https://github.com/itytophile/rab/releases
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
and clone the armors files repo (the armor files must be in a folder named "armors"):
```sh
git clone https://github.com/itytophile/monster-hunter-rise-armors.git armors
```
then you can run the software (execute the command directly in the rab folder):
```sh
cargo run --release
```
If you want to move the binary somewhere else, it is located here: `rab/target/release/rab(.exe)`

WARNING: The binary must be next to the "armors" folder for it to work!!!!
