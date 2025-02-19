# o3 "o[1]ne-o[2]n-o[3]ne"
Record your daily activities for quick recall in one-on-ones with your manager.

### Running Locally
To create the Postgres database locally, set any ENV variables (described below) and run `make` or `make setup`.

To create the binary "o3", run `make bin`.

### Options
- Default is `add` if no arguments are specified.
- `a` | `add` - Add a summary to the database.
- `ls` | `list` - List all summaries.
  - Optionally supply a number (0-65535) to list summaries between now and that many days ago.

### Database
Create the database by running the "o3_database.sql" found in the root directory.  Make sure you change the <O3_POSTGRES_USER> to the desired application user.

### Config
Add .env file in project directory with the following variables:

```env
O3_POSTGRES_HOST=localhost
O3_POSTGRES_PORT=5432
O3_POSTGRES_DATABASE=o3_database
O3_POSTGRES_USER=o3_application_user
O3_POSTGRES_PASSWORD=<PASSWORD>
```

Also, you can add an .o3_config file to home directory, and export all in POSTGRES variables there.  

```zsh
export O3_POSTGRES_HOST=localhost
export O3_POSTGRES_PORT=5432
export O3_POSTGRES_DATABASE=o3_database
export O3_POSTGRES_USER=o3_application_user
export O3_POSTGRES_PASSWORD=<PASSWORD>
```

Then source the .o3_config file in your .zshrc.

```zsh
source ~/.o3_config
```


### Executable
Add symlink to PATH.

First, create a release binary.

```zsh
cargo build --release
```

Then symlink the binary to a directory in your PATH.

```zsh
sudo ln -s /path/to/o3/target/release/o3 /usr/local/bin
```

Now, you should be able to run the `o3` command anywhere on your system.
