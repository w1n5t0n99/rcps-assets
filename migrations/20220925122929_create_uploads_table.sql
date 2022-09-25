-- Create uploads table
CREATE TABLE uploads(
    sid SERIAL,
    PRIMARY KEY(sid),
    uploaded_file TEXT NOT NULL,
    upload_date timestamptz NOT NULL,
    total INTEGER NOT NULL,
    Skipped INTEGER NOT NULL,
    added INTEGER NOT NULL
);