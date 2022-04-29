# World of Tanks Battle Results Parser

This project aims to parse World of Tanks battle results from two sources:
 - `.dat` files from World of Tanks cache folder
 - `.wotreplay` files from the game directory (Work in Progress)

Currently, this project produces a binary (See releases) that parses `.dat` files only.

## Getting Started
### Datfile Parser
 
1. Download latest Datfile parser for your platform of choice (Only windows officially tested) and extract the archive.

Run the `--help` command to see how to use the binary:
```
./wot_datfile_parser_cli --help
```

Run the `.exe` without any options to automatically parse the cache folder. You can also double click on it in Windows.
```
./wot_datfile_parser_cli
```
You can also parse a single `.dat` file like so:
```
./wot_datfile_parser_cli --file <FILE_NAME>
```

If you do not specify an output directory, it will output to the same directory as the `.exe`

### Build From Source
If you are familiar with the rust ecosystem you can build the executable like this:
```
cargo build --release --bin wot_datfile_parser_cli
```

## Examples
Output files of `wot_datfile_parser_cli` can be found [here](datfile_parser/examples)
## Supported Versions of World of Tanks
### Datfile parser
 - `1.15.0` - `1.16.1`


## Backwards Compatibility
### Datfile parser
Backwards compatibility is not guaranteed for the Datfile parser. This is not very important anyway since these files don't persist indefintely like `.wotreplay` files do.

## Credits
 - https://github.com/StranikS-Scan/WorldOfTanks-Decompiled
    - Makes it really easy to track changes in updates!
 - https://github.com/Phalynx/WoT-Replay-To-JSON
 - https://github.com/evido/wotreplay-parser
 - Contributors to [vbAddict Wiki](https://web.archive.org/web/20180407110623/http://wiki.vbaddict.net/pages/WoT_Developer_Wiki)
 - http://forum.worldoftanks.eu/index.php?/topic/185348-11011-wot-replay-analyzer-wip-1-08112020/
