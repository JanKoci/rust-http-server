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