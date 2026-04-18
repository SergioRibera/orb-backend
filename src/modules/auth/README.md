# Auth Module

Handles user registration, login, and JWT token lifecycle.

## Endpoints

Base path: `/api/v1/auth`

### POST `/register`

Creates a new user account.

**Request**
```json
{
  "name": "Juan",
 "second_name": "Carlos",
  "first_surname": "García",
  "second_surname": "López",
  "email": "juan@example.com",
  "password": "secret123"
}
```

> `second_name`, `first_surname`, `second_surname` are optional.

**Response `201 Created`**
```json
{
  "id": "019703a1-...",
  "email": "juan@example.com"
}
```

**Errors**

| Status | Condition |
|--------|-----------|
| `409 Conflict` | Email already registered |
| `500 Internal Server Error` | bcrypt or DB failure |

---

### POST `/login`

Authenticates a user and returns JWT tokens.

**Request**
```json
{
  "email": "juan@example.com",
  "password": "secret123"
}
```

**Response `200 OK`**
```json
{
  "access_token": "<jwt>",
  "refresh_token": "<jwt>",
  "token_type": "Bearer"
}
```

**Errors**

| Status | Condition |
|--------|-----------|
| `401 Unauthorized` | User not found or wrong password |
| `500 Internal Server Error` | bcrypt or JWT failure |

---

### POST `/refresh`

Issues new tokens from a valid refresh token.

**Request**
```json
{
  "refresh_token": "<jwt>"
}
```

**Response `200 OK`**
```json
{
  "access_token": "<jwt>",
  "refresh_token": "<jwt>",
  "token_type": "Bearer"
}
```

**Errors**

| Status | Condition |
|--------|-----------|
| `401 Unauthorized` | Token invalid, expired, or user not found |

---

## Token Lifetimes

| Token         | Lifetime   |
|---------------|------------|
| Access token  | 15 minutes |
| Refresh token | 7 days     |

Both tokens are signed with **HS256** using `JWT_SECRET` from the environment.

## JWT Payload

**Access token**
```json
{
  "sub": "<user_uuid>",
  "email": "juan@example.com",
  "exp": 1234567890
}
```

**Refresh token**
```json
{
  "sub": "<user_uuid>",
  "exp": 1234567890
}
```

## Password Hashing

Passwords are hashed with **bcrypt** at `DEFAULT_COST` (12 rounds) before storage. Plain passwords are never persisted.

## File Structure

```
auth/
├── mod.rs          — route config
├── handler.rs      — HTTP layer (deserialize, call service, serialize)
├── service.rs      — business logic (bcrypt, JWT)
├── repository.rs   — SQL queries
└── model.rs        — User, DTOs, JWT claims
```
