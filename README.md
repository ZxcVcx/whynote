# Whynote

A Lightweight Personal Blogging System Implemented in Rust

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributor](#contributor)
- [License](#license)

## Installation

Instructions on how to install and set up your project.

### Standalone way

#### backend

Download the release from the [release page](https://github.com/ZxcVcx/whynote/releases);

Write a .env file in the same directory as the executable file:

```shell
MONGODB_URI="mongodb://localhost:27017/" # or any other mongodb uri
MONGODB_NAME="whynote" # or any name you like

ADDR=127.0.0.1 # should use 0.0.0.0 in docker
PORT=8009

# now these two config are useless, but I will keep them for a while...
GQL_VER=v1
GIQL_VER=v1i

GQL_PATH="/graphql"
GIQL_PATH="/graphiql"

# this is the secret key for the jwt token, you can generate a new one by running the following command
# openssl rand -base64 32
JWT_SECRET=0F4EHz+1/hqVvZjuB8EcooQs1K6Q

CLAIM_EXP=10000000000
```

and then run the executable file.

#### frontend

If you want to use the frontend, you can download `whynote-frontend-yew.zip`
from the [release page](https://github.com/ZxcVcx/whynote/releases), and extract it, put the `dist` folder for any web server like nginx or apache.

If you want take any customization, like custom  you can fork the [repo](https://github.com/ZxcVcx/whynote), and edit the `/frontend-yew/build_prop.toml` file, GitHub Actions will build the frontend and upload the zip file to the Artifacts.

Atherwise, you can clone the [repo](https://github.com/ZxcVcx/whynote), and build the frontend by yourself.

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli wasm-opt


git clone https://github.com/ZxcVcx/whynote.git
cd whynote/frontend-yew
trunk build --release
```

### Docker way

```shell
git clone https://github.com/ZxcVcx/whynote.git
cd whynote
docker-compose up
```

### Development way

At first, you should install the `mongodb` and `rust` environment, and clone the repo:

```shell
git clone https://github.com/ZxcVcx/whynote.git
```

open two terminal, one for the backend, the other for the frontend.

backend: 

```shell
cd whynote/backend-axum
cp .env.example .env
# edit the .env file
cargo run
```

frontend:

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli wasm-opt

cd whynote/frontend-yew
# edit the build_prop.toml file
trunk serve
```

## Usage

### MongoDB

The most important thing is, the mongodb uri in `backend-axum/.env` file should be correct. You don't need to create the database, the backend will create the database and the collections automatically.

### Frontend

Remember to change the `comment` in the `frontend-yew/build_prop.toml` file to your own address.

## Contributor

- [Nathan Wang](https://github.com/ZxcVcx/) - a CS student from DQU creator and maintainer, feel free to contact me if you have any questions.

## License

MIT License