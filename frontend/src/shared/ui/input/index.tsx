import { forwardRef, type InputHTMLAttributes } from 'react'

interface IProps extends InputHTMLAttributes<HTMLInputElement> {}

export const Input = forwardRef<HTMLInputElement, IProps>(
	({ ...props }, ref) => {
		return (
			<input className='border rounded-lg py-2 px-5' {...props} ref={ref} />
		)
	},
)
