# Workflow

## Project Structure
The dependency graph looks something like that: \
controller > service > store 
1. A controller is ONLY for the receiving and sending of http packages \
2. The services provide the general logic (e.g. checking if the login credentials are correct) \
3. The store provides functions to access data storages like databases needed by the service

## Programming SQL queries

### Requirements

First make sure the database is RUNNING

### Querying

The macros you use are sqlx::query!(SQL_QUERY) and sqlx::query_as!(SQL_QUERY)

More under ['sqlx documentation'](https://docs.rs/sqlx/latest/sqlx/)

### Check Queries

cargo install diesel_cli --no-default-features --features "postgres"

To check if a query is valid you can run `cargo sqlx prepare` \
This validates your query (NEED THE DATABASE TO RUN). It is also run when building.

## Docker

to run the docker container locally 

you need to change the DATABASE_URL entry in your `.env` so that it uses your local ip \
to find you local ip use the `ipconfig` command

then run the following commands

`docker build -t rust-backend .`

`docker run -p 8000:8000 rust-backend`
