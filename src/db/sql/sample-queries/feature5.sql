-- 1. Find lists that have items in common with your recently ranked items
-- 2. Order lists by most likes
-- 3. Find a score for each list based on Similarity and Popularity. Have this score in a column next to List ID
-- 4. Return the top 10 recommended lists

WITH YourRankedItems AS (
    SELECT li.email, li.itemID, l.listType
    FROM ListItems li JOIN Lists l ON li.email = l.email AND li.listName = l.listName
    WHERE l.email = 'example_email'
),
SimilarLists AS (
    SELECT li.email, li.listName, COUNT(li.itemID) AS similarity, COUNT(lk.likerEmail) AS likes
    FROM ListItems li LEFT JOIN Likes lk ON li.email = lk.likingEmail AND li.listName = lk.listName
    WHERE li.itemID IN (
        SELECT yri.itemID
        FROM YourRankedItems yri
    )
    GROUP BY li.email, li.listName
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


