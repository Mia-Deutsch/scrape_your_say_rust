
FROM rust:1.84

WORKDIR /scrape_your_say_rust

COPY . .

#RUN cargo build --release

CMD ["cargo", "run"]