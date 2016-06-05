# rust-sfml-rocket [![Build Status](https://travis-ci.org/zakkor/rust-sfml-rocket.svg?branch=master)](https://travis-ci.org/zakkor/rust-sfml-rocket)

> Rocket is a little game written in Rust, using the [Rust-SFML](https://github.com/jeremyletang/rust-sfml) library.

## Gameplay

![Screenshot](screenshots/ss.png)

## How to play

You play as a little square that can change colors. The goal is to avoid the platforms that have a color different to your current one. Passing through a platform with the same color as you will give you points. As time progresses, you go faster and faster, making things more difficult.

Button                 | Action
---------------------- | ------------
<kbd>Left Click</kbd>  | Cycle player color to the left
<kbd>Right Click</kbd> | Cycle player color to the right
<kbd>Space</kbd>       | Dash
<kbd>R</kbd>           | Restart the game
Move mouse left/right  | Move the player left/right on the horizontal axis
<kbd>Esc</kbd>         | Pause/unpause the game

## Building

You will need to install the [SFML 2.3.x](http://www.sfml-dev.org/download/sfml/2.3.2/) and [CSFML 2.3](http://www.sfml-dev.org/download/csfml/) libraries to be able to build.

On Ubuntu 14.04 at least, using the precompiled versions of the libraries from www.sfml-dev.org will cause a runtime error, so you will need to build them yourself.

<b>(Check the .travis.yml file for tips)</b>

## License

[GNU General Public License, version 3](https://github.com/zakkor/rust-sfml-rocket/blob/master/LICENSE.md)
