-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE inventory_movements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    product_id UUID NOT NULL REFERENCES products (id),
    store_id UUID NOT NULL REFERENCES stores (id),
    type VARCHAR NOT NULL,
    quantity DECIMAL(10, 2) NOT NULL,
    reference_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);