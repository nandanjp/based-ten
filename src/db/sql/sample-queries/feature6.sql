WITH RECURSIVE Circles AS (
    (SELECT g.gid, g.groupName, g.ownedBy
    FROM Groups g
    WHERE g.gid = 1)
    UNION
    (
        SELECT gm2.gid, g.groupName, g.ownedBy
        FROM Circles c JOIN GroupMembers gm1 ON c.gid = gm1.gid
        JOIN GroupMembers gm2 ON gm1.email = gm2.email
        JOIN Groups g ON gm2.gid = g.gid
    )
)

SELECT * FROM Circles
ORDER BY gid;