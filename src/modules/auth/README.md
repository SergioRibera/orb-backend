# Auth Module

Handles session management only. Does not own any tables — reads users via IAM's `UserRepository`.

## Endpoints

Base path: `/api/v1/auth`

### POST /login

```json
// Request
{ "email": "user@example.com", "password": "secret" }

// Response 200
{
  "access_token": "<jwt>",
  "refresh_token": "<jwt>",
  "token_type": "Bearer"
}
```

### POST /refresh

```json
// Request
{ "refresh_token": "<jwt>" }

// Response 200
{
  "access_token": "<jwt>",
  "refresh_token": "<jwt>",
  "token_type": "Bearer"
}
```

## Tokens

| Token         | TTL    | Payload                              |
|---------------|--------|--------------------------------------|
| access_token  | 15 min | sub, email, roles[], permissions[]   |
| refresh_token | 7 days | sub                                  |

Algorithm: HS256. Secret from `JWT_SECRET` env var.
