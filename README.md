# Poinsettia

stupidly simple in memory key value store

## Get Started

server

```sh
docker build -t poinsettia .
docker run -p 6379:6379 poinsettia
```

client

via cli

```sh
cargo run --bin cli -- set id 1
cargo run --bin cli -- get id
```
via netcat

```sh
echo "set id 2" | nc 127.0.0.1 6379
echo "get id" | nc 127.0.0.1 6379
```

## License
MIT
