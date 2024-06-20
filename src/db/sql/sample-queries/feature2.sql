SELECT l.listName, l.listType, l.email
FROM (GroupMembers m JOIN Users u ON m.email = u.email) JOIN Lists l
    ON u.email = l.email
WHERE m.gid = 5
ORDER BY l.listType;
