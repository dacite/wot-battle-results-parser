# World of Tanks Battle Results Parser

## DOCS Work in Progress

This project aims to parse World of Tanks battle results from two sources:
 - `.dat` files from World of Tanks cache folder (Work in Progress)
 - `.wotreplay` files from the game directory (Work in Progress)

Currently I am making major changes to this repo and is therefore a work in progress
## Supported Versions of World of Tanks
### Datfile parser
 - `1.15.0` - `1.17.0`

## Projects that use this library:
- Wot Packet Analyzer: https://dacite.github.io/wot-packet-analyzer
   - An analyzer for packets in a `.wotreplay` files. Useful for development
   - Uses `wot_replay_parser` library
   
## Backwards Compatibility
### Datfile parser
Backwards compatibility is not guaranteed for the Datfile parser. This is not very important anyway since these files don't persist indefintely like `.wotreplay` files do.

## Credits
 - https://github.com/Monstrofil/replays_unpack
    - Everything on `.def` files comes from here
 - https://github.com/lkolbly/wows-replays
 - https://github.com/StranikS-Scan/WorldOfTanks-Decompiled
    - Makes it really easy to track changes in updates!
 - https://github.com/Phalynx/WoT-Replay-To-JSON
 - https://github.com/evido/wotreplay-parser
 - Contributors to [vbAddict Wiki](https://web.archive.org/web/20180407110623/http://wiki.vbaddict.net/pages/WoT_Developer_Wiki)
 - http://forum.worldoftanks.eu/index.php?/topic/185348-11011-wot-replay-analyzer-wip-1-08112020/
