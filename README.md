# o3 "o[1]ne-o[2]n-o[3]ne"
Record your daily activities for quick recall in one-on-ones with your manager.

### Setup
To create the Postgres database locally, set any ENV variables (described below) and run `make` or `make setup`.

This will create the database by running the "o3_database.sql" found in the root directory. It will substitute the POSTGRES_USER of the env variable O3_POSTGRES_USER if set.

To create the binary "o3", run `make bin`.  This will install the binary in `/usr/local/bin`.  It will require su permissions, so you will be prompted to enter your password.


### Commands
After creating the `o3` binary, verfify it's in your path by typing `which o3`. You should see `/usr/local/bin/o3`.

- `o3` - Default is `add` if no arguments are specified.
- `o3 a | add` - Add a summary to the database.
- `o3 ls | list [0-65535]` - List all summaries.
  - Optionally supply a number (0-65535) to list summaries between now and that many days ago.


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


### Manually create the executable
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
