
<div align="center">
  <img src="https://raw.githubusercontent.com/weldsorm/welds/main/page/src/assets/images/banner.png"/>
  <h3>An async ORM written in rust using sqlx and/or Tiberius.</h3>
</div>


# DEPRECATED: Welds - CLI

## The welds-cli tool has been deprecated in favor of the cli tool [Gumbo](https://crates.io/crates/gumbo). This cli is much better maintains and serves the same purpose.



Welds is an ORM for Rust. 

This crate is a command line tool to help you use welds.

It is used to generate rust code for your struct definitions.

You point it at your database and out comes a bunch of rust files for all the tables in your database.

# Install
```bash
cargo install welds-cli
```

# How to use

1) Set a connection string to your database using the ENV DATABASE_URL

```bash
export DATABASE_URL=postgres://postgres:password@localhost:5432
```

2) use the welds-cli to create a `welds.yaml` database definition file.
```bash
welds update
```

3) use the welds-cli to generate rust code. 
```bash
welds generate
```
