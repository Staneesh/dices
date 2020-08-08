FROM rust

WORKDIR /app

COPY .bin/game_server .

#RUN cargo install --bin game_server --path game_server

EXPOSE 2137
CMD [ "./game_server" ]
