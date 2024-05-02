-- Need to load JSON extension for storing an array of strings

CREATE TABLE IF NOT EXISTS entities
(
    id         TEXT PRIMARY KEY NOT NULL,
    eid         TEXT,
    etype       TEXT,
    content       JSON,
    search_tags  TEXT,
    ttl           integer(4),
    created_ts    timestamp with time zone,
    updated_ts    timestamp with time zone
);