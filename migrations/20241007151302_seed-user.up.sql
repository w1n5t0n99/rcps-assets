-- password_hash = password12345678
INSERT INTO users(email, email_verified, password_hash, given_name, family_name, role_id, picture)
SELECT 'relam@richmond-county.k12.va.us', true, '$argon2id$v=19$m=19456,t=2,p=1$ec5XJarxXF1OHLzQiTvgsA$t5DVvvS/6SyEr3WVerk7tGeGimrY+DUjumJrrWOkb3g', 'Reed', 'Elam', roles.id as role_id, '/static/images/User.svg'
FROM roles
WHERE name = 'admin'

