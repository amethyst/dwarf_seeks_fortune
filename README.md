[![Build Status](https://travis-ci.com/Jazarro/towerdef.svg?branch=master)](https://travis-ci.com/Jazarro/towerdef)

# Dwarf Seeks Fortune (Working title)
A 2D puzzle platformer made with the Amethyst game engine. This game is an homage to the 1988 classic [King's Valley II](https://en.wikipedia.org/wiki/King%27s_Valley_II). 

The aim of this project is to be a helpful resource to anyone learning Amethyst for the first time. It is my intention to write guides explaining how certain features are implemented in this game. I'll not claim that this is the only way or even the best way to do things, but to someone just starting out it may prove helpful regardless.

## Gameplay
You're a dwarf, digging through the ancient ruins of your ancestors. Each level presents a different puzzle. The aim is to collect all keys, after which the door to the next level is unlocked. Puzzle elements include one-time-use tools that must be picked up and used in a specific order at specific locations.

## Design goals:
- This project aims to be a helpful resource for people learning Amethyst. Code should ideally be  thoroughly documented and easy to understand.
- I personally hate games that make you use the mouse more than necessary. This game must be fully accessible with a keyboard alone. Mouse input should always be optional.
- This game should be accompanied by guides explaining how certain features are implemented. These guides should serve as a next step after reading the pong tutorial and the basic examples.
- This game should ideally depend on a specific release version of the Amethyst engine. This will make for a more stable example. 

## Features:
- [x] Full keyboard support. No mouse should be necessary at any point. That said, mouse support for the editor is on the roadmap.
- [x] In-game level editor. Allows for rapid iteration process when designing levels.
- [x] Time rewinding mechanic to help fix mistakes when solving the puzzles. Might be removed if it proves superfluous or reductive to the experience.
- [ ] Derpy movement mechanics akin to those of the game this is based on. This game purposely refrains from using a full physics simulation, opting instead for deterministic and predictable grid-based movements suitable for a puzzle game.
- [ ] Multiple playable levels.
- [ ] Sound effects and music.