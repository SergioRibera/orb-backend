-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    name VARCHAR NOT NULL,
    barcode VARCHAR UNIQUE,
    description TEXT,
    cost DECIMAL(10, 2),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);