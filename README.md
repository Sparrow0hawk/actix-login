# Actix Web - Login

Looking at login with Actix web.

## Setup

To use this repository you will need:
- Rust
- Docker

To run locally:

```bash
cargo run
```

Visit http://127.0.0.1:8080 to view the app.

To run the tests:

```bash
cargo test
```

To build the container:

```bash
docker build . -t actix_login
```

To run the container:

```bash
docker run --rm -p 8080:8080 actix_login:latest
```

Visit http://127.0.0.1:8080 to view the app.

## Deployment

To deploy you will need:
- flyctl
- A fly.io account

```bash
flyctl auth login

flyctl launch
```
