import { redirect } from '@sveltejs/kit';

export const load = () => {
	const token = typeof window !== 'undefined' ? localStorage.getItem('auth_token') : null;
	if (!token) {
		throw redirect(302, '/login');
	}
};
