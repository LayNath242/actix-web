CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    description VARCHAR NOT NULL,
    user_id SERIAL NOT NULL REFERENCES users(id) ON UPDATE CASCADE,
    created_at timestamp default current_timestamp
)