PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS items(
    id        INTEGER PRIMARY KEY,
    pn        TEXT,
    name      TEXT,
    maturity  INTEGER,
    version   INTEGER,
    type      INTEGER,
    UNIQUE(pn, version)
);
    
CREATE TABLE IF NOT EXISTS children(
    id_parent  INTEGER,
    id_child   INTEGER,
    quantity   INTEGER,
    FOREIGN KEY(id_parent) REFERENCES items(id),
    FOREIGN KEY(id_child) REFERENCES items(id),
    UNIQUE(id_parent, id_child)
);
    
CREATE VIEW IF NOT EXISTS view_children AS
    SELECT
        items.id, 
        items.pn, 
        items.name, 
        items.version,
        items.maturity,
        items.type,
        children.quantity,
        children.id_parent
    FROM items, children 
    WHERE children.id_child = items.id;
    
CREATE VIEW IF NOT EXISTS view_where_used AS
    SELECT
        children.id_parent as id,
        items.pn, 
        items.name,
        items.version,
        items.maturity,
        items.type,
        children.id_child
    FROM items, children 
    WHERE children.id_parent = items.id;

CREATE TABLE IF NOT EXISTS config(
    key       TEXT PRIMARY KEY,
    value     TEXT
);