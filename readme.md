## DB - Diesel_CLI
**SET UP**
This will create a folder with *migrations* and a config file *diesel.toml* and also the *db_name* DB
- RUN:  cargo install diesel_cli --no-default-features --features postgres
- RUN:  set DATABASE_URL=postgres://username:password@localhost:5432/db_name?sslmode=disable
- RUN:  diesel setup

<br>

**SQL CODE**
TABLES
- RUN:  diesel migration generate ***table_name***

FILES [.sql]
- up:   create the actual table (as sql code)
- down: drop the actual table

Finally run migrations
- RUN:  diesel migration run

<br>

___
## Diesel - Rust
**SET UP**
First we need to set the "#[macro-use]" in *lib.rs* to be able to use *diesel* package in all the proyect

FILES
- cargo.toml:
```toml
diesel = { version = "1.4.2", features = ["postgres","uuidv07","r2d2"]}
```

- lib.rs:
```rust
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
```

<br>

**CONNECTION**
Then we need to connect to DB like in [main.rs]

<br>

**MODELS [tables]**
Finally we can create the tables in rust. Diesel_CLI allows to get *table schemas* from DB [the tables created during migration]

FILES:
- RUN:  diesel print-schema > src/schema.rs
This schemas are a format schema that rust manage using macros [table macro] to use it as a driver to connect to db from rust.
to use this schema, we need to create a new *struct* to connect it to this *schema*

Check [models.rs]