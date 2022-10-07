# Cargo tree table

Convert the output of a hardcoded `cargo tree` command into a markdown table.

## Usage

```
cargo run -- /path/to/project/Cargo.toml > outputs/project.md
```

For zebra specifics run:

```
cargo run -- /path/to/project/Cargo.toml true > outputs/project.md
```

Examples:

```

$ cargo run -- /home/oxarbitrage/zebra/issue5214/zebra/Cargo.toml true > outputs/zebra.md

$ cargo run -- Cargo.toml > outputs/cargo-tree-table.md

$ cargo run -- /home/oxarbitrage/zebra/frost/frost/Cargo.toml > outputs/frost.md

```


## Outputs

| Project | Bumped
|---------|-------
| [Zebra](outputs/zebra.md) | 2022-10-07 
| [Cargo Tree Table](outputs/cargo-tree-table.md) | 2022-10-07 
| [FROST](outputs/frost.md) | 2022-10-07 
