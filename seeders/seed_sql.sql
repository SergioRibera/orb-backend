-- seed_sql.sql
-- Handles tables with UUID FKs and DECIMAL columns that grow-rs cannot parameterize.
-- Safe to re-run (ON CONFLICT DO NOTHING / DO UPDATE).

-- ── Products ─────────────────────────────────────────────────────────────────
INSERT INTO products (name, barcode, description, cost) VALUES
    ('Coca-Cola 600ml',       '7501055361013', 'Refresco de cola 600ml',          10.50),
    ('Pepsi 600ml',           '7501032300478', 'Refresco de cola 600ml',          9.50),
    ('Agua Ciel 500ml',       '7501055357283', 'Agua purificada 500ml',           5.00),
    ('Sabritas Original 45g', '7501011102025', 'Papas fritas sabor original',     11.00),
    ('Doritos Nacho 50g',     '7501011102032', 'Totopos con queso nacho',         12.00),
    ('Leche Lala Entera 1L',  '7501055363789', 'Leche entera pasteurizada 1 lt',  22.00),
    ('Pan Bimbo Blanco',      '7501008020023', 'Pan de caja blanco grande',       35.00),
    ('Jabon Palmolive 150g',  '7506306228003', 'Jabon de tocador',                15.00),
    ('Shampoo HyS 400ml',     '7500435126090', 'Shampoo anticaspa',               68.00),
    ('Chocolate Carlos V',    '7501025401026', 'Chocolate con leche 29g',         9.00),
    ('Atun Dolores 140g',     '7501004901012', 'Atun en agua',                    18.50),
    ('Galletas Marias 200g',  '7501008400081', 'Galletas de vainilla',            16.00)
ON CONFLICT (barcode) DO NOTHING;

-- ── Devices ──────────────────────────────────────────────────────────────────
INSERT INTO devices (store_id, name)
SELECT s.id, d.name
FROM (VALUES
    ('Sucursal Centro',    'Caja 1'),
    ('Sucursal Centro',    'Caja 2'),
    ('Sucursal Norte',     'Caja 1'),
    ('Sucursal San Pedro', 'Caja 1')
) AS d(store_name, name)
JOIN stores s ON s.name = d.store_name
ON CONFLICT DO NOTHING;

-- ── Role ↔ Permissions (admin = all) ─────────────────────────────────────────
INSERT INTO catalogs.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM catalogs.roles r
CROSS JOIN catalogs.permissions p
WHERE r.name = 'admin'
ON CONFLICT DO NOTHING;

-- cajero
INSERT INTO catalogs.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM catalogs.roles r
JOIN catalogs.permissions p ON p.name IN (
    'products:read', 'sales:read', 'sales:write',
    'cash:read', 'cash:write', 'customers:read', 'customers:write'
)
WHERE r.name = 'cajero'
ON CONFLICT DO NOTHING;

-- supervisor
INSERT INTO catalogs.role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM catalogs.roles r
JOIN catalogs.permissions p ON p.name IN (
    'products:read', 'inventory:read', 'inventory:write',
    'sales:read', 'cash:read', 'customers:read'
)
WHERE r.name = 'supervisor'
ON CONFLICT DO NOTHING;

-- ── User ↔ Roles ──────────────────────────────────────────────────────────────
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u JOIN catalogs.roles r ON r.name = 'admin'
WHERE u.email = 'emilianom129@gmail.com'
ON CONFLICT DO NOTHING;

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u JOIN catalogs.roles r ON r.name = 'cajero'
WHERE u.email = 'ana.lopez@orb.com'
ON CONFLICT DO NOTHING;

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u JOIN catalogs.roles r ON r.name = 'supervisor'
WHERE u.email = 'carlos.ramirez@orb.com'
ON CONFLICT DO NOTHING;

-- ── User ↔ Stores ─────────────────────────────────────────────────────────────
INSERT INTO user_stores (user_id, store_id)
SELECT u.id, s.id FROM users u CROSS JOIN stores s
WHERE u.email = 'emilianom129@gmail.com'
ON CONFLICT DO NOTHING;

INSERT INTO user_stores (user_id, store_id)
SELECT u.id, s.id FROM users u JOIN stores s ON s.name = 'Sucursal Centro'
WHERE u.email = 'ana.lopez@orb.com'
ON CONFLICT DO NOTHING;

INSERT INTO user_stores (user_id, store_id)
SELECT u.id, s.id FROM users u JOIN stores s ON s.name = 'Sucursal Norte'
WHERE u.email = 'carlos.ramirez@orb.com'
ON CONFLICT DO NOTHING;

-- ── Product ↔ Categories ──────────────────────────────────────────────────────
INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Bebidas'
WHERE p.barcode IN ('7501055361013','7501032300478','7501055357283')
ON CONFLICT DO NOTHING;

INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Snacks'
WHERE p.barcode IN ('7501011102025','7501011102032')
ON CONFLICT DO NOTHING;

INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Lacteos'
WHERE p.barcode = '7501055363789'
ON CONFLICT DO NOTHING;

INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Panaderia'
WHERE p.barcode IN ('7501008020023','7501008400081')
ON CONFLICT DO NOTHING;

INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Higiene Personal'
WHERE p.barcode IN ('7506306228003','7500435126090')
ON CONFLICT DO NOTHING;

INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Dulces y Chocolates'
WHERE p.barcode = '7501025401026'
ON CONFLICT DO NOTHING;

INSERT INTO product_categories (product_id, category_id)
SELECT p.id, c.id FROM products p JOIN categories c ON c.name = 'Enlatados'
WHERE p.barcode = '7501004901012'
ON CONFLICT DO NOTHING;

-- ── Product Prices ────────────────────────────────────────────────────────────
INSERT INTO product_prices (product_id, store_id, price)
SELECT p.id, s.id, v.price
FROM (VALUES
    ('7501055361013', 'Sucursal Centro', 18.00),
    ('7501032300478', 'Sucursal Centro', 17.00),
    ('7501055357283', 'Sucursal Centro', 10.00),
    ('7501011102025', 'Sucursal Centro', 18.00),
    ('7501011102032', 'Sucursal Centro', 20.00),
    ('7501055363789', 'Sucursal Centro', 32.00),
    ('7501008020023', 'Sucursal Centro', 50.00),
    ('7506306228003', 'Sucursal Centro', 22.00),
    ('7500435126090', 'Sucursal Centro', 95.00),
    ('7501025401026', 'Sucursal Centro', 14.00),
    ('7501004901012', 'Sucursal Centro', 28.00),
    ('7501008400081', 'Sucursal Centro', 24.00),
    ('7501055361013', 'Sucursal Norte',  19.00),
    ('7501032300478', 'Sucursal Norte',  18.00),
    ('7501055357283', 'Sucursal Norte',  10.00),
    ('7501011102025', 'Sucursal Norte',  19.00),
    ('7501055363789', 'Sucursal Norte',  33.00),
    ('7501008020023', 'Sucursal Norte',  52.00)
) AS v(barcode, store_name, price)
JOIN products p ON p.barcode = v.barcode
JOIN stores   s ON s.name    = v.store_name
ON CONFLICT (product_id, store_id) DO UPDATE SET price = EXCLUDED.price;

-- ── Inventory Movements (stock inicial) ──────────────────────────────────────
INSERT INTO inventory_movements (product_id, store_id, type, quantity)
SELECT p.id, s.id, 'purchase', v.qty
FROM (VALUES
    ('7501055361013', 'Sucursal Centro', 120),
    ('7501032300478', 'Sucursal Centro',  96),
    ('7501055357283', 'Sucursal Centro', 200),
    ('7501011102025', 'Sucursal Centro', 150),
    ('7501011102032', 'Sucursal Centro', 100),
    ('7501055363789', 'Sucursal Centro',  60),
    ('7501008020023', 'Sucursal Centro',  40),
    ('7506306228003', 'Sucursal Centro',  80),
    ('7500435126090', 'Sucursal Centro',  30),
    ('7501025401026', 'Sucursal Centro', 200),
    ('7501004901012', 'Sucursal Centro',  50),
    ('7501008400081', 'Sucursal Centro',  45),
    ('7501055361013', 'Sucursal Norte',   80),
    ('7501032300478', 'Sucursal Norte',   60),
    ('7501055357283', 'Sucursal Norte',  100),
    ('7501011102025', 'Sucursal Norte',   80),
    ('7501055363789', 'Sucursal Norte',   40),
    ('7501008020023', 'Sucursal Norte',   25)
) AS v(barcode, store_name, qty)
JOIN products p ON p.barcode = v.barcode
JOIN stores   s ON s.name    = v.store_name;
