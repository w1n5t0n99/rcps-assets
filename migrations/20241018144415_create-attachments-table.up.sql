CREATE TABLE
    "image_attachments" (
        id SERIAL PRIMARY KEY,
        hash TEXT NOT NULL UNIQUE,
        filename TEXT NOT NULL,
        url TEXT NOT NULL,
        content_type TEXT NOT NULL,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW()
    );

CREATE TABLE
    "document_attachments" (
        id SERIAL PRIMARY KEY,
        hash TEXT NOT NULL UNIQUE,
        filename TEXT NOT NULL,
        url TEXT NOT NULL,
        content_type TEXT NOT NULL,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW()
    );

