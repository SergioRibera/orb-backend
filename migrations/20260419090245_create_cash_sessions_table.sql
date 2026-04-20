-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE cash_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    store_id UUID NOT NULL REFERENCES stores (id),
    user_id UUID NOT NULL REFERENCES users (id),
    device_id UUID NOT NULL REFERENCES devices (id),
    opened_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    closed_at TIMESTAMP,
    opening_amount DECIMAL(10, 2) NOT NULL,
    closing_amount DECIMAL(10, 2),
    expected_amount DECIMAL(10, 2)
);