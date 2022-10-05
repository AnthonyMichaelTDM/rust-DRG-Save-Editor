# DRG Save Editor
This is a standalone DRG save editor written in rust.

# NOTE: currently this is just a GUI, the backend hasn't been implemented yet.

## There are likely to be bugs, see the Known Issues and Troubleshooting section!

## The project is inspired by [robertnunn's DRG-Save-Editor](https://github.com/robertnunn/DRG-Save-Editor), but aims to resolve some known issues and work on the most recent releases of DRG.

## Requirements
A computer with rust installed, pretty much anything that can run DRG is capable of compiling and running this editor.

## Installation and running
### Compiling it yourself
- clone the repo: `git clone https://github.com/AnthonyMichaelTDM/rust-DRG-Save-Editor.git`
- make sure you have rust installed, if you don't, you can follow the [instrustions here](https://www.rust-lang.org/tools/install)
- open your terminal in the base directory of the project
- run the `cargo run` command to compile and run the program, if there are any compilation errors, open an issue on this github repo and I'll try to help you out

## Known Issues
- as the GUI isn't finished yet, the program will panic if it's compiled and ran.

## Troubleshooting
If the editor opens but doesn't edit your save properly (i.e., values not being read properly, changes not being reflected in-game, etc) please open an issue, describe the problem as thoroughly as you can, and attach a copy of your save file from BEFORE any edits were attempted.

## Usage
### ALWAYS BACKUP YOUR SAVE FILE!
The editor will make a backup of the save file you open in the same folder as the save file with the extension of `.old`. The editor makes this backup at the moment you open the save file.

The editor should be pretty self-explanatory, see the screenshot below.

Some notes:
- There is a context menu in the overclock tree listing to add overclocks to the inventory
- Changing XP values will update the other relevant fields when the focus changes (i.e., click on a different part of the program or another program entirely)
- If you have promotions beyond Legendary 3 those promotions will be preserved as long as the drop-down is set to "Legendary 3+". If you don't have enough promotions for a specific dwarf and set them to "Legendary 3+" it will keep whatever the original value was.

![main_screen](sshot.png)
## Changelog

## To-Do
- Finish GUI frontend
- add backend for deserializing major components (resources, class xp/lvl/promotions, weapon OCs) of sav files
- add backend for serializing save files
- update for season 3
- "Restore from backup" option in toolbar menu

- GUI polish
- Better readme
- "Restore from backup" option in toolbar menu

## Long term goals (would be nice, not will take a lot of effort)
- Cosmetic overclock support
- Assignment support
- Character loadout support
- Perk support
- Weapon modification support
- Milestone support
- Bells & Whistles
