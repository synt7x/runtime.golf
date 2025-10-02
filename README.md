# runtime.golf

Runtime Golf takes a new approach to code golf, focused on runtime efficiency rather than code length. Compete to solve challenges with the fastest code possible, using a variety of programming languages.

## Running

First, obtain a copy of the project source.

```
git clone https://github.com/synt7x/runtime.golf
```

Running your own instance requires some dependencies, primarily [OpenResty](https://openresty.org), [Rust](https://rust-lang.org/), [NodeJS](https://nodejs.org/), and [SQLite3](https://sqlite.org/).

Once the dependencies are installed, you will be required to initialize the `data` directory as well as the `content/logs/error.log` file. You must create a `.env` file with the following values:

```
DATABASE_URL=sqlite:../data/runtime_golf.db
CLIENT_SECRET=...github oauth client secret...
CLIENT_ID=...github oauth client id...
JWT_SECRET=...randomly generated JWT key...
URL=...github oauth redirect uri...
```

Before building the server, you must first run `make db` once to initialize the database (this will not need to be run in subsequent restarts). To start the server, you need to run `make serve` to start the OpenResty reverse-proxy, and then `make run` to start the server.
> Note: You may need to run `cargo install sqlx-cli` and `cargo sqlx prepare` in order to properly compile the backend.