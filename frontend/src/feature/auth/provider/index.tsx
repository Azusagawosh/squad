import type { User, UserData } from '../type'
import { AuthContext } from '../context'
import { useState, type FC, type PropsWithChildren } from 'react'
import { api } from '@/shared/axios'

export const AuthProvider: FC<PropsWithChildren> = ({ children }) => {
	const [isAuth, setIsAuth] = useState<boolean | undefined>(undefined)
	const [user, setUser] = useState<User | null>(null)

	const saveSession = (data: {
		access_token: string
		refresh_token: string
		user: UserData
	}) => {
		localStorage.setItem('access_token', data.access_token)
		localStorage.setItem('refresh_token', data.refresh_token)
		setUser({
			accessToken: data.access_token,
			refreshToken: data.refresh_token,
			user: data.user,
		})
		setIsAuth(true)
	}

	const clearSession = () => {
		setUser(null)
		setIsAuth(false)
		localStorage.removeItem('access_token')
		localStorage.removeItem('refresh_token')
		localStorage.removeItem('token')
	}

	const registerUserWithEmailAndPassword = async (
		username: string,
		email: string,
		password: string,
		repeatPassword: string,
		setIsLoading: (IsLoading: boolean) => void,
	) => {
		if (password !== repeatPassword) {
			return
		}

		setIsLoading(true)

		await api
			.post('/auth/register', { username, email, password })
			.then(res => {
				saveSession(res.data)
			})
			.catch(e => {
				console.log(e)
			})
			.finally(() => {
				setIsLoading(false)
			})
	}

	const loginWithEmailAndPassword = async (
		email: string,
		password: string,
		setIsLoading: (IsLoading: boolean) => void,
	) => {
		setIsLoading(true)

		await api
			.post('/auth/login', { email, password })
			.then(res => {
				saveSession(res.data)
			})
			.catch(e => {
				console.log(e)
			})
			.finally(() => {
				setIsLoading(false)
			})
	}

	const logout = async () => {
		const refreshToken = localStorage.getItem('refresh_token')
		try {
			if (refreshToken)
				await api.post('/auth/logout', { refresh_token: refreshToken })
		} catch (error) {
			console.warn('Logout request failed; clearing the local session.', error)
		} finally {
			clearSession()
		}
	}

	const changePassword = async (oldPassword: string, newPassword: string) => {
		await api.patch('/users/me/password', {
			old_password: oldPassword,
			new_password: newPassword,
		})
	}

	const deleteAccount = async () => {
		await api.delete('/users/me')
	}

	return (
		<AuthContext.Provider
			value={{
				isAuth,
				user,
				loginWithEmailAndPassword,
				logout,
				changePassword,
				deleteAccount,
				registerUserWithEmailAndPassword,
			}}
		>
			{children}
		</AuthContext.Provider>
	)
}
