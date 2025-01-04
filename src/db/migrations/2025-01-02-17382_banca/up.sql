-- Your SQL goes here
CREATE TABLE banca (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nume VARCHAR(255),
    adresa VARCHAR(255),
    sucursala_id INT REFERENCES sucursala(id)
);