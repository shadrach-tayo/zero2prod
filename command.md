Saves metadata for all invocations of `query!` and related macros to
`sqlx-data.json` in the current directory, overwriting if needed.
```bash
cargo sqlx prepare
```

Run a single test function pass logs in to bunyan formatter

```bash
cargo t [test_name] | bunyan
```

Run sqlx migration

```bash
sqlx migrate run
```

Create new database migration
```bash
sqlx migrate add [migration name]
```
