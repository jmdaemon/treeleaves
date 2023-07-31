# Setup

Our desired database structure is the following:

- `.treeleaves`: Stores the needed database files
- `src/schema/`: Stores the generated database schema files
- `migrations/sqlite3/`: Stores the database migrations.

`diesel-cli` does not easily allow you to deal with multiple database configurations natively.

We achieve the desired structure by:

1. Setting `--database-url` directly.
    1. We setup our database-urls in `.envs/`
    2. We pass the `--database-url` in our setup script:
        - `diesel --database-url "$db" setup`
2. Setting:
    - Schema from the given `diesel-configs/cfg` file
    - Migration directory directly in `diesel-cli`

<!--Our command to generate the table is then:-->
Our command to generate the schema is then:

```bash
    diesel  --database-url "$db" \
    --config-file "$cfg" \
    --migration-dir "$dir" \
    migration generate
```
