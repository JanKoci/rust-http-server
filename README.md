Simple Http server that logs all requests to stdout.

Build the project with ``cargo build``

```
Usage: ./http_server_rust [OPTIONS]

Options:
-p, --port <PORT>     Port to listen on [default: 7878]
-a, --address <HOST>  Host address to listen on [default: 127.0.0.1]
-H, --headers         Prints http headers
-h, --help            Print help
```

Example output:
```
$ ./http_server_rust -H
-> 127.0.0.1:52914 "GET /api/users?id=johny HTTP/1.1"
Headers:
  user-agent: PostmanRuntime/7.36.1
  host: 127.0.0.1:7878
  accept-encoding: gzip, deflate, br
  accept: */*
  connection: keep-alive
```

```
$ ./http_server_rust
INFO - 127.0.0.1 "GET /api/users?id=johny HTTP/1.1" 404 0 "-" "PostmanRuntime/7.36.1" 0.000092
INFO - 127.0.0.1 "POST /api/users/new HTTP/1.1" 404 0 "-" "PostmanRuntime/7.36.1" 0.000064
```