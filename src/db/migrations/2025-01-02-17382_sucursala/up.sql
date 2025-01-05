-- Your SQL goes here
CREATE TABLE sucursala (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nume VARCHAR(200),
    adresa VARCHAR(200),
    banca_id INT,
    FOREIGN KEY (banca_id) REFERENCES banca(id) ON DELETE CASCADE
);