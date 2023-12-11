# axum_yt

### Axum project for YouTube

![Alternative text](screenshot.png "title")

#### Docker (Postgres)

    docker run -d -p 5432:5432 --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword postgres
    my-postgres is the instance name
    password = mysecretpassword
    user = postgres

to stop the container -> $docker stop my-postgres
