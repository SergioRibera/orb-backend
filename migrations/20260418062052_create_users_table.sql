-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    name VARCHAR(100) NOT NULL,
    second_name VARCHAR(100),
    first_surname VARCHAR(100),
    second_surname VARCHAR(100),
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);