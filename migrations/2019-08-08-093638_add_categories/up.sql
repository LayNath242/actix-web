CREATE TABLE categories(
    id SERIAL PRIMARY KEY,
    title VARCHAR(20) UNIQUE NOT NULL,
    user_id SERIAL NOT NULL REFERENCES users(id) ON UPDATE CASCADE,
    created_at timestamp default current_timestamp
);