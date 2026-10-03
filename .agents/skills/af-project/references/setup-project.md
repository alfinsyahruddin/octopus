# Project Setup & Bootstrap

Use this guide when initiating a new personal project or setting up a new repository under the `af-project` guideline.

---

## 1. Project Contract Definition

> [!IMPORTANT]
> Never generate code files or directory scaffolding before clarifying the project contract.

Define and document these 5 fundamental requirements:
1. **Core Purpose**: One clear sentence stating the problem this software solves.
2. **Target Platform**: Target runtime (e.g., Web SPA + REST API, Mobile app, CLI tool).
3. **First Vertical Slice**: The minimal end-to-end user journey that delivers tangible value (e.g., "User loads settings, modifies notification preference, and persists to database").
4. **Data & Integration Boundaries**: Required database models, persistent storage, third-party APIs, and external authentication providers.
5. **Names & Identifiers**: Distinct crate names, package identifiers, and repository names.

---

## 2. Ordered Bootstrap Sequence

For standard Rust/Actix backend and SvelteKit web applications, follow this 5-phase bootstrap sequence:

```text
Phase 1: Root Manifests ──► Phase 2: Backend Scaffolding ──► Phase 3: Frontend Scaffolding
                                                                      │
Phase 5: Baseline Verification ◄── Phase 4: Environment & Infra ◄─────┘
```

### Phase 1: Repository Root & Manifests
1. Initialize the git repository:
   ```sh
   git init
   ```
2. Create root `.gitignore` to prevent secret leaks:
   ```gitignore
   .env
   .env.*
   !.env.example
   !.env.docker.example
   target/
   node_modules/
   build/
   .svelte-kit/
   ```
3. Create initial `README.md` and `AGENTS.md` following [Documentation Guidelines](documentation.md).

### Phase 2: Backend Scaffolding
1. Scaffold `backend/` as a Rust package:
   ```sh
   cargo new --bin backend
   ```
2. Establish directory structure from [Project Structure](project-structure.md):
   ```sh
   mkdir -p backend/src/{clients,constants,entities,enums,guards,helpers,repositories,routes,services,setup}
   mkdir -p backend/{migrations,tests}
   ```
3. Add standard dependencies from [Tech Stack](tech-stack.md) into `backend/Cargo.toml`.
4. Implement foundation primitives from [Backend Foundations](backend/foundations.md) (ready-to-copy starter files available in [`templates/backend/src/`](../templates/backend/src/)):
   - `entities/base_response.rs` and `entities/app_response.rs`
   - `entities/app_error.rs`
   - `entities/app_config.rs`
   - `setup/setup_db.rs`
   - `di.rs`, `http.rs`, and `main.rs`
5. Create initial baseline migration in `backend/migrations/202601010001_initial_schema.sql`.

### Phase 3: Frontend Scaffolding
1. Scaffold `frontend/` as a SvelteKit single-page app with Bun:
   ```sh
   bunx sv create frontend
   # Select: minimal app, TypeScript, Prettier, ESLint, Vitest, Playwright, Bun
   ```
2. Configure pure Client-Side Single-Page Application (CSR) mode:
   - In `frontend/src/routes/+layout.ts`:
     ```ts
     export const ssr = false;
     export const prerender = false;
     ```
   - Install the Kit 3 static adapter:
     ```sh
     bun add --cwd frontend -d @sveltejs/adapter-static@^4
     ```
   - Configure the adapter in `frontend/vite.config.ts` as shown below.
3. Install Tailwind CSS v4 and iconography:
   ```sh
   bun add --cwd frontend -d @tailwindcss/vite tailwindcss
   bun add --cwd frontend @iconify/svelte
   ```
   Add `@import 'tailwindcss';` and `@custom-variant dark (&:where(.dark, .dark *));` to `src/app.css`.
   Configure `vite.config.ts` (SvelteKit 3 configures the adapter and preprocessing directly in the Vite plugin):
   ```ts
   import adapter from '@sveltejs/adapter-static';
   import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
   import { sveltekit } from '@sveltejs/kit/vite';
   import tailwindcss from '@tailwindcss/vite';
   import { defineConfig } from 'vite';

   export default defineConfig({
     envPrefix: ['VITE_', 'PUBLIC_'],
     plugins: [
       tailwindcss(),
       sveltekit({
         preprocess: vitePreprocess(),
         adapter: adapter({ fallback: 'index.html' })
       })
     ],
     server: {
       port: 3000
     }
   });
   ```
4. Declare `#lib/*` subpath imports in `package.json` and extend `$app/tsconfig` in `tsconfig.json` as shown in [Frontend Foundations](frontend/foundations.md#4-sveltekit-3-build--adapter-configuration).
5. Set up foundation primitives from [Frontend Foundations](frontend/foundations.md) (ready-to-copy starter files available in [`templates/frontend/`](../templates/frontend/)).

### Phase 4: Environment & Infrastructure Configuration
1. Create environment templates for host and Docker:
   - `backend/.env.example` & `backend/.env.docker.example`
   - `frontend/.env.example` & `frontend/.env.docker.example`
2. Create local copies:
   ```sh
   cp backend/.env.example backend/.env
   cp frontend/.env.example frontend/.env
   ```
3. Configure `docker-compose.yml` in the repository root for local backing services (PostgreSQL, optional Redis) as detailed in [Environment and Compose](environment-and-docker.md) (starter template available in [`templates/docker-compose.yml`](../templates/docker-compose.yml)).

### Phase 5: Baseline Verification
> [!TIP]
> Always verify that the empty shell compiles, lints, and starts before writing business logic.

Choose checks for the platform and components scaffolded. For this standard backend-and-frontend stack, run the applicable backend and frontend baseline checks below. Do not start optional services or run checks for components that are not part of the setup. For later tasks, use the proportional verification guidance in [Workflow & Verification](workflow.md).

1. Start only the backing services required by the scaffold (Redis is optional unless the app uses it):
   ```sh
   docker compose up postgres -d
   ```
   If Redis is configured and needed, start it with `docker compose up redis -d`.
2. Run backend checks:
   ```sh
   cd backend && cargo fmt --check && cargo test && cargo clippy --all-targets --all-features --locked -- -D warnings
   ```
3. Run frontend checks:
   ```sh
   cd frontend && bun run check && bun run lint && bun run test:unit && bun run format:check
   ```
4. Review the changed files and working tree. An initial commit is optional and requires the user's authorization; the commit message examples in [Workflow & Verification](workflow.md) specify style, not permission to commit.
