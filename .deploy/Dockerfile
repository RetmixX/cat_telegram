FROM rust:latest
WORKDIR /project

COPY cat_telegram project
COPY cats /project/cats
COPY .env /project/.env

ENTRYPOINT ["./project"]