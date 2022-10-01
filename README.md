# DRG Save Editor
This is a standalone DRG save editor written in rust.

## There are likely to be bugs, see the Known Issues and Troubleshooting section!

## The project is inspired by [robertnunn's DRG-Save-Editor](https://github.com/robertnunn/DRG-Save-Editor), but aims to resolve some known issues and work on the most recent releases of DRG.

## Requirements
A computer with rust installed, pretty much anything that can run DRG is capable of compiling and running this editor.

## Installation and running
### Compiling it yourself
- clone the repo: `git clone https://github.com/AnthonyMichaelTDM/rust-DRG-Save-Editor.git`
- make sure you have rust installed, if you don't, you can follow the [instrustions here](https://www.rust-lang.org/tools/install)
- open your terminal in the base directory of the project
- run the `Cargo run` command to compile and run the program, if there are any compilation errors, open an issue on this github repo and I'll try to help you out

## Known Issues

## Troubleshooting
If the editor opens but doesn't edit your save properly (i.e., values not being read properly, changes not being reflected in-game, etc) please open an issue, describe the problem as thoroughly as you can, and attach a copy of your save file from BEFORE any edits were attempted.

## Usage
### ALWAYS BACKUP YOUR SAVE FILE!
The editor will make a backup of the save file you open in the same folder as the save file with the extension of `.old`. The editor makes this backup at the moment you open the save file.

The editor should be pretty self-explanatory, see the screenshot below.

Some notes:
- There is a context menu in the overclock tree listing to add overclocks to the inventory
- You can CTRL+Click on overclocks to select more than one
- Changing XP values will update the other relevant fields when the focus changes (i.e., click on a different part of the program or another program entirely)
- If you have promotions beyond Legendary 3 those promotions will be preserved as long as the drop-down is set to "Legendary 3+". If you don't have enough promotions for a specific dwarf and set them to "Legendary 3+" it will keep whatever the original value was.
- the `DRG Save Editor.zip` file is out of date, the only reason it's still in the repository is because there's really no point in removing it 

![main_screen](sshot.png)
## Changelog
- v1.0
  - Initial release

## To-Do
- Cosmetic overclock support
- GUI polish
- Better readme
- "Restore from backup" option in toolbar menu

## Would be nice, but ehh...
- Assignment support
- Character loadout support
- Perk support
- Weapon modification support
- Milestone support
- Bells & Whistles
