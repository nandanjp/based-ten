SELECT l.email, l.listName
FROM GroupMembers g, Lists l
WHERE g.gid = 1 AND g.email = l.email;
