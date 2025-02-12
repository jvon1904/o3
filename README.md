# o3 "o[1]ne-o[2]n-o[3]ne"
Recording your daily activities for quick recall in one-on-ones with your manager.

### Options
- `ls` - List all summaries.

### Database
Create the database by running the "o3_database.sql" found in the root directory.  Make sure you change the <POSTGRES_USER> to the desired application user.

### Config
Add .env file in project directory with the following variables:

```env
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DATABASE=o3_database
POSTGRES_USER=o3_application_user
POSTGRES_PASSWORD=<PASSWORD>
```

Also, you can add an .o3_config file to home directory, and export all in POSTGRES variables there.  

```zsh
export POSTGRES_HOST=localhost
export POSTGRES_PORT=5432
export POSTGRES_DATABASE=o3_database
export POSTGRES_USER=o3_application_user
export POSTGRES_PASSWORD=<PASSWORD>
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
