# Lear: A Shakespearean Command Line Utility

Lear is a command line &ldquo;utility&rdquo; that you can use to amuse yourself and your friends. Whenever you mistype `clear` into the terminal, `lear` will dutifully print out a passage from everyone&rsquo;s favorite tragedy about a mad king.

This idea was loosely inspired by [`sl` (Steam Locomotive)](https://github.com/mtoyoda/sl), [Fortune](<https://en.wikipedia.org/wiki/Fortune_(Unix)>), and all of the other weird command line scripts out there.

## Usage

To get started, type `lear` into the console (preferably by accident).

![a demonstration of lear, with a passage from act 3 scene 2 printed to the console](/img/lear-demo.png)

To print a specific passage, use the `quote` subcommand. It accepts the act, scene, starting line, and ending line as positional arguments. This example quotes lines 7&ndash;8 from Act 3, Scene 3.

```console
$ lear quote 3 3 7 8
EDMUND
    Most savage and unnatural!

GLOUCESTER
    Go to; say you nothing. There's a division betwixt
    ...

                                                            (Lr. 3.3.7-8)
```

You can use the `contents` command to print a table of contents, which will show you which acts and scenes are available, as well as how many lines they contain.

```console
$ lear contents
 Act  Scene  Lines
-------------------
  1     1     332
        2     191
        3     27
...
```

## Installation

If you have `cargo` installed on your machine already, `lear` can be installed with

```console
$ cargo install lear
```

A homebrew installation is hopefully coming soon.

## Contributing

If you would like to contribute to this tool, please open an issue or submit a pull request

## License

This project is dual licensed under MIT and Apache 2.0.
