Installation:
download the exe file from website and run it

- rustc is the rust compiler which creates executable for running rust program similar to node for  nodejs
- rustup is cli for updating the version of the rust similar to nvm for nodejs
- rustfmt is cli command to format the rust code similiar to prittier
- Rustaceans is the name for rust developer
- cargo is the package manager like npm
- Packages are know as crates in rust

## Cargo
Create project with cargo
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

## Variables & Mutability
Variables are let and const.
- let:  is immutable as default mean the variable cannot be re-assigned. To re-assign the mut keyword need to be add.
Ex: let mut age = 5;
- const: is also immutable 
  - It never be re-assigned.
  - Required to define type of the variable
  - It can be global variable
  - cannot assign let variable as value of const variable
    - Ex: let age = 5; const myfutureAge: u32 = 20 + age; it will throw an error.
  - use Capital letter convention  

Shadowing: is the way to re declare the variable.<br>
Ex: let mut age = 10;
age = age + 1;
let age = age;<br>
The advantage of using shadowing is:
  - After mutating the value, we can make it unmutatable.
  - The variable type can be change like string to number. 

