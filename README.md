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


## Executing program

### Creating a new character interactively

In order to do this, run the programm with the `create` command:

```bash
cargo run --release -- create
```

An interactive, command-line creation process starts that asks you a couple of questions and results in writing a JSON file as output.

### Printing details from character sheets

```
cargo run --release -- print path/to/character.json
```
Prints (almost) all the character details noted in the file. (At the moment, this is not complete, but I'm working on that sometime soon)

**Note** Your character sheets have to be json files. They also must have the `.json` file ending, or vampire-charact-rs will not recognise them or consider them character sheets. You should also not have any other json files in this directory, otherwise the program might try to read them and that could result in strange errors.

### Listing characters in a directory

```
cargo run --release -- list path/to/a/directory
```

Lists out characters found in the directory by printing the player's and character's name for each character in the directory.

## Help

<!-- Any advice for common problems or issues. -->

## Contributing

See [Contributing.md](docs/Contributing.md).


## Authors

Contributors names and contact info

### Lexi Blesius

- Github: [Ablesius](https://github.com/Ablesius)
- Mastodon: [@luyin@lgbtqia.space ](https://lgbtqia.space/@luyin)

## Acknowledgments

<!-- Inspiration, code snippets, etc. -->
* [The One App](https://gitlab.com/the-one-app/the-one-app) by Loic Hausammann - while the RPG system is very different, the whole idea of The One App is very similar to mine, so I draw, while not outright code, but a lot of inspiration from this!
