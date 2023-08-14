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

## Database

We will be using a few PostgreSQL databases.

### Setup

#### Development

For our development environment, we will have just a single database cluster set up in the project root under `.treeleaves`

This database cluster will contain all our development databases.
This cluster will manage both:
- The local test files data
- The shared database data

The PostgreSQL user 

We will create a new user `treeleaves-dev` to manage this cluster.

#### Production

For our production environment, we'll have two similar database clusters.
- 1 database cluster in our desired directory under `.treeleaves`.
    We'll call this cluster the Treeleaves cluster.
- 1 database cluster in `~/.local/share/data/treeleaves/data`.
    We'll call this the Shared Data cluster

These two database clusters will manage two separate stores of data.
- The Treeleaves cluster will manage local file data.
- The Shared Data cluster will manage the cached data that is shared with many Treeleaves database clusters

We'll have these two database clusters running under the current user

For our development environment we'll set up a database cluster in our directory.

The test database is run on a new

