import { Button } from '@/shared/ui/button'
import { Input } from '@/shared/ui/input'
import { useState, type FC } from 'react'

interface IProps {
	setSignIn: (signIn: boolean) => void
}

export const SignIn: FC<IProps> = ({ setSignIn }) => {
	const [email, setEmail] = useState<string>('')
	const [password, setPassword] = useState<string>('')

	return (
		<form className='form w-96'>
			<h2 className='font-semibold text-xl'>Sign In</h2>
			<Input
				value={email}
				onChange={e => setEmail(e.target.value)}
				placeholder='Email'
			/>
			<Input
				value={password}
				onChange={e => setPassword(e.target.value)}
				placeholder='Password'
			/>
			<Button>Sign In</Button>
			<p onClick={() => setSignIn(false)}>Create your account</p>
		</form>
	)
}
