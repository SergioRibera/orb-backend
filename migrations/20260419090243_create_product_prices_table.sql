-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE product_prices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    store_id UUID NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    price DECIMAL(10, 2) NOT NULL,
    UNIQUE (product_id, store_id)
);