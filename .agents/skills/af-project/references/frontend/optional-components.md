# Frontend Optional Components: Auth, Themes, & Toasts

These optional components provide client-side authentication handling, reactive theme toggling, and global toast notifications. Add them only when project requirements demand stateful user sessions and enhanced interactivity.

---

## 1. Authentication & Session State (`src/lib/helpers/auth.svelte.ts`)

The client session state and authentication lifecycle starter implementation is [`auth.svelte.ts`](../../templates/frontend/src/lib/helpers/auth.svelte.ts). It manages token storage, current user session state, and automatic logout handling via `#lib/api.js`.

Client-side route protection can be placed in [`+layout.ts`](../../templates/frontend/src/routes/dashboard/+layout.ts) to prevent unauthenticated users from seeing protected views.

> [!NOTE]
> Client navigation guards improve UX by avoiding blank screen flashes; the backend must independently authenticate and authorize every API request.

---

## 2. Theme Management (`src/lib/helpers/theme.ts` & `ThemeToggle.svelte`)

The theme helper and toggle are maintained in [`theme.ts`](../../templates/frontend/src/lib/helpers/theme.ts) and [`ThemeToggle.svelte`](../../templates/frontend/src/lib/components/ThemeToggle.svelte). The helper reads a stored preference or system setting, applies the `.dark` class, and persists user changes. Include these optional files only when the product needs an in-app theme control.

---

## 3. Global Toast Notifications

The toast store and viewport starter implementations are [`toast.svelte.ts`](../../templates/frontend/src/lib/helpers/toast.svelte.ts) and [`ToastViewport.svelte`](../../templates/frontend/src/lib/components/ToastViewport.svelte). The store owns the reactive queue and timeout-based dismissal; the viewport owns accessible presentation and transitions. Mount the viewport once in the root layout and call the `toast.success`, `toast.error`, or `toast.info` helpers from feature state.

---

## 4. Frontend Package Configuration

Keep runtime and dependency compatibility requirements aligned with the frontend starter and the versions supported by the project environment.

See the authoritative [frontend `package.json`](../../templates/frontend/package.json) for scripts, dependency versions, and the `#lib/*` import mapping. Keep version and command changes there; this reference records the compatibility decision above.
