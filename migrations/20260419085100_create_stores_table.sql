-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE stores (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    name VARCHAR NOT NULL,
    address TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);