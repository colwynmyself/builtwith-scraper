# BuiltWith Scaper

Test work to pull data from the BuiltWith API. This is mostly so I can get used to doing some real work in Rust that isn't a TODO app.

## Running the project

1. `cargo install diesel_cli --no-default-features --features postgres`
    Note on Windows: I had to add the postgres `lib` and `bin` folder to my path and add `$HOME/.cargo/config` with the content
    ```
    [target.x86_64-pc-windows-msvc.pq]
    rustc-link-search = ["D:\\Program Files\\PostgreSQL\\10\\lib"]
    rustc-link-lib = ["libpq"]
    ```
2. `cp Config.toml{.dist,}` and edit with your data
3. `cp .env{.dist,}` and edit with your data
4. `diesel migration run`
5. `cargo run`

Access on `localhost:8000`