FROM rust as build

RUN mkdir -p /code
COPY . /code

WORKDIR /code
RUN cargo install --path .

FROM debian

COPY --from=build /usr/local/cargo/bin/dnd-bot /bin

CMD dnd-bot
