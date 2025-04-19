interface ReadingTileProps {
	label: string
	value: string
}

export default function ReadingTile({ label, value }: ReadingTileProps) {
	return (
		<div className="bg-gray-100 p-4 rounded-lg">
			<div className="text-gray-600">{label}</div>
			<div className="text-xl font-medium">{value}</div>
		</div>
	)
}
