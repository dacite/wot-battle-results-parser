CREATE TABLE IF NOT EXISTS Achievements
(
    arenaUniqueID   INT,
    avatarSessionID INT,
    accountDBID     INT,
    achievementID   INT,

    PRIMARY KEY (arenaUniqueID, avatarSessionID, achievementID),
    FOREIGN KEY (arenaUniqueID) REFERENCES Battle (arenaUniqueID)
)