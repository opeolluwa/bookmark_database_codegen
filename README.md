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

1. Add the package with cargo

```sh
cargo add --git https://github.com/opeolluwa/bookmark_database_codegen --tag <TAG>  bookmark_database_codegen
```

```toml
sea-orm = { version = "1.0.0-rc.5", features = [
    "sqlx-postgres",
    "runtime-tokio-rustls",
    "macros",
] }
```

### Executing program

#### Using the migration

In your `main.rs`

```rust
use migration::{Migrator, MigratorTrait};

fn main(){
    Migrator::up(&connection, None).await?;
}
```

#### Using entities

```rust
use vault_database::entities::prelude::*;
use vault_database::entities::user_information::{self};
use vault_database::entities::vault;
```

code blocks for commands

## Documentation

Describe any special instructions that are necessary to install a software
package on your computer (if applicable).

## Help

Any advise for common problems or issues.
