# Whats Up

Application to see if your friends are available to hang out.

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/install/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install)

### Installing

1. Clone the repo
2. Run `docker-compose up -d` to start the database and keycloak
3. Add keycloak realm "whats_up"
4. Add keycloak client "app"
5. Add keycloak scope "openid"
6. Add to keycloak client "app" the scope "openid"
6. Create user(s) in keycloak in realm "whats_up"

## ToDo

* [ ] Add tests
* [/] Add CI/CD
* [ ] Add frontend
* [ ] Add App for iOS and Android
