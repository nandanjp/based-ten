-- Add up migration script here
CREATE TABLE Movies (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    mediaImage TEXT,
    createdOn DATE
);
CREATE TABLE Songs (
    id SERIAL PRIMARY KEY,
    title VARCHAR(30) NOT NULL,
    author VARCHAR(30),
    album VARCHAR(30),
    mediaImage TEXT,
    createdOn DATE
);
CREATE TABLE VideoGames (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    mediaImage TEXT,
    console VARCHAR(30),
    createdOn DATE
);
CREATE TABLE Anime (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    mediaImage TEXT,
    numEpisodes INT,
    createdOn DATE
);
CREATE TABLE Users (
    email VARCHAR(30) NOT NULL UNIQUE,
    userName VARCHAR(20) NOT NULL PRIMARY KEY,
    userPassword VARCHAR(30) NOT NULL,
    createdAt TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE Groups (
    gid SERIAL PRIMARY KEY,
    groupName VARCHAR(30) NOT NULL,
    ownedBy VARCHAR(30) NOT NULL REFERENCES Users(userName)
);
CREATE TABLE GroupMembers (
    userName VARCHAR(30) NOT NULL REFERENCES Users(userName),
    gid INT NOT NULL REFERENCES Groups(gid),
    PRIMARY KEY(userName, gid)
);
CREATE TYPE ListType AS ENUM ('anime', 'movies', 'songs', 'videogames');
CREATE TABLE Lists (
    userName VARCHAR(30) NOT NULL REFERENCES Users(userName),
    listName VARCHAR(30) NOT NULL,
    listType ListType NOT NULL,
    PRIMARY KEY(userName, listName),
    UNIQUE (userName, listName)
);
CREATE TABLE ListItems (
    userName VARCHAR(30) NOT NULL,
    listName VARCHAR(30) NOT NULL,
    rankingInList INT NOT NULL,
    itemID INT NOT NULL,
    PRIMARY KEY(userName, listName, itemID),
    FOREIGN KEY (userName, listName) REFERENCES Lists(userName, listName),
    CHECK(
        rankingInList <= 10
        AND rankingInList >= 1
    )
);
CREATE TABLE Likes (
    likerName VARCHAR(30) NOT NULL REFERENCES Users(userName),
    likingName VARCHAR(30) NOT NULL,
    listName VARCHAR(30) NOT NULL,
    PRIMARY KEY (likerName, likingName, listName),
    UNIQUE (likerName, likingName, listName),
    FOREIGN KEY (likingName, listName) REFERENCES Lists(userName, listName)
);
CREATE TABLE Follows (
    follower VARCHAR(30) NOT NULL REFERENCES Users(userName),
    following VARCHAR(30) NOT NULL REFERENCES Users(userName),
    PRIMARY KEY (follower, following)
);
CREATE VIEW Media AS
SELECT id,
    title,
    mediaImage,
    createdOn,
    'movies'::ListType AS type
FROM Movies
UNION
SELECT id,
    title,
    mediaImage,
    createdOn,
    'songs'::ListType AS type
FROM Songs
UNION
SELECT id,
    title,
    mediaImage,
    createdOn,
    'videogames'::ListType AS type
FROM VideoGames
UNION
SELECT id,
    title,
    mediaImage,
    createdOn,
    'anime'::ListType AS type
FROM Anime;
-- Checking if a listitem is being added to a list that already contains the same rank
CREATE OR REPLACE FUNCTION check_rank_exists() RETURNS TRIGGER AS $$ BEGIN IF EXISTS (
        SELECT 1
        FROM ListItems
        WHERE userName = NEW.userName
            AND listName = NEW.listName
            AND rankingInList = NEW.rankingInList
    ) THEN RAISE EXCEPTION 'An item with the same rank has already been added to the list';
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER ItemRankIsUnique BEFORE
INSERT ON ListItems FOR EACH ROW EXECUTE FUNCTION check_rank_exists();