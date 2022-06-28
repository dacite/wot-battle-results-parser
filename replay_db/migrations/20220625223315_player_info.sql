CREATE TABLE IF NOT EXISTS PlayerInfo
(
    replayID                  INT,
    avatarSessionID           INT,

    accountDBID               INT,

    clanAbbrev                TEXT,
    customRoleSlotTypeId      INT,
    fakeName                  TEXT,
    forbidInBattleInvitations BOOLEAN,
    igrType                   INT,
    isAlive                   BOOLEAN,
    isGodModeActive           INT,
    isTeamKiller              BOOLEAN,
    maxHealth                 INT,
    name                      TEXT,
    team                      INT,
    vehicleType               TEXT,
    wtr                       INT,


    PRIMARY KEY (replayID, avatarSessionID),
    FOREIGN KEY (replayID) REFERENCES ReplayCommon (replayID),
    UNIQUE (replayID, accountDBID),
    UNIQUE (replayID, name),
    UNIQUE (replayID, fakeName)
);