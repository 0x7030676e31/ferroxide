mkdir -p "$HOME/.ferroxide"
rm -fr .sqlx "$HOME/.ferroxide/database.sqlite3"
sqlite3 "$HOME/.ferroxide/database.sqlite3" < schema.sql
DATABASE_URL=sqlite:"$HOME"/.ferroxide/database.sqlite3 cargo sqlx prepare