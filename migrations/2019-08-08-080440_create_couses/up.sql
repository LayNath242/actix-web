CREATE TABLE courses(
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    thumbnail VARCHAR NULL,
    video_url VARCHAR NULL ,
    description VARCHAR(100) NULL,
    cate_id SERIAL NOT NULL REFERENCES categories(id) ON UPDATE CASCADE,
    price FLOAT NOT NULL,
    created_at DATE NOT NULL
);



CREATE TABLE user_courses (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users(id) ON UPDATE CASCADE,
    course_id SERIAL NOT NULL REFERENCES courses(id) ON UPDATE CASCADE
);