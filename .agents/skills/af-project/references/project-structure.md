# Project Structure & File Placement

Maintain a clean, predictable repository directory structure where code, tests, configuration, and documentation reside in dedicated, discoverable locations.

---

## 1. Full-Stack Repository Tree

The standard repository layout for a Rust/Actix API and SvelteKit web application:

```text
<project>/
├── backend/                      # Rust API service (Tokio, Actix-web, SQLx, Redis)
│   ├── migrations/               # Sequential, append-only SQLx migration scripts
│   ├── src/
│   │   ├── clients/              # External API adapters and third-party HTTP clients
│   │   ├── constants/            # Global constants (e.g., Redis key prefixes, limits)
│   │   ├── entities/             # Domain structs, DTOs, errors, response envelopes, config
│   │   ├── enums/                # System-wide domain enums and database-mapped types
│   │   ├── guards/               # Actix extractors (authentication, role-based authorization)
│   │   ├── helpers/              # Pure utility functions (hashing, math, parsing)
│   │   ├── repositories/         # Direct PostgreSQL query execution (SQLx)
│   │   ├── routes/               # Actix HTTP route handlers and parameter extraction
│   │   ├── services/             # Business logic orchestration, validation, domain rules
│   │   ├── setup/                # Infrastructure initialization (database pools, Redis, HTTP)
│   │   ├── di.rs                 # Dependency injection container and app data wiring
│   │   ├── http.rs               # Middleware, CORS configuration, centralized error handling
│   │   ├── lib.rs                # Library entrypoint exporting modules for tests
│   │   └── main.rs               # Server runtime bootstrap entrypoint
│   ├── tests/                    # Integration and HTTP contract test suites
│   ├── Cargo.toml                # Rust package manifest
│   ├── Dockerfile                # Multi-stage production container build (Alpine)
│   ├── .dockerignore             # Docker build context exclusion list
│   ├── .env.example              # Host development environment template
│   └── .env.docker.example       # Docker container environment template
│
├── frontend/                     # SvelteKit 3 single-page web client (Svelte 5, Bun, Tailwind v4)
│   ├── src/
│   │   ├── app.css               # Tailwind CSS v4 design tokens and CSS variables
│   │   ├── app.html              # Shell HTML template
│   │   ├── lib/
│   │   │   ├── api.ts            # Typed API client with unified response handling
│   │   │   ├── constants.ts      # Shared UI constants and configuration keys
│   │   │   ├── types.ts          # Frontend domain interfaces and type definitions
│   │   │   ├── components/       # Svelte 5 UI components (shared, layout, and feature-specific)
│   │   │   └── helpers/          # Client utilities, reactive state modules (.svelte.ts)
│   │   └── routes/               # CSR client routes (+layout.svelte, +layout.ts, +page.svelte)
│   ├── tests/
│   │   ├── unit/                 # Vitest unit and component tests
│   │   └── e2e/                  # Playwright browser end-to-end user journey tests
│   ├── package.json              # Bun dependencies and scripts
│   ├── vitest.config.ts          # Kit 3 component tests with JSDOM
│   ├── playwright.config.ts      # Playwright test configuration
│   ├── tsconfig.json             # TypeScript configuration extending $app/tsconfig
│   ├── vite.config.ts            # Kit 3 adapter, preprocessing, Tailwind, and environment config
│   ├── nginx.conf                # Nginx Alpine SPA routing fallback and cache rules
│   ├── Dockerfile                # Multi-stage container build (Bun -> Nginx Alpine SPA)
│   ├── .dockerignore             # Docker build context exclusion list
│   ├── .env.example              # Host development environment template
│   └── .env.docker.example       # Container build environment template
│
├── docs/                         # Detailed architecture, environment, and contributor guides
├── docker-compose.yml            # Local development backing services and full-stack orchestration
├── AGENTS.md                     # Contributor entrypoint and critical architecture invariants
├── LICENSE                       # Terms selected for this project's distribution
└── README.md                     # Product overview, prerequisites, and quick-start instructions
```

---

## 2. File Placement Quick Lookup

When adding new files or features, use this lookup table to determine the exact location:

| If you are creating... | Place it in... | Notes / Constraints |
| :--- | :--- | :--- |
| **Request / Response DTO** | `backend/src/entities/` | Never define DTOs inside route handlers or repositories. |
| **Database Row Model** | `backend/src/entities/` | Derived with `sqlx::FromRow` where applicable. |
| **Domain Error Variants** | `backend/src/entities/app_error.rs` | Managed in centralized `AppError` enum. |
| **Raw SQL Queries** | `backend/src/repositories/` | Parameterized SQL queries only (`$1, $2`). |
| **Business Logic / Workflow** | `backend/src/services/` | Coordinates repositories, validation, and caching. |
| **HTTP Route Handler** | `backend/src/routes/` | Thin extractors returning `AppResponse<T>`. |
| **Auth / Role Check Extractor** | `backend/src/guards/` | Implements Actix `FromRequest`. |
| **Third-Party API Adapter** | `backend/src/clients/` | Define an abstract trait for unit test mockability. |
| **Database Migration Script** | `backend/migrations/` | Sequential prefix (`YYYYMMDDHHMM_name.sql`). Append-only. |
| **HTTP Contract Test** | `backend/tests/` | Black-box HTTP tests using `actix_web::test`. |
| **Reusable UI Component** | `frontend/src/lib/components/` | Svelte 5 runes only (`$props`, `$state`, `$derived`). |
| **Client Reactive Store** | `frontend/src/lib/helpers/` | Encapsulate in `.svelte.ts` modules. |
| **Client HTTP Transport** | `frontend/src/lib/api.ts` | Centralized typed wrapper around `fetch()` with auto-refresh deduplication. |
| **Client Navigation Route** | `frontend/src/routes/` | Page layout and view rendering. |
| **Browser E2E Test** | `frontend/tests/e2e/` | Playwright journey test with 100% mocked API calls. |
| **SPA Fallback Server Conf** | `frontend/nginx.conf` | Alpine Nginx config routing client paths to `index.html`. |
| **Static Adapter Config** | `frontend/vite.config.ts` | Passes `@sveltejs/adapter-static` with fallback `index.html` to `sveltekit()`. |
| **Tailwind & Vite Setup** | `frontend/vite.config.ts` | Configures `@tailwindcss/vite` and `envPrefix: ['VITE_', 'PUBLIC_']`. |

---

## 3. Placement Guardrails

> [!WARNING]
> - **No Server Routes in Frontend**: The frontend is strictly a Single-Page Application (`ssr = false`). Never create `+page.server.ts` or `+server.ts` files in `frontend/src/routes/`.
> - **No SQL in Services**: Services must never execute raw SQL or know about database drivers. Database calls must always go through `repositories/`.
> - **No Business Rules in Routes**: Handlers must only deserialize parameters, call a service method, and map the result to `AppResponse<T>`.
