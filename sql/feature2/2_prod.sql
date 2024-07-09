SELECT l.listName, l.listType, l.username
    FROM (GroupMembers m JOIN Users u ON m.username = u.username) JOIN Lists l
        ON u.username = l.username
    WHERE m.gid = 1
    ORDER BY l.listType;
