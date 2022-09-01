# Just Write

Micro-journaling tool, a rewrite of [jw](https://git.josias.dev/jw).

Many journaling programs are designed for writing long entries. This is for writing out your thoughts Twitter-style, with a CLI for writing, searching, and editing posts.

This tool provides a distraction-free environment for writing out thoughts quickly to save for later. It has been developed for my own personal journaling, but you may find it useful as well.

See this [blog post](https://josias.dev/posts/justwrite) for an outline of the philosophy of this project.

**Note:** this project was just rewritten. The new codebase may be unstable. Use at your own risk.

Principles:
* Posting is as simple as opening, writing, and closing
* All other work should be done by the tool

Features:
* Full text search
* Categorization
* Post tagging

In development:
* Tauri GUI

## Installation

Quick installation of the CLI (requires rustc and cargo to be installed):

```sh
$ git clone https://git.josias.dev/just-write && cd just-write
$ make install-cli
$ jw help
```

It should run on any system supported by Rust, but only Linux has been tested.

## Usage

This tool is designed around notebooks, which are collections of posts.

Here we make a notebook called "journal".

```sh
$ jw new journal`
```

To write a post, run `jw post journal`. This opens your default editor. If `EDITOR` isn't found, it resorts to `vi`. When you are done, save and quit. The tool handles the rest.

You can run a full-text search on your posts with the `search` subcommand. It returns a list of paths in which the query was found.

## Configuration

Just Write is designed to not require configuration in most situations to enable writing without hassle, but certain behavior can be configured if desired.

Configuration is stored in its own directory on different platforms. On Linux and BSD it can normally be found at `~/.config/justwrite/config.toml`.

```toml
root = "~/Documents/just-write"
```

This example configuration sets the notebook directory ("root") to `~/Documents/just-write`. There are currently no further configuration options.

### Notebooks

Notebooks are collections of posts stored in the program's `root` directory. They are individually configured.

Each notebook has its own `notebook.toml` in its corresponding directory. 

```toml
name = "journal"
description = "General notes and random thoughts"
file_path = "%Y-%m-%d-%s.md" # the path of each individual file, relaced with the date and the first text of the post (`%s`).
metadata = true # add the timestamp to the beginning of each post
```

Notebooks are given a basic configuration when initialized by the `new` subcommand.

## Similar Projects 

- [ThotKeeper](https://github.com/cmpilato/thotkeeper)
- [jrnl](https://github.com/jrnl-org/jrnl/)
- [jrny](https://git.sr.ht/~detondev/jrny)
- [nb](https://github.com/xwmx/nb)

## Mirrors

* [git.josias.dev](https://git.josias.dev/just-write)
* [Codeberg](https://codeberg.org/josias/just-write)
* [GitHub](https://github.com/justjosias/just-write)

## License

Copyright (C) 2022 Josias Allestad

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
