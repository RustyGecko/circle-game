# circle-game 
[![Build Status](https://travis-ci.org/RustyGecko/circle-game.svg?branch=update-nightly)](https://travis-ci.org/RustyGecko/circle-game)

A small game for the [DK3750 Development Kit](http://www.silabs.com/products/mcu/lowpower/Pages/efm32gg-dk3750.aspx).

The game uses the `TFT screen` and an external gameboard that 
is connected to `Port C, Pins 0-7` of the breakout board that is available 
on the development kit.

# Building
The game is dependant on [emlib](https://github.com/RustyGecko/emlib.git) and works
an example application of using the library together with the DK3750.

The following dependencies need to be installed:
* [ARM GCC Embedded Toolchain](https://launchpad.net/gcc-arm-embedded) - Used to build 
Silicon Labs emlib for the EFM32.

See the [.travis.yml](https://github.com/RustyGecko/circle-game/blob/master/.travis.yml) of how this can
be done on a normal linux system.
