export interface ApiEnvelope<T> {
	data: T | null;
	status: number;
	message: string | null;
	timestamp: string;
}

export interface UserSession {
	id: string;
	email: string;
	role: 'ADMIN' | 'MEMBER';
}

export interface AuthTokens {
	access_token: string;
	refresh_token: string;
}
