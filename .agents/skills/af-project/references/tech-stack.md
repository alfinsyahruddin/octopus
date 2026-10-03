# Tech Stack Defaults & Criteria

These conventions define the standard defaults when building a new Rust backend or SvelteKit web frontend under the `af-project` guideline.

---

## 1. Backend Stack Matrix

| Concern | Standard Default | Criteria to Add | Rationale |
| :--- | :--- | :--- | :--- |
| **API Runtime** | Rust, Tokio, Actix-web 4 | Core HTTP runtime for all backend services | High throughput, memory safety, strong concurrency |
| **Relational Database** | PostgreSQL 16+, SQLx | When persistent, structured relational data is required | Pure SQL control, compile-time query verification |
| **Schema Migrations** | SQLx Migrations | Append-only SQL files embedded at compile time via `sqlx::migrate!()` | Zero-dependency deployment; binary runs migrations on startup |
| **Shared State & Caching** | None initially | Redis 7+ for revocable user sessions, rate limiting, queues, or distributed cache | Sub-millisecond distributed in-memory key-value storage |
| **Outbound HTTP** | Reqwest (rustls-tls, json) | When calling external APIs, encapsulated in dedicated client adapters | Async connection pooling and TLS support |
| **Security & Auth** | Argon2id, JWT (jsonwebtoken) | For account registration, password hashing, and token authentication | Cryptographically secure against GPU/ASIC attacks |
| **Validation** | Validator crate, typed errors | Input validation at service and DTO boundaries | Clean declarative struct field validation |
| **Environment Loading** | `dotenvy` crate | Loads `.env` during local host execution without error if file is missing in container | Predictable local developer workflow |
| **Release Optimization** | `[profile.release]` (LTO fat, strip symbols, panic=abort) | Production binary compilation | Lean, high-speed release binary with minimal size |
| **Code Quality** | Rustfmt, Clippy (`-D warnings`) | Static analysis and formatting enforced on every commit | Eliminates compiler warnings, enforces idiomatic Rust |

---

## 2. Frontend Stack Matrix

| Concern | Standard Default | Criteria to Add | Rationale |
| :--- | :--- | :--- | :--- |
| **Web Framework** | SvelteKit 3, Svelte 5, TypeScript | Single-Page Application (CSR only, `ssr = false`) | High performance, fine-grained reactivity with Runes |
| **Static SPA Adapter** | `@sveltejs/adapter-static` (`fallback: 'index.html'`) | Production SPA asset build served by Nginx Alpine | Emits pure static HTML/CSS/JS without Node.js server overhead |
| **Package Manager** | Bun | Fast package management, script runner, and lockfile provider | Sub-second dependency installation and fast execution |
| **CSS & Design Tokens** | Tailwind CSS v4 (`@tailwindcss/vite`) | Utility classes styled with CSS variables and custom `@theme` design tokens | Zero-runtime CSS with modern cascade layers |
| **Iconography** | `@iconify/svelte` with Lucide IDs | Scalable vector icons using standard Lucide identifiers (e.g. `lucide:sun`) | Tree-shakeable, uniform icon set |
| **Charts & Visuals** | Native SVG or charting libraries | Add domain-specific charting libraries (e.g., `lightweight-charts`, Chart.js) when required | Lightweight and accessible data visualization |
| **Unit & Component Testing** | Vitest, JSDOM | Unit tests for pure utilities and component behavior | Fast ESM-native test runner |
| **Browser E2E Testing** | Playwright (Chromium) | Critical user journeys with 100% mocked backend API endpoints | Reliable browser automation without flakiness |

---

## 3. Project Infrastructure Matrix

| Concern | Standard Default | Criteria to Add | Rationale |
| :--- | :--- | :--- | :--- |
| **Local Orchestration** | Docker Compose (`docker-compose.yml` in root) | Local backing services (PostgreSQL, optional Redis) and full-stack testing | Reproducible local environment matching production |
| **Containerization** | Multi-stage Dockerfiles | Rust release compilation (Alpine) and Bun static build to Nginx Alpine SPA | Minimal runtime image sizes without build tools |
| **Configuration** | Service-specific dual `.env` templates | Distinct host (`.env.example`) and container (`.env.docker.example`) in `backend/` and `frontend/` | Clear separation of host localhost vs container network |

---

## 4. Architectural Guardrails: What NOT to Use

> [!CAUTION]
> Avoid introducing the following patterns or libraries:
> - **Do NOT use ORMs (Diesel, SeaORM)**: Always use SQLx with parameterized SQL. We prioritize explicit query control, schema predictability, and compile-time verification over ORM magic.
> - **Do NOT use Legacy Svelte Syntax**: Never generate Svelte 3/4 idioms (`export let`, `$:`, `on:click`). Always use Svelte 5 runes (`$props`, `$state`, `$derived`, `onclick`).
> - **Do NOT use npm/yarn/pnpm**: Always use `bun` as the project package manager and script runner to maintain lockfile consistency.
> - **Do NOT embed external calls directly in services**: Always wrap outbound third-party APIs in a `clients/` adapter behind a trait.
> - **Do NOT use Tailwind `@layer components` for feature or page styling**: Colocate styles directly in Svelte components with utility classes or scoped `<style>` blocks. Keep global `src/app.css` strictly for design tokens, CSS variables, and base resets.
