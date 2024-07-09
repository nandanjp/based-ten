WITH mutuals AS (
    SELECT f1.following
    FROM Follows f1, Follows f2
    WHERE f1.follower = 'john.smith'
    AND f1.following = f2.follower 
    AND f2.following = 'john.smith'
)
SELECT following,
CASE 
    WHEN following IN (SELECT * FROM mutuals) THEN TRUE
    ELSE FALSE
END as followsBack
FROM Follows
WHERE follower = 'john.smith';
