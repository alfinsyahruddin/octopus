# Backend Foundations

Use the starter files as the source of truth for response envelopes, errors, configuration, database setup, dependency injection, and server startup. The reference here records the decisions those files encode and the integration steps for a new backend.

## Response and error contract

- Every handler returns `AppResponse<T>` from [`app_response.rs`](../../templates/backend/src/entities/app_response.rs), which wraps `BaseResponse<T>` from [`base_response.rs`](../../templates/backend/src/entities/base_response.rs).
- Convert successful `Result<T, AppError>` values with `.json()`. Use `json_data()` for a payload without a fallible operation and `json()` for a message-only success.
- `AppError` in [`app_error.rs`](../../templates/backend/src/entities/app_error.rs) owns HTTP status mapping and the public error envelope. Keep internal error details in server logs; return the generic internal error message to clients.
- Add conversions for new infrastructure error types in `AppError`; keep business decisions in services and HTTP extraction in routes.

## Startup integration

The starter files are grouped by responsibility:

| Concern | Authoritative starter |
| --- | --- |
| Environment parsing and required settings | [`app_config.rs`](../../templates/backend/src/entities/app_config.rs) |
| Database pool creation | [`setup_db.rs`](../../templates/backend/src/setup/setup_db.rs) |
| Shared HTTP client creation | [`setup_http_client.rs`](../../templates/backend/src/setup/setup_http_client.rs) |
| Dependency ownership and service wiring | [`di.rs`](../../templates/backend/src/di.rs) |
| Middleware, route registration, and error mapping | [`http.rs`](../../templates/backend/src/http.rs) |
| Process startup and graceful shutdown | [`main.rs`](../../templates/backend/src/main.rs) |
| Root response routes | [`routes/mod.rs`](../../templates/backend/src/routes/mod.rs) |

When adapting a starter, preserve fallible startup (`Result` and `?`), construct shared dependencies once, and pass them through application state. Keep the root health route independent of optional stores so deployment probes can report process health without requiring Redis.

## Adding a foundation module

1. Copy the needed starter files from [`templates/backend/src/`](../../templates/backend/src/).
2. Add the corresponding module declarations to the target crate and add only the dependencies required by the selected components.
3. Put request and response DTOs in `entities/`, SQL in repositories, business rules in services, and HTTP extraction in routes.
4. Add an HTTP contract test for each new status or response-envelope behavior. For protected routes, verify that a missing credential returns HTTP `401` and the standard error envelope.

The complete implementation stays in the starter files. Keep excerpts here limited to decisions and integration-specific examples so the reference cannot drift from the copied code.
