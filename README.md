# Redizone: Redis compatible server convert longitude and latitude to timezone name(s).

Build with [`tidwall/redcon.rs`](https://github.com/tidwall/redcon.rs) and
[`ringsaturn/tzf-rs`](https://github.com/ringsaturn/tzf-rs).

## Install

### Via `cargo``

```bash
cargo install redizone
```

## Usage

```console
$ redis-cli -p 6380
127.0.0.1:6380> get_tz 116.3883 39.9289
Asia/Shanghai
127.0.0.1:6380> get_tzs 116.3883 39.9289
1) "Asia/Shanghai"
```

See also:

- [`ringsaturn/tzf-sever`](https://github.com/ringsaturn/tzf-server): HTTP&Redis
  server written in Go
- [`racemap/rust-tz-service`](https://github.com/racemap/rust-tz-service):
  "Timezone Rest API"
