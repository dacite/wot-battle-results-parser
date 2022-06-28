CREATE TABLE IF NOT EXISTS ReplayCommon
(
    replayID             INT     NOT NULL,
    battleType           INT     NOT NULL,
    arenaUniqueID        TEXT,
    clientVersionFromExe TEXT    NOT NULL,
    clientVersionFromXml TEXT    NOT NULL,
    dateTime             TEXT    NOT NULL,
    gameplayID           TEXT    NOT NULL,
    mapDisplayName       TEXT    NOT NULL,
    mapName              TEXT    NOT NULL,
    playerID             INT     NOT NULL,
    playerName           TEXT    NOT NULL,
    playerVehicle        TEXT    NOT NULL,
    regionCode           TEXT    NOT NULL,
    serverName           TEXT    NOT NULL,
    isComplete           BOOLEAN NOT NULL,
    PRIMARY KEY (replayID)
);