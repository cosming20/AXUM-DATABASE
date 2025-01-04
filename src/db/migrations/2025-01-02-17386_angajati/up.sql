-- Your SQL goes here
CREATE TABLE angajati (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nume VARCHAR(255),
    prenume VARCHAR(255),
    telefon VARCHAR(255),
    banca_id INT REFERENCES banca(id)
);