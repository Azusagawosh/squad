import { createBrowserRouter } from 'react-router'
import { Home } from '@/pages/home'
import { Authorize } from '@/pages/authorize'
import { ProtectedRoute } from '@/feature/auth'

export const router = createBrowserRouter([
	{
		index: true,
		path: '/',
		element: (
			<ProtectedRoute>
				<Home />
			</ProtectedRoute>
		),
	},
	{ path: '/authorize', element: <Authorize /> },
])
