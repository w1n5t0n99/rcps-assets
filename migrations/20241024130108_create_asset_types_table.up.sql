CREATE TABLE
    "asset_type" (
        id SERIAL PRIMARY KEY,
        brand TEXT NOT NULL UNIQUE,
        model TEXT NOT NULL,
        picture0 TEXT,
        picture1 TEXT,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW(),
        UNIQUE (brand, model)
    );
