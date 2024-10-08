CREATE TABLE
    "users" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid7()),
        email TEXT NOT NULL UNIQUE,
        email_verified BOOLEAN NOT NULL DEFAULT FALSE,
        password_hash TEXT NOT NULL,
        given_name TEXT NOT NULL,
        family_name TEXT NOT NULL,
        role_id INTEGER NOT NULL,
        picture TEXT NOT NULL,
        created_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMP
            WITH TIME ZONE NOT NULL DEFAULT NOW()
    );
