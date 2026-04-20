-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    name VARCHAR,
    phone VARCHAR,
    email VARCHAR
);