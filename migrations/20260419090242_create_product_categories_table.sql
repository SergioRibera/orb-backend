-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE product_categories (
    product_id UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES categories (id) ON DELETE CASCADE,
    PRIMARY KEY (product_id, category_id)
);