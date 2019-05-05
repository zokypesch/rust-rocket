# create new project
cargo new rg-price --bin && cd rg-price
# karena kita pake rocket library, rocket ada dependency ke nightly so kita pasang dl
rustup default nightly # pasang defailt "nightly" Rustup for managing Rust versions,  ... kalo mau balikin rustup default rustc 1.34.0
rustc --version

nahh itu mirip kayak pyenv atau rvm

rustup installs rustc, cargo, rustup and other standard tools to Cargo's bin directory. On Unix it is located at $HOME/.cargo/bin and on Windows at %USERPROFILE%\.cargo\bin. This is the same directory that cargo install will install Rust programs and Cargo plugins.

This directory will be in your $PATH environment variable, which means you can run them from the shell without further configuration. Open a new shell and type the following:

rustup show 
rustup default nightly-x86_64-apple-darwin
rustup default stable-x86_64-apple-darwin

https://github.com/rust-lang/rustup.rs
rustup update && cargo update # update / install library

# check cargo and lib updated
cargo --version && rustc --version

# add dependency in cargo.toml
[dependencies]
rocket = "0.3.6"
rocket_codegen = "0.3.6"

# edit file main.rs, check the file

# run the project
cargo run

# doc rocketnya
https://rocket.rs/guide/getting-started/

# install diesel orm rust
cargo install diesel_cli

# connect diesel to local database
export DATABASE_URL=mysql://user:pass@127.0.0.1/heroes .> add in .zshrc
diesel setup
    Creating database: heroes #result

# create migration
diesel migration generate heroes # generate migration files

# run the migration
diesel migration run

# please add new depedency
diesel = { version = "1.0.0", features = ["mysql"] }
diesel_codegen = { version = "*", features = ["mysql"] }
r2d2 = "*"
r2d2-diesel = "*"

# to generate schema within hero struct
diesel print-schema > src/schema.rs 
# basiclly di code diatas bakalan generate schema dia bakalan ngebaca file di src trs di bikinin schema nya...

