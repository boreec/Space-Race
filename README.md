# Space Race

This game is an amateur reproduction of the [Space Race](https://en.wikipedia.org/wiki/Space_Race_(video_game)) game initially created by Atari, Inc. in 1973.

This program was built completely with rust and the sdl2 bindings.

To see similar projects I made: https://boreec.fr/projects/

## How to run ?

Use **cargo** to build and run the program:
```bash
cargo run --release
```

## How to play ?

Your goal is to pilot your spaceship (on the left) towards the top of screen, while avoiding missiles.
You have 45 seconds (indicated by the decreasing timeline in the middle of the screen) in order to score
more points than your opponent (on the right), controlled by the computer.

Every time you get touched by a missile, you respawn after a few seconds at the bottom of the screen!
Although your opponent keeps moving upward without resting, it may not be the best strategy to win...

## Controls

You can use the following keys during the game:
* **ESC key** : Quit the game (you can also click on the window's cross).
* **Space key**: Restart the game.
* **Up arrow key**: Move your spaceship toward the top of the screen.
* **Down arrow key**: Move your spaceship toward the bottom of the screen.

## Troubleshooting

If you have issues building the program, make sure the following packages are installed on the system:

```bash
sudo apt-get install -y libsdl2-dev libsdl2-ttf-dev libsdl2-gfx-1.0.0 libsdl2-gfx-dev
```
