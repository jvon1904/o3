CUR_DIR := $(PWD)
setup:
ifdef POSTGRES_USER
	cat o3_database.sql | sed 's/__POSTGRES_USER__/$(POSTGRES_USER)/g'
else
	cat o3_database.sql | sed 's/__POSTGRES_USER__/postgres/g' | psql
endif

release:
	cargo build --release

bin: release
	sudo rm /usr/local/bin/o3
	sudo ln -s $(CUR_DIR)/target/release/o3 /usr/local/bin/o3
