import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import '@/app/style/global.css'
import App from './app/App.tsx'
import { AuthProvider } from './feature/auth/index.ts'

createRoot(document.getElementById('root')!).render(
	<StrictMode>
		<AuthProvider>
			<App />
		</AuthProvider>
	</StrictMode>,
)
