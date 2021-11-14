# process_reader
Collects basic properties about running processes and writes them into a database


```shell
cargo install sqlx-cli
docker volume create pgdata 
docker run -it --rm -e PGDATA=/var/lib/postgresql/data/pgdata -v pgdata:/var/lib/postgresql/data -e POSTGRES_PASSWORD=dev -e POSTGRES_USER=dev -p 5555:5432 postgres
export DATABASE_URL="postgres://dev:dev@localhost:5555/dev"
sqlx db create
sqlx migrate run
```
