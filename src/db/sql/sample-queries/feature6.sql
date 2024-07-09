WITH RECURSIVE Circles AS (
    (SELECT g.gid, g.groupName, g.ownedBy, 1 AS level
    FROM Groups g
    WHERE g.gid = 1)
    UNION
    (
        SELECT gm2.gid, g.groupName, g.ownedBy, 1 + level AS level
        FROM Circles c JOIN GroupMembers gm1 ON c.gid = gm1.gid
        JOIN GroupMembers gm2 ON gm1.email = gm2.email
        JOIN Groups g ON gm2.gid = g.gid
        WHERE level < 3
    )
)

SELECT DISTINCT gid, groupName, ownedBy FROM Circles
ORDER BY gid;