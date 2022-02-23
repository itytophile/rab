Web version available: https://itytophile.github.io/rab/

# RAB

Rusty Armor Builds - Monster Hunter Rise Armor Set Creation Tool

Armor files used by RAB here: https://github.com/itytophile/monster-hunter-rise-armors

Localization files here: https://github.com/itytophile/rab-locale

MH Icons from https://monsterhunter.fandom.com/wiki/User:YukiHerz/SVG_Icons

Other icons from Font Awesome https://fontawesome.com/license/free

## Installation

If you use Windows or Linux, you can download the binary here https://github.com/itytophile/rab/releases

RAB will download on its own the armor and localization files if they are not present.

However you can still download the files manually [here](https://github.com/itytophile/monster-hunter-rise-armors) and [here](https://github.com/itytophile/rab-locale). The armor files must be in a folder named "armors" and the localization files in a folder named "locale". Both folders must be next to the executable.

## Need help for localization!

At the moment, the skills and armors are translated into

- English
- Polish
- French
- German
- Italian
- Russian
- Spanish

I need help for translating the UI in all these languages except French (English is done but as I am not a native speaker I can't really say this is perfect).

Unfortunately Chinese, Korean and Japanese can't be supported yet. The GUI library (iced) I use doesn't support them.

If you are interested, you can check how a localization is done here: https://github.com/itytophile/rab-locale

It should be understandable, you can compare the French and English localization to see how this is done.

## Screenshots

Choose the wished skills and RAB searches compatible builds for you:

![Main RAB page](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/rab_main.png)

You can add your own talismans to RAB:

![Talisman menu](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/talisman_menu.png)

Manage your builds!

![Build details](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/rab_details.png)
![Build list](https://raw.githubusercontent.com/itytophile/rab/main/docs/screenshots/rab_builds.png)

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
