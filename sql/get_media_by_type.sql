-- Feature 3
WITH ListsWithItemIDs AS (
    SELECT l.userName,
        l.listName,
        li.itemID,
        l.listType as itemType
    FROM ListItems li
        JOIN Lists l ON li.userName = l.userName
        AND li.listName = l.listName
),
ListsWithLikes AS (
    SELECT l.*
    FROM Lists l
        JOIN Likes lk on l.listName = lk.listName
),
ListLikeCounts AS (
    SELECT lwl.userName,
        lwl.listName,
        COUNT(*) as likeCount
    FROM ListsWithLikes lwl
    GROUP BY lwl.userName,
        lwl.listName
),
TotalLikesByItem AS (
    SELECT lwi.itemID,
        lwi.itemType,
        CAST(SUM(llc.likeCount) AS INTEGER) as totalLikes
    FROM ListsWithItemIDs lwi
        JOIN ListLikeCounts llc ON lwi.userName = llc.userName
        AND lwi.listName = llc.listName
    GROUP BY lwi.itemID,
        lwi.itemType
)
SELECT id, title, mediaimage, createdon, type AS "type: ListType", totalLikes
FROM Media m
    JOIN TotalLikesByItem l ON m.id = l.itemID
    AND m.type = l.itemType
WHERE m.type = $1::ListType
ORDER BY l.totalLikes DESC;
