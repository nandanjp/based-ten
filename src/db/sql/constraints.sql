CREATE ASSERTION ListItemIsMedia
CHECK (
  FROM Lists JOIN ListItems ON Lists.email = ListItems.email AND Lists.listName = ListItems.listName,
  WHERE ListItems.itemID in (SELECT id FROM Media)
);