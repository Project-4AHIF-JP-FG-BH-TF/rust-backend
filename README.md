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

For communicating with the db please take a look at the [diesel docs](https://diesel.rs/guides/getting-started.html)

### Check Queries

cargo install diesel_cli --no-default-features --features "postgres"

https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md

## Docker

to run the docker container locally 

you need to change the DATABASE_URL entry in your `.env` so that it uses your local ip \
to find you local ip use the `ipconfig` command

then run the following commands

`docker build -t rust-backend .`

`docker run -p 8000:8000 rust-backend`
