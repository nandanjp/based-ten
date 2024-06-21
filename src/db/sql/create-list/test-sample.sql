SELECT *
FROM VideoGames
WHERE title LIKE 'God%';
SELECT *
FROM Users
WHERE email = 'alice.johnson@example.com';
INSERT INTO Lists
VALUES (
        'alice.johnson@example.com',
        'Worst Video Games',
        'videogames'
    )
RETURNING email,
    listName,
    listType;
INSERT INTO ListItems
VALUES (
        'alice.johnson@example.com',
        'Worst Video Games',
        3
    )
RETURNING itemID,
    email,
    listName;