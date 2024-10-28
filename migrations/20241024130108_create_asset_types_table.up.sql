CREATE TABLE
    "asset_types" (
        id SERIAL PRIMARY KEY,
        brand TEXT NOT NULL,
        model TEXT NOT NULL,
        description TEXT,
        cost TEXT,
        picture TEXT,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW(),
        UNIQUE (brand, model)
    );

CREATE INDEX idx_model ON asset_types (model);
