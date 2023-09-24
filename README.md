# Actix Web - Login

Looking at login with Actix web.

## Setup

To use this repository you will need:
- Rust
- Docker
- Node 19

### Locally

To run locally:

1. Build the static assets

```bash
npm install && npm run build
```

2. Run cargo

```bash
cargo run
```

Visit http://127.0.0.1:8080 to view the app.

To run the tests:

```bash
cargo test
```

### Docker

To run via a Docker container:

1. Build the static files

```bash
npm install && npm run build
```

2. To build the container:

```bash
docker build . -t actix_login
```

3. To run the container:

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

To use the GitHub action you will need to set a repository secret with the name
`FLY_API_TOKEN` containing the value generated from the following command:

```bash
fly tokens create deploy -x 1000h
```
