# Mobile Vault Database Entities and Migration

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)

## Description

Mobile Vault Shared Database service

## Getting Started

### Dependencies

[Rust](https://rust-lang.org)

### Installing

```sh
cargo add --git https://github.com/opeolluwa/vault_database --no-default-features
```

### Executing program

```sh
cargo add sea-orm
```

#### Using the migration

In your `main.rs`

```rust
use migration::{Migrator, MigratorTrait};

fn main(){
    Migrator::up(&connection, None).await?;
}
```

code blocks for commands

## Documentation

Describe any special instructions that are necessary to install a software
package on your computer (if applicable).

## Help

Any advise for common problems or issues.
