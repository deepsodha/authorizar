-- Need to load JSON extension for storing an array of strings
-- https://antonz.org/json-virtual-columns/ for search_tags
-- https://sqlite.org/forum/forumpost/dfd4739c57
-- open SQLite in WAL mode `PRAGMA journal_mode=WAL;`

CREATE TABLE IF NOT EXISTS policies
(
    id          TEXT PRIMARY KEY NOT NULL,
    content     TEXT                NOT NULL,
    search_tags        JSON,
    ttl         integer(4)             NOT NULL,    
    created_ts      timestamp with time zone,
    updated_ts   timestamp with time zone
);