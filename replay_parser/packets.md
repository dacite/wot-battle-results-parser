`0x18` - Game Version
    - length: u32
    - str of length

`0x00` - Player Stuff?
    - 10 bytes of unknown data
    - length: u32 (for the rest of the packet)
    - [playerNameStringLength: u8, playerNameString]
    - [sessionIDStrLen: u8, sessionIDStr]
    - arenaUniqueID: u64
    - arenaTypeID i32
    - arenaBonusType: u8
    - arenaGuiType: u8
    - [pickleLen: u8, pickle]
    - weatherPresetID: u8