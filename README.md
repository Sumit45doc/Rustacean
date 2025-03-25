installation:
download the exe file from website and run it

- rustc is the command for running rust program similar to node for  nodejs
- rustup is cli for updating the version of the rust similar to nvm for nodejs
- rustfmt is cli command to format the rust code similiar to prittier
- Rustaceans is the name for rust developer
- cargo is the package manager like npm
- Packages are know as crates in rust

****Create project with cargo ****
```shell
cargo new <Project_name>
```

- packages is known as crates
- run program in cargo by command:
  ```shell
  cargo run
  ```
  Note: it will build and run

- for release(production) the code use command:
  ```shell
    cargo build --release
  ```

- To install any crate write the crate name with version in Cargo.toml file.  <br>Example:
  rand = "0.9.0"
**Alternative using command**
```shell
  cargo add <package_name>
```

- To remove crates use command:
```shell
  cargo remove <package-name>
```

- To update the crates use command:
  ```shell
    cargo update
  ```

- To open documentation of all the crates and project use command:
  ```shell
    cargo doc --open
  ```