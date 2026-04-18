-- Active: 1757026795294@@127.0.0.1@5432@orb
CREATE TABLE IF NOT EXISTS catalogs.role_permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7 (),
    role_id UUID NOT NULL REFERENCES catalogs.roles (id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES catalogs.permissions (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (role_id, permission_id)
);