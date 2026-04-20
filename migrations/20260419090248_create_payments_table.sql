-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    sale_id UUID NOT NULL REFERENCES sales (id) ON DELETE CASCADE,
    method VARCHAR NOT NULL,
    amount DECIMAL(10, 2) NOT NULL
);