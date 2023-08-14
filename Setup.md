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

| Setup                               | Value                                          |
|-------------------------------------|------------------------------------------------|
| Database Cluster Directory (Target) | `/var/lib/postgres/data/dev/treeleaves/target` |
| Database Cluster Directory (Shared) | `/var/lib/postgres/data/dev/treeleaves/shared` |
| PostgreSQL User Account             | `postgres`                                     |
| Database User                       | `treeleaves-dev`                               |

#### Production

| Setup                               | Value                            |
|-------------------------------------|----------------------------------|
| Database Cluster Directory (Target) | `[target]/.treeleaves/data`      |
| Database Cluster Directory (Shared) | `~/.local/share/treeleaves/data` |
| PostgreSQL User Account             | `postgres`                       |
| Database User                       | `[user]`                         |

The two clusters manage manage two separate types of data.
- `Target` manages the local target files data.
- `Shared` manages the global files data for all Treeleaves databases for the user.

Pros:

- Multiple database clusters can be setup and managed for different target directories
- These clusters can be enabled/disabled individually
- The clusters can make use of the user's global data config

Cons:

- There is redundant data for the shared/global database data when running with multiple users
