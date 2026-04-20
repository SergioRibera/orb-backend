# IAM Module (Identity & Access Management)

Owns users, roles, permissions, and their relationships.

## Tables

| Table                      | Schema   | Description                        |
|----------------------------|----------|------------------------------------|
| `users`                    | public   | User accounts                      |
| `catalogs.roles`           | catalogs | Named roles                        |
| `catalogs.permissions`     | catalogs | Named permissions                  |
| `catalogs.role_permissions`| catalogs | Permissions assigned to roles      |
| `user_roles`               | public   | Roles assigned to users            |

## Endpoints

Base path: `/api/v1`

### Users

| Method | Path         | Description     | Status |
|--------|--------------|-----------------|--------|
| POST   | `/users`     | Register user   | 201    |
| GET    | `/users`     | List users      | 200    |

```json
// POST /users — Request
{
  "name": "Juan",
  "second_name": "Carlos",       // optional
  "first_surname": "García",     // optional
  "second_surname": "López",     // optional
  "email": "juan@example.com",
  "password": "secret"
}

// POST /users — Response 201
{ "id": "<uuidv7>", "email": "juan@example.com" }
```

### Roles

| Method | Path                              | Description             | Status |
|--------|-----------------------------------|-------------------------|--------|
| POST   | `/roles`                          | Create role             | 201    |
| GET    | `/roles`                          | List roles              | 200    |
| POST   | `/users/{user_id}/roles/{role_id}`| Assign role to user     | 204    |

```json
// POST /roles — Request
{ "name": "admin" }

// POST /roles — Response 201
{ "id": "<uuidv7>", "name": "admin" }
```

### Permissions

| Method | Path                                          | Description                  | Status |
|--------|-----------------------------------------------|------------------------------|--------|
| POST   | `/permissions`                                | Create permission            | 201    |
| GET    | `/permissions`                                | List permissions             | 200    |
| POST   | `/roles/{role_id}/permissions/{permission_id}`| Assign permission to role    | 204    |

```json
// POST /permissions — Request
{ "name": "products:write" }

// POST /permissions — Response 201
{ "id": "<uuidv7>", "name": "products:write" }
```

## Access model

```
user → user_roles → role → role_permissions → permission
```

Permissions are resolved transitively at login and embedded in the JWT.
