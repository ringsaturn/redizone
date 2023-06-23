# Redizone: Redis compatible server convert longitude and latitude to timezone name(s).

```console
127.0.0.1:6380> get_tz 116.3883 39.9289
Asia/Shanghai
127.0.0.1:6380> get_tzs 116.3883 39.9289
1) "Asia/Shanghai"
```
