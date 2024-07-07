WITH RECURSIVE Circles AS (
    (g.gid, g.groupName, g.ownedBy
    FROM Groups g
    WHERE g.gid = 1)
    UNION
    (
        SELECT gm.gid, g.groupName, g.ownedBy
        FROM Circles c JOIN GroupMembers gm ON c.gid = gm.gid
        JOIN Groups g ON gm.gid = g.gid
        WHERE AND gm.email IN (
            SELECT u.email
            FROM Users u JOIN GroupMembers gm ON u.email = gm.email
            WHERE gm.gid = 1 AND u.email != 'john.doe@example.com'
            LIMIT 3
        )
    )
)

SELECT * FROM Circles
ORDER BY gid;