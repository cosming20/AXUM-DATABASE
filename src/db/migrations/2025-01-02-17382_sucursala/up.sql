-- Your SQL goes here
CREATE TABLE sucursala (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nume VARCHAR(200) NOT NULL,
    adresa VARCHAR(200) NOT NULL,
    banca_id INT NOT NULL,
    FOREIGN KEY (banca_id) REFERENCES banca(id) ON DELETE CASCADE
);