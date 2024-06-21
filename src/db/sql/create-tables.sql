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
    email VARCHAR(30) NOT NULL PRIMARY KEY,
    displayName VARCHAR(20) NOT NULL,
    userPassword VARCHAR(30) NOT NULL,
    createdAt TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE Groups (
    gid SERIAL PRIMARY KEY,
    groupName VARCHAR(30) NOT NULL,
    ownedBy VARCHAR(30) NOT NULL REFERENCES Users(email)
);
CREATE TABLE GroupMembers (
    email VARCHAR(30) NOT NULL REFERENCES Users(email),
    gid INT NOT NULL REFERENCES Groups(gid),
    PRIMARY KEY(email, gid)
);
CREATE TYPE ListType AS ENUM ('anime', 'movies', 'songs', 'videogames');
CREATE TABLE Lists (
    email VARCHAR(30) NOT NULL REFERENCES Users(email),
    listName VARCHAR(30) NOT NULL,
    listType ListType NOT NULL,
    PRIMARY KEY(email, listName),
    UNIQUE (email, listName)
);
CREATE TABLE ListItems (
    email VARCHAR(30) NOT NULL,
    listName VARCHAR(30) NOT NULL,
    rankingInList INT NOT NULL,
    itemID INT NOT NULL,
    PRIMARY KEY(email, listName, itemID),
    FOREIGN KEY (email, listName) REFERENCES Lists(email, listName)
);
CREATE TABLE Likes (
    likerEmail VARCHAR(30) NOT NULL REFERENCES Users(email),
    likingEmail VARCHAR(30) NOT NULL,
    listName VARCHAR(30) NOT NULL,
    PRIMARY KEY (likerEmail, likingEmail, listName),
    UNIQUE (likerEmail, likingEmail, listName),
    FOREIGN KEY (likingEmail, listName) REFERENCES Lists(email, listName)
);
CREATE TABLE Follows (
    followerEmail VARCHAR(30) NOT NULL REFERENCES Users(email),
    followingEmail VARCHAR(30) NOT NULL REFERENCES Users(email),
    PRIMARY KEY (followerEmail, followingEmail)
);
CREATE VIEW Media AS
SELECT id,
    title,
    mediaImage,
    createdOn,
    'movie' AS type
FROM Movies
UNION
SELECT id,
    title,
    mediaImage,
    createdOn,
    'song' AS type
FROM Songs
UNION
SELECT id,
    title,
    mediaImage,
    createdOn,
    'videogame' AS type
FROM VideoGames
UNION
SELECT id,
    title,
    mediaImage,
    createdOn,
    'anime' AS type
FROM Anime;