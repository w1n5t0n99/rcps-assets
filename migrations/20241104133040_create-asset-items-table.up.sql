CREATE TABLE
    "asset_items" (
        id SERIAL PRIMARY KEY,
        asset_id TEXT UNIQUE,
        serial_number TEXT UNIQUE,
        name TEXT,
        brand TEXT,
        model TEXT,
        school TEXT,
        room TEXT,
        funding_source TEXT,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW(),
        FOREIGN KEY (brand, model) REFERENCES asset_types (brand, model)
    );

