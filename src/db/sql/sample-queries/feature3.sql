WITH ListsWithItemIDs AS (
    SELECT Lists.email,
        Lists.listName,
        itemID,
        listType as itemType
    FROM ListItems
        JOIN Lists ON ListItems.email = Lists.email
        AND ListItems.listName = Lists.listName
),
ListLikeCounts AS (
    SELECT Lists.email,
        Lists.listName,
        listType,
        COUNT(*) as likeCount
    FROM (
            SELECT *
            FROM Lists
                JOIN Likes on Lists.listName = Likes.listName
        )
    GROUP BY Lists.email,
        Lists.listName,
        listType
),
TotalLikesByItem AS (
    SELECT itemID,
        itemType,
        SUM(likeCount) as totalLikes
    FROM ListsWithItemIDs lwi
        JOIN ListLikeCounts llc ON lwi.email = llc.email
        AND lwi.listName = llc.listName
    GROUP BY itemID,
        itemType
)
SELECT *
FROM Media m
    JOIN TotaLikesByItem l ON m.itemID = l.itemID
    AND m.type = l.itemType
WHERE m.type = 'movie';