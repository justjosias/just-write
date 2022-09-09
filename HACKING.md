# Hacking

Thanks for your interest! This guide should help you get started with the codebase.

The code is separated into three crates:
* `gui`: the Tauri-powered Web application
* `cli`: the command-line interface
* `internals`: code shared between both

The GUI is under massive construction at the moment, so you probably want to work with the CLI and the internals.

## CLI

Building the CLI in particular (without compiling the GUI), can be done with `cargo build --path cli` or `make cli-build`.

Installation of the CLI can be done with `cargo install --path cli` or with `make cli-install`, which also installs Bash completions.

## GUI

The GUI is unstable and may mess up your Just Write configuration. Be sure to set it up in an isolated environment if you'd like to test it.

First, install [Tauri](https://tauri.app):
```sh
$ cargo install cargo-tauri
```

This will compile Tauri from source. You can find other methods of installing on their website.

When Tauri is installed, you can run the GUI with `cargo tauri dev`. Building system packages (.debs and AppImages) is done with `cargo tauri build`. These can be run from within the `gui` directory.

The source for the application is divided between the Rust code, which interacts with the system and binds to `jw-internals` (`src-tauri`), and the JavaScript-based UI (`ui`). These are bundled together when the application is built.
