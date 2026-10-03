export type ToastType = "success" | "error" | "info";

export interface ToastItem {
  id: number;
  type: ToastType;
  message: string;
}

let nextId = 0;

export const toasts: ToastItem[] = $state([]);

/**
 * Displays a toast notification and removes it after the given duration.
 */
export function showToast(
  type: ToastType,
  message: string,
  duration = 4000,
): void {
  const id = nextId++;
  toasts.push({ id, type, message });
  setTimeout(() => {
    const idx = toasts.findIndex((t) => t.id === id);
    if (idx !== -1) toasts.splice(idx, 1);
  }, duration);
}

/**
 * Dismisses a toast immediately by ID.
 */
export function dismissToast(id: number): void {
  const idx = toasts.findIndex((t) => t.id === id);
  if (idx !== -1) toasts.splice(idx, 1);
}

/** Convenience toast helpers */
export const toast = {
  success: (msg: string, duration?: number) =>
    showToast("success", msg, duration),
  error: (msg: string, duration?: number) => showToast("error", msg, duration),
  info: (msg: string, duration?: number) => showToast("info", msg, duration),
  dismiss: (id: number) => dismissToast(id),
};
