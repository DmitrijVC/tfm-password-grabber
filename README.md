# Transformice Password Grabber
Shitty DLL that tries to get a user's password from memory of the game "[Transformice](https://www.transformice.com/)".

It won't find the password every time, and I am too lazy for continuing the development of this library. <br>
*Maybe in the future...*

This project was made for fun as a part of my learning process of game hacking.

## Usage

### Build / Download
To download this DLL, go to *"Releases"* found at the right.

To build this DLL, clone the repo adn run: <br>
`cargo build --release --target=i686-pc-windows-msvc`

### Inject
Any Injector can be used, but if you are wondering, I was using `GH Injector`. <br>
You can download it from [here](https://guidedhacking.com/resources/guided-hacking-dll-injector.4/).

If injection was successful, tfm process' console should appear.

## Example
<img src="https://i.imgur.com/TILgBlL.png">

## ­
**Special thanks** to [Guided Hacking](https://guidedhacking.com) for their [Game Hacking Bible](https://guidedhacking.com/threads/ghb0-game-hacking-bible-introduction.14450) which is the best learning source for game hacking.
