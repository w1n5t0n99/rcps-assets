CREATE TABLE
    "asset_types" (
        id SERIAL PRIMARY KEY,
        brand TEXT NOT NULL UNIQUE,
        model TEXT NOT NULL,
        description TEXT,
        cost TEXT,
        picture0 TEXT,
        picture1 TEXT,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW(),
        UNIQUE (brand, model)
    );

CREATE INDEX idx_model ON asset_types (model);
