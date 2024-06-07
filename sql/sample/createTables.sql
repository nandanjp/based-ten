CREATE TABLE Movies
{
  movieID INT NOT NULL PRIMARY KEY,
  title VARCHAR(30),
  mediaImega TEXT,
  createdOn DATE,
};

CREATE TABLE Songs
{
  songID INT NOT NULL PRIMARY KEY,
  title VARCHAR(30),
  author VARCHAR(30),
  album VARCHAR(30),
  mediaImega TEXT,
  createdOn DATE,
};

CREATE TABLE VideoGames
{
  videoGameID INT NOT NULL PRIMARY KEY,
  title VARCHAR(30),
  mediaImega TEXT,
  createdOn DATE,
  platform VARCHAR(30),
};

CREATE TABLE Anime
{
  animeId INT NOT NULL PRIMARY KEY,
  title VARCHAR(30),
  mediaImega TEXT,
  createdOn DATE,
  episodes INT,
};
