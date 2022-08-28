-- Create assets table
CREATE TABLE assets(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    asset_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    serial_num TEXT NOT NULL UNIQUE,
    model TEXT,
    brand TEXT,
    date_added timestamptz NOT NULL
);