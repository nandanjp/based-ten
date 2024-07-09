WITH mutuals AS (
    SELECT f1.followingName
    FROM Follows f1, Follows f2
    WHERE f1.followerName = 'JaneSmith'
    AND f1.followingName = f2.followerName 
    AND f2.followingName = 'JaneSmith'
)
SELECT followingName,
CASE 
    WHEN followingName IN (SELECT * FROM mutuals) THEN TRUE
    ELSE FALSE
END as followsBack
FROM Follows
WHERE followerName = 'JaneSmith';
