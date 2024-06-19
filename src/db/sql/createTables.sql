CREATE TABLE Movies (
    id INT NOT NULL PRIMARY KEY,
    title VARCHAR(30) NOT NULL,
    mediaImega TEXT,
    createdOn DATE
);
CREATE TABLE Songs (
    id INT NOT NULL PRIMARY KEY,
    title VARCHAR(30) NOT NULL,
    author VARCHAR(30),
    album VARCHAR(30),
    mediaImega TEXT,
    createdOn DATE
);
CREATE TABLE VideoGames (
    id INT NOT NULL PRIMARY KEY,
    title VARCHAR(30) NOT NULL,
    mediaImega TEXT,
    createdOn DATE,
    console VARCHAR(30)
);
CREATE TABLE Anime (
    id INT NOT NULL PRIMARY KEY,
    title VARCHAR(30) NOT NULL,
    mediaImega TEXT,
    createdOn DATE,
    numEpisodes INT
);
CREATE TABLE Users (
    email VARCHAR(30) NOT NULL PRIMARY KEY,
    displayName VARCHAR(20) NOT NULL,
    userPassword VARCHAR(30) NOT NULL
);
CREATE TABLE Groups (
    gid INT NOT NULL PRIMARY KEY,
    groupName VARCHAR(30) NOT NULL,
    ownedBy VARCHAR(30) NOT NULL REFERENCES Users(email)
);
CREATE TABLE GroupMembers(
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
CREATE TABLE ListItems(
    email VARCHAR(30) NOT NULL,
    listName VARCHAR(30) NOT NULL,
    itemID INT NOT NULL,
    FOREIGN KEY (email, listName) REFERENCES Lists(email, listName),
    UNIQUE (email, listName)
);
CREATE TABLE Likes(
    likerEmail VARCHAR(30) NOT NULL REFERENCES Users(email),
    likingEmail VARCHAR(30) NOT NULL,
    listName VARCHAR(30) NOT NULL,
    PRIMARY KEY (likerEmail, likingEmail, listName),
    UNIQUE (likerEmail, likingEmail, listName),
    FOREIGN KEY (likingEmail, listName) REFERENCES ListItems(email, listName)
);
CREATE TABLE Follows(
    followerEmail VARCHAR(30) NOT NULL REFERENCES Users(email),
    followingEmail VARCHAR(30) NOT NULL REFERENCES Users(email),
    PRIMARY KEY (followerEmail, followingEmail)
);