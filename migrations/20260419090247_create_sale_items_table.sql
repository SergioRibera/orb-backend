-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE sale_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    sale_id UUID NOT NULL REFERENCES sales (id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products (id),
    quantity DECIMAL(10, 2) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    subtotal DECIMAL(10, 2) NOT NULL
);