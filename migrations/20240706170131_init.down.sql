-- Add down migration script here
DROP TABLE IF EXISTS Follows;
DROP TABLE IF EXISTS Likes;
DROP TABLE IF EXISTS ListItems;
DROP TABLE IF EXISTS Lists;
DROP TABLE IF EXISTS GroupMembers;
DROP TABLE IF EXISTS Groups;
DROP VIEW IF EXISTS Media;
DROP TABLE IF EXISTS Users;
DROP TABLE IF EXISTS Movies;
DROP TABLE IF EXISTS Songs;
DROP TABLE IF EXISTS VideoGames;
DROP TABLE IF EXISTS Anime;
DROP TYPE IF EXISTS ListType;