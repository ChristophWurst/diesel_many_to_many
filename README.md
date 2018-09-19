# Diesel many to many playground

Example code to demonstrate how to query many-to-many associated table rows with Rust

```bash
docker run --name diesel_manytomany -e POSTGRES_USER=username -e POSTGRES_PASSWORD=password -e POSTGRES_DB=diesel_manytomany -p 5432:5432 postgres

#in a different shell
diesel migration run
cargo run
```

To see the queries run, use e.g. `timms/postgres-logging` instead of `postgres` as the Docker image.
