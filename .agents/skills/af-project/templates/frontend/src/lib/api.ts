const BASE_URL = import.meta.env.PUBLIC_API_BASE_URL || 'http://127.0.0.1:8000';

export class ApiError extends Error {
	constructor(
		public override message: string,
		public status: number
	) {
		super(message);
		this.name = 'ApiError';
	}
}

interface ApiEnvelope<T> {
	data: T | null;
	status: number;
	message: string | null;
	timestamp: string;
}

let refreshPromise: Promise<string | null> | null = null;

export function handleAuthFailure(): void {
	if (typeof window !== 'undefined') {
		localStorage.removeItem('auth_token');
		localStorage.removeItem('refresh_token');
		if (!window.location.pathname.startsWith('/login')) {
			window.location.href = '/login';
		}
	}
}

export async function attemptTokenRefresh(): Promise<string | null> {
	const refreshToken = typeof window !== 'undefined' ? localStorage.getItem('refresh_token') : null;
	if (!refreshToken) {
		handleAuthFailure();
		return null;
	}

	if (refreshPromise) return refreshPromise;

	refreshPromise = (async () => {
		try {
			const res = await fetch(`${BASE_URL}/api/users/refresh`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ refresh_token: refreshToken })
			});

			const envelope: ApiEnvelope<{ access_token: string; refresh_token: string }> = await res.json();
			if (!res.ok || envelope.status >= 400 || !envelope.data) {
				handleAuthFailure();
				return null;
			}

			localStorage.setItem('auth_token', envelope.data.access_token);
			localStorage.setItem('refresh_token', envelope.data.refresh_token);
			return envelope.data.access_token;
		} catch {
			handleAuthFailure();
			return null;
		} finally {
			refreshPromise = null;
		}
	})();

	return refreshPromise;
}

export async function request<T>(
	path: string,
	init?: RequestInit,
	token?: string,
	retryOnAuth = true
): Promise<T> {
	const authToken = token ?? (typeof window !== 'undefined' ? localStorage.getItem('auth_token') : undefined);
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		Accept: 'application/json',
		...(authToken ? { Authorization: `Bearer ${authToken}` } : {})
	};

	const res = await fetch(`${BASE_URL}${path}`, {
		...init,
		headers: { ...headers, ...(init?.headers ?? {}) }
	});

	let envelope: ApiEnvelope<T>;
	try {
		envelope = await res.json();
	} catch {
		if (res.status === 401 && retryOnAuth && path !== '/api/users/login' && path !== '/api/users/refresh') {
			const newToken = await attemptTokenRefresh();
			if (newToken) return request<T>(path, init, newToken, false);
		}
		throw new ApiError('An unexpected server response occurred', res.status);
	}

	if (!res.ok || envelope.status >= 400) {
		if ((res.status === 401 || envelope.status === 401) && retryOnAuth && path !== '/api/users/login' && path !== '/api/users/refresh') {
			const newToken = await attemptTokenRefresh();
			if (newToken) return request<T>(path, init, newToken, false);
		}
		throw new ApiError(envelope.message || 'An unexpected error occurred', envelope.status || res.status);
	}

	return envelope.data as T;
}
