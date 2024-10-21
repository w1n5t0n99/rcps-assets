CREATE TABLE
    "attachments" (
        id SERIAL PRIMARY KEY,
        hash TEXT NOT NULL UNIQUE,
        filename TEXT NOT NULL UNIQUE,
        content_type TEXT NOT NULL,
        first_seen_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW()
    );
