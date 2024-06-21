WITH ListsWithItemIDs AS (
    SELECT l.email,
        l.listName,
        li.itemID,
        l.listType as itemType
    FROM ListItems li
        JOIN Lists l ON li.email = l.email
        AND li.listName = l.listName
),
ListsWithLikes AS (
    SELECT l.*
    FROM Lists l
        JOIN Likes lk on l.listName = lk.listName
),
ListLikeCounts AS (
    SELECT lwl.email,
        lwl.listName,
        COUNT(*) as likeCount
    FROM ListsWithLikes lwl
    GROUP BY lwl.email,
        lwl.listName
),
TotalLikesByItem AS (
    SELECT lwi.itemID,
        lwi.itemType,
        SUM(llc.likeCount) as totalLikes
    FROM ListsWithItemIDs lwi
        JOIN ListLikeCounts llc ON lwi.email = llc.email
        AND lwi.listName = llc.listName
    GROUP BY lwi.itemID,
        lwi.itemType
)
SELECT *
FROM Media m
    JOIN TotalLikesByItem l ON m.id = l.itemID
    AND m.type = l.itemType
WHERE m.type = 'movies'::ListType
ORDER BY l.totalLikes DESC;
