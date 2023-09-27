`protoc-rs` is a simple standalone command line tool for generating a usable
Rust crate from Protobuf `.proto`  files. It is **not** a Rust implementation of
[protoc](https://github.com/protocolbuffers/protobuf) but rather a supplement to it when a set of
Protobuf definitions need to be compiled into multiple languages and Rust being one of them.

# Requirements

The Cargo manifest file, `Cargo.toml` must exist (it will not be generated). It must depend on
[tonic](https://docs.rs/tonic/latest/tonic/) and [prost](https://docs.rs/prost/latest/prost/), e.g.

```toml
[dependencies]
tonic = "0"
prost = "0"
```

If the repository contains protobuf generated files for multiple languages, it is recommended to
put the generated Rust files into a subdirectory called `rust/` instead of the usual source path
`src/`. That makes it clear that this directory contains the generated Rust code, while other
subdirectories contain other language code (e.g. `go/`, `cpp/`, ...). To make this work and make the
crate importable, the custom source code location must also be set in the `Cargo.toml`. For example:

```toml
[lib]
path = "rust/lib.rs"
```

The root library file is always the default name `lib.rs`.

# Usage by example

1. Navigate to the crates root directory (e.g. where `Cargo.toml` is located).
2. Compile the `.proto` files. The command works similar to `protoc`. If they are located under
`proto/`, the command would look like this:
   ```
   protoc-rs compile -I proto/ --out=rust/ proto/*.proto
   ```
   All compiled proto files must be passed to a single call of the command so the `lib.rs` file can
   be generated accordingly. Also, all source files must be put under one of the specified include
   paths.


Now, the `rust/` directory contains a valid Rust module structure and the crate can be imported by
other crates. You can validate the crate by running `cargo check`.
