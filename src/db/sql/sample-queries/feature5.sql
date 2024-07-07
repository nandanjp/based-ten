-- 1. Find lists that have items in common with your recently ranked items
-- 2. Order lists by most likes
-- 3. Find a score for each list based on Similarity and Popularity. Have this score in a column next to List ID
-- 4. Return the top 10 recommended lists

WITH YourRankedItems AS (
    SELECT li.email, li.itemID, l.listType
    FROM ListItems li JOIN Lists l ON li.email = l.email AND li.listName = l.listName
    WHERE l.email = 'jane.smith@example.net'
),
SimilarLists AS (
    SELECT l.email, l.listName, COUNT(lk.likerEmail) AS likes
    FROM Lists l JOIN Likes lk ON l.email = lk.likingEmail AND l.listName = lk.listName
    JOIN YourRankedItems yri ON l.listType = yri.listType
    WHERE EXISTS (
        SELECT 1
        FROM ListItems li
        WHERE li.email = l.email AND li.listName = l.listName AND li.itemID IN (
            SELECT yri.itemID
            FROM YourRankedItems yri
        )
    )
    AND l.email != yri.email
    GROUP BY l.email, l.listName
),
SimilarListsWithSimilarity AS (
    SELECT sl.email, sl.listName, sl.likes, COUNT(li.itemID) AS similarity
    FROM SimilarLists sl JOIN ListItems li ON sl.email = li.email AND sl.listName = li.listName
    WHERE li.itemID IN (
        SELECT yri.itemID
        FROM YourRankedItems yri
    )
    GROUP BY sl.email, sl.listName, sl.likes
),
TotalUsers AS (
    SELECT COUNT(DISTINCT email) AS totalUsers
    FROM Users
),
ListsByScore AS (
    SELECT sl.email, sl.listName, (sl.similarity / 10) + (sl.likes / tu.totalUsers) AS score
    FROM SimilarListsWithSimilarity sl, TotalUsers tu
)

SELECT * FROM SimilarLists;

-- SELECT lbs.email, lbs.listName, lbs.score
-- FROM ListsByScore lbs
-- ORDER BY lbs.score DESC
-- LIMIT 10;


