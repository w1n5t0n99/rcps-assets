CREATE TABLE
    "attachments" (
        id SERIAL PRIMARY KEY,
        hash TEXT NOT NULL,
        filename TEXT NOT NULL,
        content_type TEXT NOT NULL,
        first_seen_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW()
    );
