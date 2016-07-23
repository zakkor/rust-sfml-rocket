# rust-sfml-rocket [![Build Status](https://travis-ci.org/zakkor/rust-sfml-rocket.svg?branch=master)](https://travis-ci.org/zakkor/rust-sfml-rocket)

> Rocket is a little game written in Rust, using the [Rust-SFML](https://github.com/jeremyletang/rust-sfml) library.

## Gameplay

[<b>GFY of gameplay</b>](https://gfycat.com/FrighteningRelievedGuillemot)

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

## Download binaries
Grab the latest stable release from https://github.com/zakkor/rust-sfml-rocket/releases

## Building

## Linux

You will need to install the [SFML 2.3.x](http://www.sfml-dev.org/download/sfml/2.3.2/) and [CSFML 2.3](http://www.sfml-dev.org/download/csfml/) libraries to be able to build.

On Ubuntu 14.04 at least, using the precompiled versions of the libraries from www.sfml-dev.org will cause a runtime error, so you will need to build them yourself.

I have added a simple bash script to download, build, and install both SFML-2.3.2 and CSFML-2.3 from source.
##### 1. Clone this repository
`git clone https://github.com/zakkor/rust-sfml-rocket`

##### 2. Install dependencies needed for SFML
`sudo apt-get install libpthread-stubs0-dev libgl1-mesa-dev libx11-dev libx11-xcb-dev libxcb-image0-dev libxrandr-dev libxcb-randr0-dev libudev-dev libfreetype6-dev libglew-dev libjpeg8-dev libgpgme11-dev libsndfile1-dev libopenal-dev libjpeg62 cmake`

These are the Ubuntu packages, if you're not on an Ubuntu-based distro, install the equivalent packages.

##### 3. Edit install_deps.sh with your own info
`cd rust-sfml-rocket/install_deps`

Open up `install_deps.sh` with your favorite editor, find the line that says 'Replace this path', and replace the path with the ABSOLUTE path to the install_deps directory.

In my case, the full path is `/home/ed/Programs/rust-sfml-rocket/install_deps/CSFML-2.3/cmake/Modules`

##### 4. Run the script
This will install both SFML and CSFML to `/usr/local`.

<b>If your distro does not use this directory structure, edit `install_deps.sh` and modify the `sudo make install` lines to `sudo make DESTDIR="/your/path" install`. </b>

`sudo bash install_deps.sh`

You should be all good to go!

##### 5. Run the game
Go back to the root dir: `cd ..`

Run the game: `cargo run`

After cargo is done downloading and compiling dependencies, it will compile and run the game.

## Mac OS X

#### 1. Download dependencies
I recommend using [Homebrew](http://brew.sh/).

Get SFML and CSFML:

`brew install sfml csfml`

#### 2. Clone the repository
`git clone https://github.com/zakkor/rust-sfml-rocket`

`cd rust-sfml-rocket`

`cargo run`

That's it!

## License

[GNU General Public License, version 3](https://github.com/zakkor/rust-sfml-rocket/blob/master/LICENSE.md)
