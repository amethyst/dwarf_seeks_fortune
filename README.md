[![Build Status](https://travis-ci.com/Jazarro/towerdef.svg?branch=master)](https://travis-ci.com/Jazarro/towerdef)

# towerdef (Working title)

Design goals:
- No mouse should be necessary, strive to be able to do everything with keyboard.
- Develop suite of debug tools early on to make development easier.
    - Map editor (be able to jump into level straight away from the editor)
- Be a proper example of how to use Amethyst. Document everything. Be helpful resource for people learning Amethyst.
- Write guides explaining how certain features are implemented? (e.g. asset loading, camera, etc) Guides should serve as next step after reading the pong tutorial and basic examples in Amethyst, be a real-world implementation.
- Add tests where necessary.

Brainstorm ideas:
- Tools
    - Break 1 / 2 blocks horizontally
    - Break 1 / 2 blocks below
    - Push blocks
    - Dynamite (bomberman style)
    - Tunnel bore machine
    - Placeable ladder
    - Balloons that go up, attachable to tools/blocks? Moves tools to where you can actualy use them.
    - Water streams to move tools or blocks?
- Other
    - Trap that flings knife? Knife goes round forever.

TODO:
- Level data format, tile refs and separate tile data
- Formalise z-ordering
- Jumping and collision detection.
- Maybe extract editor into its own crate?

Level bounds:
- Create clear distinction between what is inside level and what is outside, by using different background. For now, always use rectangular shape.
