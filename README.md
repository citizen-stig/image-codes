# Image Codes

Web service for generating QR and Bar codes, written in Rust.
It uses [actix-web](https://actix.rs) framework.

## Usage

```
cargo run
```

Open browser: 

* `localhost:8088/encode/BarCode?payload=hello` for BarCode
* `localhost:8088/encode/QRCode?payload=hello` for QRCode


The server respects `Accept` HTTP header, so links can be used in `<img>` tag or in Ajax request, in which case resulting image will be base64 encoded.
