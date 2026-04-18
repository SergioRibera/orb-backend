-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE IF NOT EXISTS catalogs.permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);