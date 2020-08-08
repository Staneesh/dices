#!/bin/sh

cd $(dirname "$0")
cargo build -p game_server

mkdir -p .bin
cp target/debug/game_server .bin
docker-compose build
rm -rf .bin

docker-compose up
