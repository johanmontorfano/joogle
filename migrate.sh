PG_DB_URL=$(dotenv -p PG_DIESEL_URL)

diesel setup --database-url="$PG_DB_URL"
