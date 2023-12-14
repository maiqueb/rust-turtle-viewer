# rust-turtle-viewer
A sample rust web application w/ a postgreSQL backend

## Setup DB
You can build the helper container using (from the repo's root):
```bash
buildah build -t quay.io/mduarted/turtle-db:latest -f images/postgres.Dockerfile
```

You can start the DB with:
```bash
podman run --name turtle-db -d -p 5432:5432 quay.io/mduarted/turtle-db
```
