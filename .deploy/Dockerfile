FROM rust:latest as build
RUN apt-get update

COPY . .

RUN cargo build --release

FROM rust:latest

WORKDIR /project

COPY --from=build /target/release/cat_telegram project
COPY --from=build /cats /project/cats
COPY --from=build .env /project/.env

ENTRYPOINT ["./project"]