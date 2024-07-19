# Battlerite Controller "Cheat"

A program that turns gamepad inputs into keyboard and mouse inputs for the game Battlerite.

This was developed entirely for fun. This program is not viable in the slightest, playing with it is really difficult and not worth it.

Also don't use this in real games, please. I'm not sure how allowed this is.

## Controls

| keyboard & mouse | gamepad
|---|---|
WASD movement | Left stick
Aim (mouse position) | Right stick
Left click | Right trigger (LT)
Right click | Left trigger (RT)
Q | Left button (LB)
E | Right button (RB)
Space | Button South (A for xbox) <br> Exceptionally aimed with Left stick (movement stick), not Right stick.
R | D-pad down
F | D-pad up
Escape | Start

## How to use

Download the repository
```sh
git clone https://github.com/ornitongithub/battlerite-controller-cheat && cd battlerite-controller-cheat
```

Download the Rust compiler: https://www.rust-lang.org/tools/install

And compile the project
```sh
cargo build --release # find your executable in ./target/debug
```
or
```sh
cargo run --release # compile and run directly
```
On Linux, you may need to download `xdotool` for the compilation to work. (If you encounter [this error](https://github.com/enigo-rs/enigo/issues/55).)

On Windows, I have no clue.