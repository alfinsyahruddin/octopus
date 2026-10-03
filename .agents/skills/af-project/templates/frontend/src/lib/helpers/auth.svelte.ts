import { request, handleAuthFailure } from '#lib/api.js';

export interface UserSession {
	id: string;
	email: string;
	role: 'ADMIN' | 'MEMBER';
}

class AuthState {
	user = $state<UserSession | null>(null);
	isAuthenticated = $derived(this.user !== null);
	isLoading = $state(true);

	init() {
		const token = typeof window !== 'undefined' ? localStorage.getItem('auth_token') : null;
		if (token) {
			this.fetchCurrentUser();
		} else {
			this.isLoading = false;
		}
	}

	async fetchCurrentUser() {
		this.isLoading = true;
		try {
			this.user = await request<UserSession>('/api/users/me');
		} catch {
			this.user = null;
			handleAuthFailure();
		} finally {
			this.isLoading = false;
		}
	}

	setSession(token: string, refreshToken: string, user: UserSession) {
		localStorage.setItem('auth_token', token);
		localStorage.setItem('refresh_token', refreshToken);
		this.user = user;
	}

	logout() {
		this.user = null;
		handleAuthFailure();
	}
}

export const authState = new AuthState();
