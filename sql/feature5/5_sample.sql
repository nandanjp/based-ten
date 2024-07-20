WITH YourRankedItems AS (
        SELECT li.username, li.itemID, l.listType
        FROM ListItems li JOIN Lists l ON li.username = l.username
            AND li.listName = l.listName
        WHERE l.username = 'jane.smith'
    ),
    SimilarLists AS (
        SELECT l.username, l.listName, COUNT(lk.likerName) AS likes
        FROM Lists l LEFT OUTER JOIN Likes lk ON l.username = lk.likingName
            AND l.listName = lk.listName
        JOIN YourRankedItems yri ON l.listType = yri.listType
        WHERE EXISTS (
            SELECT 1
            FROM ListItems li
            WHERE li.username = l.username AND li.listName = l.listName AND li.itemID IN (
                SELECT yri.itemID
                FROM YourRankedItems yri
            )
        )
        AND l.username != yri.username
        GROUP BY l.username, l.listName
    ),
    RecommendedLists AS (
        SELECT sl.username, sl.listName, sl.likes, COUNT(li.itemID) AS similarity
        FROM SimilarLists sl JOIN ListItems li ON sl.username = li.username
            AND sl.listName = li.listName
        WHERE li.itemID IN (
            SELECT yri.itemID
            FROM YourRankedItems yri
        )
        GROUP BY sl.username, sl.listName, sl.likes
    )
    
    SELECT * FROM RecommendedLists
    ORDER BY similarity DESC, likes DESC;
