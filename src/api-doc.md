# rs web quick start


# Prerequisite
sudo apt install build-essential

sudo apt-get install libpq-dev

cargo install diesel_cli

cargo install diesel_ext

# Usage

modify DATABASE_URL in [.env](..%2F.env)

diesel setup

create table in [schema.rs](schema.rs)

diesel migration generate --diff-schema create_table

diesel_ext -m -r

add             
#[diesel(table_name = crate::schema::permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]        
to model , 

derive     WebApiGen ,

http://localhost:5090/docs
