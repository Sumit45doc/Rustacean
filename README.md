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

- for checking the code use command:
  ```shell
    cargo check
  ```
    Note: This will speed up development process by providing quicker way to check if updated code still compiles. This does not generates any executable.

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
Variables are created by using keyword "let".
- A variable is immutable by default mean the variable cannot be re-assigned. To re-assign the "mut" keyword shouild be used.
- Ex: 
  ```shell
    let mut age = 5;
  ```

- "const" keyword is used to create a Constant which is also immutable(Once assigned it's value can't be changed).
  - It never be re-assigned.
  - It is Required to define type of the constant(must be annotated).
  - It can have Global or Local scope. 
  - Cannot assign let variable as value of const variable
    
  - Ex: 
  ```shell
    let age = 5;
    const myfutureAge: u32 = 20 + age; //it will throw an error.
  ```
  - Use Capital letters to name a constant.  

Shadowing: is the way to re-declare the variable.<br>

- Ex: 
  ```shell
    let mut age = 10;
    age = age + 1;
    let age = age;
  ```

The advantage of using shadowing is:
  - After mutating the value, we can make it immutable.
  - The variable type can be changed like string to number.(Making use of only one variable instead seperate variable for string and number).
    
