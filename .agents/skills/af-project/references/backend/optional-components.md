# Backend Optional Components: Auth, Sessions, & Caching

These optional components provide production-grade authentication, password hashing, and revocable Redis session management. Add them only when project requirements include user accounts and stateful sessions.

---

## 1. Domain Constants, Roles, & Tokens

- Centralized token lifespans and Redis key prefixes are maintained in [`constants/mod.rs`](../../templates/backend/src/constants/mod.rs).
- Database-mapped role enums and domain token types are defined in [`user_role.rs`](../../templates/backend/src/enums/user_role.rs) and [`token_type.rs`](../../templates/backend/src/enums/token_type.rs).

---

## 2. Password Hashing & JWT Lifecycle

- Password hashing using Argon2id with cryptographically secure random salts is implemented in [`hash_helper.rs`](../../templates/backend/src/helpers/hash_helper.rs).
- HMAC-SHA256 token creation and claims decoding are implemented in [`token_helper.rs`](../../templates/backend/src/helpers/token_helper.rs).

> [!CAUTION]
> Never log raw passwords, access tokens, or password hashes in server log streams or error traces.

---

## 3. Revocable Redis Session Tracking

> [!IMPORTANT]
> **Multi-Device Revocation Invariant**: When a user changes their password, changes roles, or deletes their account, the application must immediately revoke every active session across all devices.

To enable multi-device session revocation:
1. Store session ID → user ID mapping at `auth:session:{session_id}`.
2. Maintain a set of active session IDs per user at `auth:user-sessions:{user_id}`.

- Redis connection manager setup is provided in [`setup_redis.rs`](../../templates/backend/src/setup/setup_redis.rs).
- Multi-device session lifecycle methods (creation, validation, single revocation, and global user revocation) are implemented in [`session_service.rs`](../../templates/backend/src/services/session_service.rs).

---

## 4. Standard Backend Dependencies (`Cargo.toml`)

Keep runtime and dependency compatibility requirements aligned with the backend starter and the versions supported by the project environment.

See the authoritative [backend `Cargo.toml`](../../templates/backend/Cargo.toml) for dependencies and the release profile. Keep version and profile changes there; this reference records the compatibility decision above.
