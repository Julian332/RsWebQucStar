# rs web quick start


# prerequisite
sudo apt install build-essential

# usage
cargo install diesel_cli

modify DATABASE_URL in .env

diesel setup

create table in db

diesel migration generate --diff-schema create_posts