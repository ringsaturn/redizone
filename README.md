# Redizone: Redis compatible server convert longitude and latitude to timezone name(s).

Build with [redcon.rs] and [tzf-rs].

[redcon.rs]: https://github.com/tidwall/redcon.rs
[tzf-rs]: https://github.com/ringsaturn/tzf-rs

```console
127.0.0.1:6380> get_tz 116.3883 39.9289
Asia/Shanghai
127.0.0.1:6380> get_tzs 116.3883 39.9289
1) "Asia/Shanghai"
```
