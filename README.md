# Crab Chip
[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator for Windows & MacOS written in Rust

![Preview](/assets/screenshot.png)
> *Game: "Space Invaders" by David Winter*

## Download
You can get the emulator from the [releases tab](https://github.com/DarylPinto/crab-chip/releases) for 64-bit Windows and Intel Mac devices.

## Games
The release comes packaged with a few games. Many more are available to download from the [CHIP-8 ROM archive](https://github.com/JohnEarnest/chip8Archive/tree/master/roms) and elsewhere on the internet as freeware. Select the game you'd like to play by editing the `settings.yaml` file. All games are loaded from the `roms` folder.

## Controls
The CHIP-8 uses a hexadecimal keypad for input. These are mapped as such on a QWERTY keyboard:
```
     CHIP-8                      Computer
   Hex Keypad                    Keyboard

.---.---.---.---.            .---.---.---.---.
| 1 | 2 | 3 | C |            | 1 | 2 | 3 | 4 |
.---.---.---.---.            .---.---.---.---.
| 4 | 5 | 6 | D |            | Q | W | E | R |
.---.---.---.---.     ->     .---.---.---.---.
| 7 | 8 | 9 | E |            | A | S | D | F |
.---.---.---.---.            .---.---.---.---.
| A | 0 | B | F |            | Z | X | C | V |
.---.---.---.---.            .---.---.---.---.
```
Each game uses it's own control scheme, so if you're unsure how a game works then play around with the keys to see what each one does.

___

### Disclaimer
I'm not a Rust expert. There are almost certainly more idiomatic approaches to implementation, so keep that in mind while reading the code.

### Development Resources
* Technical reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
* Implementation guides:
	* http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
	* https://austinmorlan.com/posts/chip8_emulator/
