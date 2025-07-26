# rust_server

- This repository records typical usage of the Actix-web framework, as well as handy tips and best practices encountered during Rust development.


## Features:

-  Documenting the design and handling process for RESTful-style requests.
-  Documenting the setup and usage of database connections and ORM (SeaORM) workflows.
-  Documenting the structure of API response data, such as JSON,  as well as the overall project structure and code organization.
-  Recording the usage patterns for thread pools in Rust, including setup and task handling.
-  Recording how to use asynchronous processing in Rust.

## DataBase & ORM

- The database used here is MySQL (of course, you can switch to another RDBMS as needed).
- This project uses SeaORM for database operations 
  - see the SeaORM documentation for more details.
  - [SeaORM official Doc](https://www.sea-ql.org/SeaORM/docs/index/)
  - [SeaORM Tutorials](https://www.sea-ql.org/sea-orm-tutorial/)
- The MySQL service is set up using Docker
  - the relevant image configuration has been added to the `docker-compose.yml` file.

### Start DB

`
    docker-compose up -d
    docker-compose down 
`

### Create Table

- `DATABASE_URL="mysql://testuser:testpass@localhost:3306/testdb" sea-orm-cli migrate refresh `
    - WATCH: `refresh`

### Generate Entity from Database

`
sea-orm-cli generate entity \
    -u mysql://testuser:testpass@localhost:3306/testdb \
    -o ./entities/src
`

## TODO

- thread
- sharedObject / Ownership
- Stream