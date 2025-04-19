interface ReadingTileProps {
  label: string
  value: string
}

export default function ReadingTile({ label, value }: ReadingTileProps) {
  return (
    <div className="flex flex-col items-center justify-center p-6 rounded-lg shadow-md bg-white dark:bg-stone-800 dark:text-white">
      <div className="text-xl font-bold tracking-tight mb-2 text-gray-900 dark:text-white">
        {label}
      </div>
      <div className="text-gray-600 dark:text-gray-300">{value}</div>
    </div>
  )
}
