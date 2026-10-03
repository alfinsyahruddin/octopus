# Workflow & Verification Standards

Follow these verification procedures, commands, and contributor workflows to ensure code quality, test reliability, and smooth task handoff.

---

## 1. Automated Verification Commands

Choose verification based on the files and behavior changed. Run the smallest set of checks that gives useful coverage; run the full suite when a change spans multiple areas, affects shared foundations, or carries enough risk that focused checks are insufficient. A task does not require checks for untouched areas just because they are listed below.

Inspect the project's scripts and existing verification setup before choosing checks. Report the exact commands run and their results. If a check fails, determine whether the failure is caused by the current change. Compare with the pre-change state when practical; otherwise describe the failure and evidence without labeling it pre-existing. Do not hide failures or attribute unrelated failures to the change.

### Backend Quality Gates
Execute from `backend/`:

```sh
# 1. Format check
cargo fmt --check

# 2. Unit and HTTP contract tests
cargo test

# 3. Strict Clippy linting (warnings treated as errors)
cargo clippy --all-targets --all-features --locked -- -D warnings
```

### Frontend Quality Gates
Execute from `frontend/`:

```sh
# 1. Svelte & TypeScript type check
bun run check

# 2. ESLint code quality check
bun run lint

# 3. Unit and component tests (always run via bun run test:unit, never plain bun test)
bun run test:unit

# 4. Code formatting check
bun run format:check

# 5. Playwright E2E browser tests (with dev server only)
bun run test:e2e
```

---

## 2. Infrastructure & Docker Operations

Run these commands from the repository root:

```sh
# Start backing services (PostgreSQL & Redis) in background for host development
docker compose up postgres redis -d

# Start the full containerized stack (Postgres, Redis, Backend, Frontend)
docker compose up -d --build

# View real-time container logs
docker compose logs -f

# Stop and tear down all project containers and networks
docker compose down
```

---

## 3. Database Migration Verification

When modifying database schemas:
1. **Append-Only Rule**: Never edit or reorder an existing migration that has already been applied. Always create a new sequential file under `backend/migrations/` (e.g., `YYYYMMDDHHMM_description.sql`).
2. **Clean-Slate Verification**: Verify migrations apply cleanly to a completely fresh database instance without errors:
   ```sh
   sqlx migrate run
   ```
3. **Compile-Time Embedding**: Remember that `sqlx::migrate!()` runs at compile-time in Rust. Ensure new SQL files are present during `cargo check`, `cargo build`, and container image creation. Commit them only when a commit is authorized.

---

## 4. Pre-Handoff Review

Before reporting a task complete or submitting a pull request:

| Gate | Check | Expected Outcome |
| :--- | :--- | :--- |
| **Relevant checks** | Select applicable commands from the sections above | Checks cover the changed behavior; untouched areas need not be checked |
| **Failures** | Record command, result, and attribution evidence | Failures caused by the change are distinguished from unrelated or baseline failures |
| **Git review** | `git status --short` and inspect the diff | Intended changes are reviewable; unrelated user changes are preserved |

An uncommitted working tree is a valid handoff. Do not require a clean tree or remove unrelated changes to complete a task. Summarize which changes remain uncommitted when relevant.

---

## 5. Contributor Handoff Standards

> [!NOTE]
> **Commit Message Style (when a commit is authorized)**:
> - Use clear, descriptive natural language summaries in title or sentence case (e.g., `Add user profile settings endpoint and validation`).
> - **Do NOT use Conventional Commit prefixes** (never use `feat:`, `fix:`, `chore:`).
> - **Multiline list style is intended**: When a commit spans multiple distinct parts or changes, a multiline format (a concise summary title followed by a blank line and dashed bullet details) is explicitly intended and recommended.

#### Commit Message Format Examples:
**Single-line format:**
```text
Add user profile settings endpoint and validation
```

**Multiline list format (intended for multi-part changes):**
```text
Add user profile settings endpoint and validation

- Implement settings route, service, and DTO validation in backend
- Add settings repository query with parameterized SQL
- Add Svelte 5 settings component in frontend and wire with api.ts
- Add Playwright E2E journey test covering settings update
```

Commit message examples and formatting guidance describe style only; they do not authorize creating a commit. Commit only when the user has asked for it or has already granted applicable authorization. When finishing an implementation task:
1. **Summary of Changes**: Detail what was implemented, updated, or refactored.
2. **Verification Statement**: List exactly which commands ran and their results, including relevant failures or checks not run.
3. **Operational Limits**: Note any known edge cases, pending credentials, or remaining out-of-scope work.
4. **Preserve User Changes**: Never discard unrelated files or modifications present in the working tree.
