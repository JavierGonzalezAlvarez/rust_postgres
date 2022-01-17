# CRUD en rust en BD de postgresql
--------------------------------------
No es necesario instalar diesel

## crear la BD en postgres, acceso a postgres
$ sudo -u postgres psql
-create database apirust;
CREATE USER javier WITH PASSWORD '2525_ap';
mensaje: CREATE ROLE
GRANT ALL PRIVILEGES ON DATABASE apirust to javier;
mensaje: GRANT

## crear un proyecto en rust
$ cargo new gui --bin

## run rust
$ back/cargo run

