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

## Start the turtle-viewer service
You should start the turtle viewer service by running the container available
in github's registry. Execute the following command (has the
[Setup DB](#setup-db) step as a pre-requirement):

```bash
podman run -d \
    -e DB_USER=splinter \
    -e DB_PASSWORD=cheese \
    -e DB_NAME=turtles \
    -e DB_IP=localhost \
    -e HOST=localhost \
    -e PORT=9000 \
    --network=host  \
    ghcr.io/maiqueb/rust-turtle-viewer:main
```

After which you should see two running containers:
```bash
podman ps
CONTAINER ID  IMAGE                                    COMMAND     CREATED        STATUS        PORTS                   NAMES
40b08353aac5  quay.io/mduarted/turtle-db:latest        postgres    2 hours ago    Up 2 hours    0.0.0.0:5432->5432/tcp  turtle-db
3d7911dd173d  ghcr.io/maiqueb/rust-turtle-viewer:main              2 seconds ago  Up 2 seconds                          condescending_edison
```

You can now perform CRUD requests to the container:
```bash
# get all turtles
curl localhost:9000/turtles -H 'Content-Type: application/json' | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   481  100   481    0     0   208k      0 --:--:-- --:--:-- --:--:--  234k
{
  "Ok": [
    {
      "user_id": 1,
      "name": "leonardo",
      "email": "leo@tmnt.org",
      "weapon": "swords",
      "created_on": "2023-12-14T14:15:55.417722"
    },
    {
      "user_id": 2,
      "name": "donatello",
      "email": "don@tmnt.org",
      "weapon": "a stick",
      "created_on": "2023-12-14T14:15:55.417722"
    },
    {
      "user_id": 3,
      "name": "michaelangello",
      "email": "mike@tmnt.org",
      "weapon": "nunchuks",
      "created_on": "2023-12-14T14:15:55.417722"
    },
    {
      "user_id": 4,
      "name": "raphael",
      "email": "raph@tmnt.org",
      "weapon": "twin sai",
      "created_on": "2023-12-14T14:15:55.417722"
    }
  ]
}

# get a particular turtle
curl localhost:9000/turtles/2 -H 'Content-Type: application/json' | jq
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   116  100   116    0     0  83273      0 --:--:-- --:--:-- --:--:--  113k
{
  "user_id": 2,
  "name": "donatello",
  "email": "don@tmnt.org",
  "weapon": "a stick",
  "created_on": "2023-12-14T14:15:55.417722"
}

# create a turtle (???)
curl -XPOST localhost:9000/turtles \
    -H 'Content-Type: application/json' \
    --data '{"name": "casey jones", "email": "asd@asd.org", "weapon": "hockey stick"}'
{"user_id":16,"name":"casey jones","email":"asd@asd.org","weapon":"hockey stick","created_on":"2023-12-14T16:21:38.367918"}

# update a turtle
curl -XPUT localhost:9000/turtles/4 \
    -H 'Content-Type: application/json' \
    --data '{"name": "UPDATE", "email": "UPDATE@whoop.com", "weapon": "UPDATE"}'
{"user_id":4,"name":"UPDATE","email":"UPDATE@whoop.com","weapon":"UPDATE","created_on":"2023-12-14T14:15:55.417722"}

# delete a turtle
curl -XDELETE localhost:9000/turtles/4 -H 'Content-Type: application/json'
# this just return 204: No Content
# ... but the turtle w/ ID=4 is gooone
```