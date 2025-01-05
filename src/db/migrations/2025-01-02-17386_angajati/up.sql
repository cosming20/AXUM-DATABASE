-- Your SQL goes here
CREATE TABLE angajati (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nume VARCHAR(255) NOT NULL,
    prenume VARCHAR(255) NOT NULL,
    telefon VARCHAR(255) NOT NULL,
    banca_id INT NOT NULL,
    FOREIGN KEY (banca_id) REFERENCES banca(id) ON DELETE CASCADE
);