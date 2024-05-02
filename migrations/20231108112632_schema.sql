-- Need to load JSON extension for storing an array of strings

CREATE TABLE IF NOT EXISTS schemas
(
    id         VARCHAR  PRIMARY KEY,
    content    JSON,
    created_ts   integer(4) default (cast(strftime('%s','now') as int))
);    