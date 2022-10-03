-- Cretae user table
CREATE TABLE users(
   user_id uuid PRIMARY KEY,
   login_email TEXT NOT NULL UNIQUE,
   username TEXT NOT NULL,
   password TEXT NOT NULL
);
