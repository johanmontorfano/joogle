PG_DB_URL=$(dotenv -p PG_DIESEL_URL)

diesel migration run --database-url="$PG_DB_URL"
