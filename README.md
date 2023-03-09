# Nova

Small social network like project

## Prerequisites

Required to be installed:

- Rust
- Docker
- Docker-Compose

Recommended:

- A DB management tool for PostgreSQL, e.g. DBeaver

## Setup

### Database

Run `docker compose up` to deploy docker containers.\
The mandatory variables for docker and postgresql are located in the `docker-compose.yml` file.\
`SeaORM` is used in this project, to use all functionality of the ORM:\
Run `cargo install sea-orm-cli`

### TLS Certificate

`openssl` is required\

- Create the `/certs` directory in the root of the project: `mkdir certs`
- Change the directory: `cd ./certs`
- Generate the certificates: `openssl req -new -newkey rsa:4096 -x509 -sha256 -days 365 -nodes -out cert.pem -keyout key.pem`

### To run the server

Execute the following command: `cargo run`.

## License

The project is distributed under the Mozilla Public License 2.0 (MPL-2.0).
