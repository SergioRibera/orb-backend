-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE user_stores (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    UNIQUE (user_id, store_id)
);