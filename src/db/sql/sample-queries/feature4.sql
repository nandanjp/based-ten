WITH mutuals AS (
    SELECT f1.followingEmail
    FROM Follows f1, Follows f2
    WHERE f1.followerEmail = 'jane.smith@example.net'
    AND f1.followingEmail = f2.followerEmail 
    AND f2.followingEmail = 'jane.smith@example.net'
)
SELECT followingEmail,
CASE 
    WHEN followingEmail IN (SELECT * FROM mutuals) THEN TRUE
    ELSE FALSE
END as followsBack
FROM Follows
WHERE followerEmail = 'jane.smith@example.net';