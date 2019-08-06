-- Your SQL goes here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    created_at timestamp default current_timestamp
);

insert into roles(title) values('Admin');
insert into roles(title) values('Teacher');
insert into roles(title) values('Student');