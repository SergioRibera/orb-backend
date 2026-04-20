# Stores Module

Manages physical business locations (sucursales) and their devices.

## Tables

| Table         | Description                              |
|---------------|------------------------------------------|
| `stores`      | Physical store locations                 |
| `devices`     | POS terminals / registers inside a store |
| `user_stores` | Users authorized to work at a store      |

## Endpoints

Base path: `/api/v1`

### Stores

| Method | Path      | Description  | Status |
|--------|-----------|--------------|--------|
| POST   | `/stores` | Create store | 201    |
| GET    | `/stores` | List stores  | 200    |

```json
// POST /stores — Request
{
  "name": "Sucursal Centro",
  "address": "Av. Juárez 123"   // optional
}

// POST /stores — Response 201
{ "id": "<uuidv7>", "name": "Sucursal Centro", "address": "Av. Juárez 123" }
```

### Devices

| Method | Path                          | Description           | Status |
|--------|-------------------------------|-----------------------|--------|
| POST   | `/stores/{store_id}/devices`  | Create device         | 201    |
| GET    | `/stores/{store_id}/devices`  | List devices in store | 200    |

```json
// POST /stores/{store_id}/devices — Request
{ "name": "Caja 1" }   // name is optional

// POST /stores/{store_id}/devices — Response 201
{ "id": "<uuidv7>", "store_id": "<uuidv7>", "name": "Caja 1" }
```
