import { SignIn } from '@/feature/auth/ui/signIn'
import { SignUp } from '@/feature/auth/ui/signUp'
import { useState, type FC } from 'react'

export const Authorize: FC = () => {
	const [signIn, setSignIn] = useState<boolean>(false)

	return (
		<div className='flex justify-center items-center h-screen px-5'>
			<div className=''>
				{signIn ? (
					<SignIn setSignIn={setSignIn} />
				) : (
					<SignUp setSignIn={setSignIn} />
				)}
			</div>
			<div className='flex-1 bg-red-400'></div>
		</div>
	)
}
