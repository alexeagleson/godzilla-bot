https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#enable-building-in-offline-mode-with-query

 cargo install sqlx-cli

sqlx database create


if you need new migration:

sqlx migrate add {name}

sqlx migrate run