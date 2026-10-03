# Environment Configuration & Container Orchestration

Maintain a strict separation between host development configuration, container orchestration settings, and production secrets.

---

## 1. Dual Environment Template Architecture

> [!CAUTION]
> Never commit `.env` or `.env.docker` files containing active secrets to version control. Only commit `.env.example` and `.env.docker.example` templates with placeholder values.

Every project maintains dedicated templates for host development and container execution:

| Committed Template | Local Target File | Target Runtime Environment |
| :--- | :--- | :--- |
| `backend/.env.example` | `backend/.env` | Backend API running directly on the host machine |
| `backend/.env.docker.example` | `backend/.env.docker` | Backend API running inside Docker Compose |
| `frontend/.env.example` | `frontend/.env` | SvelteKit Vite dev server running on the host |
| `frontend/.env.docker.example` | `frontend/.env.docker` | Frontend image build arguments and browser client values |

### Root `.gitignore` Protection
Enforce strict exclusion of environment files while explicitly preserving template examples:

```gitignore
# Exclude local environment files containing credentials
.env
.env.*
!.env.example
!.env.docker.example
```

---

## 2. Host vs. Container Configuration Matrix

| Variable | Host Development (`backend/.env`) | Container Execution (`backend/.env.docker`) | Rationale |
| :--- | :--- | :--- | :--- |
| `BIND_ADDRESS` | `127.0.0.1` | `0.0.0.0` | Containers require `0.0.0.0` to accept forwarded Docker traffic |
| `PORT` | `8000` | `8000` | Standard HTTP listening port |
| `DATABASE_URL` | `postgres://...127.0.0.1:5432/app` | `postgres://...postgres:5432/app` | Container resolves service name `postgres` via Docker internal DNS |
| `REDIS_URL` | `redis://127.0.0.1:6379` | `redis://...redis:6379` | Container resolves service name `redis` via Docker internal DNS |
| `PUBLIC_API_BASE_URL` | `http://127.0.0.1:8000` | `http://127.0.0.1:8000` | **Browser must reach host machine**, not internal Docker network |

### Frontend Public Environment Variables
> [!IMPORTANT]
> Variables prefixed with `PUBLIC_` are bundled directly into client JavaScript code. Never place secret API keys, private database passwords, or JWT secrets in `PUBLIC_` variables.

See the authoritative [`vite.config.ts`](../templates/frontend/vite.config.ts). It explicitly exposes only `VITE_` and `PUBLIC_` variables to client code; never place secrets in `PUBLIC_` values.

---

## 3. Docker Compose Orchestration (`docker-compose.yml`)

The [`Compose template`](../templates/docker-compose.yml) defines the stateful services, health checks, and containerized application images. Its doubled dollar signs defer shell variable expansion until container startup.

---

## 4. Multi-Stage Dockerfile Patterns

### Backend Dockerfile (`backend/Dockerfile`)
The [`backend Dockerfile`](../templates/backend/Dockerfile) copies migrations before compiling so `sqlx::migrate!()` can embed them.

### Frontend Dockerfile (`frontend/Dockerfile`)
The [`frontend Dockerfile`](../templates/frontend/Dockerfile) builds static SPA assets with Bun and serves them from Nginx.

### Frontend Nginx Configuration (`frontend/nginx.conf`)
The [`nginx.conf`](../templates/frontend/nginx.conf) provides the SPA fallback and immutable asset caching.

### SvelteKit 3 Static Adapter (`frontend/vite.config.ts`)

Configure `@sveltejs/adapter-static` through the `sveltekit()` plugin in [`vite.config.ts`](../templates/frontend/vite.config.ts) to emit the SPA into `build/` with an `index.html` fallback. SvelteKit 3 reads this project configuration from the Vite plugin instead of `svelte.config.js`.
