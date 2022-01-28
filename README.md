Learning Rust. This is a clone of the grep command.

Followed along with https://rust-cli.github.io/book/tutorial/index.html with some adjustments:

- using a BufReader instead of read_to_string for some minor optimization
- additional tests

Run from source

```shell
cargo run -- some-pattern some-file
```

Install via Cargo and run

```shell
cargo install shea_grrs
shea_grrs -- some_pattern ./some_file
```
