WITH mutuals AS (
    SELECT f1.following
    FROM Follows f1, Follows f2
    WHERE f1.follower = 'jane.smith'
    AND f1.following = f2.follower 
    AND f2.following = 'jane.smith'
)
SELECT following,
CASE 
    WHEN following IN (SELECT * FROM mutuals) THEN TRUE
    ELSE FALSE
END as followsBack
FROM Follows
WHERE follower = 'jane.smith';
