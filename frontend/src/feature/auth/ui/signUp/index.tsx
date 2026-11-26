import { Button } from '@/shared/ui/button'
import { Input } from '@/shared/ui/input'
import { useState, type FC } from 'react'

interface IProps {
	setSignIn: (signIn: boolean) => void
}

export const SignUp: FC<IProps> = ({ setSignIn }) => {
	const [email, setEmail] = useState<string>('')
	const [password, setPassword] = useState<string>('')
	const [repeatPassword, setRepeatPassword] = useState<string>('')

	return (
		<form className='form w-96'>
			<h2 className='font-semibold text-xl'>Sign Up</h2>
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
			<Input
				value={repeatPassword}
				onChange={e => setRepeatPassword(e.target.value)}
				placeholder='Repeat password'
			/>
			<Button>Sign Up</Button>
			<p onClick={() => setSignIn(true)}>Sing in an account</p>
		</form>
	)
}
