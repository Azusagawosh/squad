import { forwardRef, type ButtonHTMLAttributes } from 'react'

interface IProps extends ButtonHTMLAttributes<HTMLButtonElement> {}

export const Button = forwardRef<HTMLButtonElement, IProps>(
	({ children, ...props }, ref) => {
		return (
			<button
				className='bg-yellow-500 rounded-sm py-2 font-semibold cursor-pointer'
				{...props}
				ref={ref}
			>
				{children}
			</button>
		)
	},
)
