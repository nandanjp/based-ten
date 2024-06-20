CREATE VIEW Media AS
    SELECT id, title, mediaImage, createdOn, 'movie' AS type FROM Movies
    UNION
    SELECT id, title, mediaImage, createdOn, 'song' AS type FROM Songs
    UNION
    SELECT id, title, mediaImage, createdOn, 'videogame' AS type FROM VideoGames
    UNION
    SELECT id, title, mediaImage, createdOn, 'anime' AS type FROM Anime;

SELECT * FROM Media WHERE lower(title)  LIKE 'Breath%';
SELECT * FROM VideoGames WHERE lower(title) LIKE 'God%';
INSERT INTO Lists VALUES ('john.doe@example.com', 'Worst Video Games', 'videogames');
INSERT INTO ListItems VALUES ('john.doe@example.com', 'Worst Video Games', 3);
