CREATE TABLE IF NOT EXISTS im_table (
    autoId INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    fromId TEXT,
    toId TEXT,
    time INT,
    format INT,
    sid TEXT UNIQUE
);