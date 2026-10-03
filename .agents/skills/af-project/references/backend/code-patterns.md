# Backend Architecture & Code Patterns

This guide defines the architectural standards, layer boundaries, and inviolable code rules for building Rust backend APIs with Actix-web.

---

## 1. Architectural Layers & Unidirectional Data Flow

Requests flow through strictly segregated layers in one direction:

```text
┌─────────────────────────────────────────────────────────────────────┐
│                            HTTP Request                             │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Guards                                                              │
│ └─ Authenticate identity and extract session claims                 │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Routes                                                              │
│ └─ Extract typed JSON, path, or query params; invoke Service        │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Services                                                            │
│ └─ Validate inputs, coordinate transactions, enforce domain rules   │
└────────────────┬───────────────────────────────────┬────────────────┘
                 │                                   │
                 ▼                                   ▼
┌─────────────────────────────────┐ ┌─────────────────────────────────┐
│ Repositories                    │ │ Clients                         │
│ └─ SQLx queries (PostgreSQL)    │ │ └─ External HTTP / AI APIs      │
└────────────────┬────────────────┘ └────────────────┬────────────────┘
                 │                                   │
                 ▼                                   ▼
┌─────────────────────────────────┐ ┌─────────────────────────────────┐
│ Database                        │ │ External Network                │
│ └─ PostgreSQL / Redis           │ │ └─ Remote APIs & Services       │
└─────────────────────────────────┘ └─────────────────────────────────┘
```

### Layer Responsibilities
- **`entities/`**: Defines the data schema across the application: request DTOs, response DTOs, database row structs, domain errors ([`AppError`](foundations.md#2-centralized-error-handling)), and the response envelope ([`BaseResponse<T>`](foundations.md#1-unified-api-response-envelope), [`AppResponse<T>`](foundations.md#1-unified-api-response-envelope)). Contains zero business logic.
- **`routes/`**: Thin Actix-web handlers. Responsible only for parameter extraction and calling the appropriate service method. Always returns `AppResponse<T>`.
- **`services/`**: The core business engine. Validates data, enforces permissions, coordinates database queries through repositories, manages cache via Redis, and calls external services.
- **`repositories/`**: Manages all SQL persistence with PostgreSQL using SQLx. Uses parameterized queries exclusively. Never embeds business validation or authorization rules.
- **`guards/`**: Actix-web `FromRequest` extractors verifying authentication tokens and extracting claims. Resource-level ownership checks remain in the service layer.
- **`clients/`**: Encapsulates external third-party HTTP calls behind abstract traits for unit testability and resilient retry logic.
- **`di.rs`**: Central dependency container. Instantiates database pools, clients, repositories, and services once at startup and registers them into Actix application state.

---

## 2. Inviolable Rust Rules: Do's & Don'ts

| Area | Strictly Required (DO) | Invariant Violation (DON'T) |
| :--- | :--- | :--- |
| **Error Handling** | Return `Result<T, AppError>` and propagate with `?`. | **Never** call `.unwrap()` or `.expect()` in production code paths. |
| **API Envelope** | Return `AppResponse<T>` via `.json()` or `.json_data()`. | **Never** return raw ad-hoc JSON structs or arbitrary strings. |
| **Database Queries** | Bind parameters using `$1, $2` via SQLx. | **Never** format or concatenate user strings into raw SQL. |
| **DTO Placement** | Place all domain structs and DTOs in `entities/`. | **Never** declare DTOs inside route handlers or repositories. |
| **Dependency Injection** | Pass shared services and pools via `web::Data<T>`. | **Never** use global mutable statics or `lazy_static` for services. |
| **Schema Migrations** | Add new sequential files under `migrations/`. | **Never** edit or reorder migrations that have already been applied. |

---

## 3. Sub-Rule Directory Map

Follow these detailed guides to implement each layer:

| Focus Area | Guide | Key Topics |
| :--- | :--- | :--- |
| **Foundations** | [Backend Foundations](foundations.md) | `BaseResponse`, `AppResponse`, `AppError`, `AppConfig`, `setup_db`, `di.rs`, `main.rs` |
| **Vertical Slice** | [Vertical Slice Feature](feature.md) | Full end-to-end feature: DTO → Repo → Service → Route → Guard → Migration → Contract Test |
| **Optional Components** | [Backend Optional Components](optional-components.md) | Argon2id hashing, `UserRole`, revocable Redis multi-session tracking, complete `Cargo.toml` |
