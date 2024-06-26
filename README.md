# Vampire-Charact-rs

Character sheet management for Vampire: the Masquerade, written in Rust

<!-- Note: Markdown *reference* links don't seem to work on GitHub README files, so always use the [default](https://example.org) format. -->

## Getting Started


<!-- ### Dependencies

*
-->

### Installing

```
cargo build --release
```

### Giving me feedback

If you want to help out, I'd love if you open issues on GitHub, create Pull requests, or try the program and give me feedback in any way! If this is your first time working with code, check out [First Time Contributing to an Open Source Project](docs/first-time-huh.md).


At the time of writing, the app cannot yet _create_ character sheets for you (I'm working on that, see [branch `operating_modes/add`](https://github.com/Ablesius/vampire-charact-rs/tree/operating_modes/add)); so in order to create them, you'd best copy one of the sample characters from [tests/sample_character_dir](tests/sample_character_dir/).

## Executing program

Your character sheets have to be json files.
**Note** that they also must have the `.json` file ending, or vampire-charact-rs will not recognise them or consider them character sheets. You should also not have any other json files in this directory.

**Note for beginners**: If you're annoyed by the cargo output "Finished `release` profile..." etc., you can disable it per run with the `-q` flag, or permanently. See `cargo help run` for information.

### Modes

The program has a couple of different operating modes, which perform different actions.
At the moment, the mode has to be provided for the program to understand the command (later this might change so that the operating mode can be inferred from the passed parameters, but not now).

#### List available characters

```
cargo run --release -- list path/to/a/directory
```
Lists out characters found in the directory by printing the player's and character's name for each character in the directory.

#### Print character details
```
cargo run --release -- print path/to/character.json
```
Prints all the character details noted in the file. For ease of use, it will currently just print the whole contents of the json file without any pretty formatting.

##### Create character interactively
```
cargo run --release -- create
```
Enters an interactive mode where you can create a character from scratch.

## Help

<!-- Any advice for common problems or issues. -->

## Authors

Contributors names and contact info

### Lexi Blesius

- Github: [Ablesius](https://github.com/Ablesius)
- Mastodon: [@luyin@lgbtqia.space ](https://lgbtqia.space/@luyin)

## Acknowledgments

<!-- Inspiration, code snippets, etc. -->
*
