# PNGMe: Encode and Decode Secret Messages in PNG Files

A simple PNG message encoder/decoder written in Rust. This project is based on the guide [pngme]: https://picklenerd.github.io/pngme_book/.

```shell   
# Installation (requires Rust)
cargo install --git https://github.com/brianjsl/pngme
```

# Usage

The full options are shown below: 

```shell
Usage: pngme <COMMAND>

Commands:
  encode  Encodes a message string of a given PNG chunk type to a specified a PNG file
  decode  Decodes encoded message strings of a specified PNG chunk type from a specified PNG file
  remove  Removes encoded messages of a specified PNG chunk type from a specified PNG file
  print   Prints a list of PNG chunks that can be searched for messages
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Examples

To encode the message `"sEcReT meSsAgE"` in a file called `dice.png` under chunk type `ruSt`:

```shell
./pngme encode dice.png ruSt "sEcReT meSsAgE" dice_secret.png
```

To decode the same message:

```shell
./pngme encode dice.png ruSt "sEcReT meSsAgE" dice_secret.png
```

To remove the message:

```shell
./pngme remove dice_secret.png ruSt       
```

To print all available chunk types that can be searched for secret messages:

```shell
./pngme print dice_secret.png      
```
