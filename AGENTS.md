# Agent & Contributor Guide

Welcome to Octopus. This document establishes the core architectural principles, golden rules, and verification procedures under the `af-project` standard.

---

## 1. Inviolable Golden Rules

1. **Strict Layering**: Routes only extract HTTP inputs and invoke services. Services orchestrate business logic and external I/O. All domain DTOs strictly reside in `backend/src/entities/`.
2. **Unified API Envelope**: Every endpoint returns `AppResponse<T>` (`Result<Json<BaseResponse<T>>, AppError>`) via `.json()` or `.json_data()`. Never return raw, ad-hoc JSON.
3. **No Unwraps in Production**: Use `Result<T, AppError>` and `?` error propagation exclusively in non-test Rust paths.
4. **Svelte 5 Runes Only**: Strictly use `$props`, `$state`, `$derived`, and `$effect`. No legacy Svelte 3/4 syntax (`export let`, `$:`, `on:click`).
5. **CSR Only**: Set `export const ssr = false;` and `export const prerender = false;` in root `+layout.ts`. Never create server routes (`+page.server.ts` or `+server.ts`).
6. **Zero-Egress E2E**: Playwright tests must mock all API endpoints and run against the frontend dev server only.
7. **Natural Language Commits**: When a commit is authorized, write a concise, descriptive message without conventional commit prefixes (`feat:`, `fix:`).
8. **No `@layer components` for Views**: Never use Tailwind `@layer components` or global `@apply` abstractions for feature- or page-specific styling. Colocate styles directly in Svelte components with utility classes or scoped `<style>` blocks.
9. **Endpoint Conventions**: Endpoints use `/canvas/` prefix (e.g. `/canvas/models`, `/canvas/evaluate`).
10. **Storage Constraint**: No PostgreSQL or Redis in this application; client state is maintained in Svelte runes and synced to `localStorage`.

---

## 2. Verification Checklist

Select checks based on the files and behavior changed:

### Backend Checks
```sh
cd backend
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features --locked -- -D warnings
```

### Frontend Checks
```sh
cd frontend
bun run check
bun run lint
bun run test:unit
bun run test:e2e
```
