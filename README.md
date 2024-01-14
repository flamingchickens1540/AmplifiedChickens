# Amplified Chickens

## Db setup
1. Install postgres
``brew install postgresql``

2. Start postgres
``brew services start postgresql``

3. Start db(make sure to update .env accordingly, default is postgres)
``psql <db-name>``

4. Install `sqlx-cli` using `cargo install sqlx-cli`, then run following:

## backend

```
cd backend/
sqlx database setup
cargo run
```

## frontend

```
cd frontend/
bun install
bun run dev
```
