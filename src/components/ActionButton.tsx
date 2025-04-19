interface ActionButtonProps {
  onClick: () => void
  children: React.ReactNode
}

export default function ActionButton({ onClick, children }: ActionButtonProps) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="px-4 py-2 w-auto rounded-md bg-stone-600 text-white hover:bg-stone-500 active:bg-stone-700"
    >
      {children}
    </button>
  )
}
