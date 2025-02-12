CREATE TABLE IF NOT EXISTS location_types  (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS locations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    barcode VARCHAR(255),
    location_type_id INT NOT NULL,
    FOREIGN KEY (location_type_id) REFERENCES location_types(id)
);

CREATE TABLE IF NOT EXISTS labwares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    barcode VARCHAR(255) NOT NULL,
    location_id INT NOT NULL,
    FOREIGN KEY (location_id) REFERENCES locations(id)
);

