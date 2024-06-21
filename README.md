# Vampire-Charact-rs

Character sheet management for Vampire: the Masquerade, written in Rust

<!-- Note: Markdown *reference* links don't seem to work on Github README files, so always use the [default](https://example.org) format. -->

## Getting Started


<!-- ### Dependencies

*
-->

### Installing

```
cargo build --release
```

### Executing program

Your character sheets have to be json files.
**Note** that they also must have the `.json` file ending, or vampire-charact-rs will not recognise them or consider them character sheets. You should also not have any other json files in this directory.

#### Modes

The program has a couple of different operating modes, which perform different actions.
At the moment, the mode has to be provided for the program to understand the command (later this might change so that the operating mode can be inferred from the passed parameters, but not now).

##### List available characters
    ```
    cargo run --release -- list path/to/a/directory
    ```
    Lists out characters found in the directory by printing the player's and character's name for each character in the directory.

##### Print character details
   ```
   cargo run --release -- print path/to/character.json
   ```
   Prints all the character details noted in the file. For ease of use, it will currently just print the whole contents of the json file without any pretty formatting.

##### Add character
   ```
   cargo run --release -- add [path/to/not/existing/character.json]
   ```
   Enters an interactive mode where you can set up a character from scratch.

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
