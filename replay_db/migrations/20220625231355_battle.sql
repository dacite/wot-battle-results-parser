CREATE TABLE IF NOT EXISTS Battle
(
    arenaUniqueID        TEXT,
    battleType           INT NOT NULL,
    gameplayID           TEXT NOT NULL,
    mapDisplayName       TEXT NOT NULL,
    mapName              TEXT NOT NULL,
    regionCode           TEXT NOT NULL,
    serverName           TEXT NOT NULL, 
    
    PRIMARY KEY (arenaUniqueID)
);