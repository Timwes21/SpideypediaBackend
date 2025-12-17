FROM rust:1.89

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["rustos"]
