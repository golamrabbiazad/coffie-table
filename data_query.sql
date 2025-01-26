-- INSERTING ITEM TO COFFIE TABLE SQLite
INSERT INTO coffie (id, name, description, count) 
VALUES 
    (5, 'HOT COFFIE 0', 'ITS TOTALLY HOT', 6),
    (6, 'HOT COFFIE 4', 'ITS HOT', 20),
    (7, 'HOT COFFIE 7', 'ITS TOTALLY', 11);


-- QUERY TO COFFIE TABLE
SELECT name FROM coffie WHERE coffie.count < 11;

-- TOTAL COFFIE ORDER COUNT
SELECT count(id) FROM coffie WHERE coffie.count  = 1 OR coffie.count > 1;

-- QUERY USER FEEDBACK FOR COFFIE
SELECT id, name FROM coffie WHERE coffie.description LIKE '%Awesome%';