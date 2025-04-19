import { useState } from "react"
import {
	type BatteryData,
	BatteryState,
	type BatteryStateType,
} from "../types/battery_data"
import ReadingTile from "./ReadingTile"

export default function Home() {
	const [batteryData, setBatteryData] = useState<BatteryData>({
		percentage: 59,
		state: BatteryState.Charging,
		health: 90,
		temperature: 25,
		rate: 99,
		timeToEmpty: 3600,
		timeToFull: 7200,
		serial: "ABCD",
		vendor: "HP",
		model: "HP",
		voltage: 9.1,
		chargeCycles: 300,
		historyPercentage: [
			[1687891200, 95],
			[1687894800, 85],
			[1687898400, 75],
			[1687902000, 65],
			[1687905600, 55],
		],
		historyRate: [
			[1687891200, 12],
			[1687894800, 15],
			[1687898400, 10],
			[1687902000, 8],
			[1687905600, 11],
		],
	})

	const formatBatteryState = (state: BatteryState): BatteryStateType => {
		switch (state) {
			case BatteryState.Unknown:
				return "Unknown"
			case BatteryState.Charging:
				return "Charging"
			case BatteryState.Discharging:
				return "Discharging"
			case BatteryState.Empty:
				return "Empty"
			case BatteryState.FullyCharged:
				return "Fully Charged"
			case BatteryState.PendingCharge:
				return "Pending Charge"
			case BatteryState.PendingDischarge:
				return "Pending Discharge"
		}
	}

	return (
		<div id="home" className="select-none min-h-screen p-8">
			<h1 className="text-4xl font-bold tracking-tight mb-2">Energi</h1>
			<p className="mb-8">Quick stats about your battery</p>

			<div id="stats" className="flex flex-row flex-wrap gap-4">
				<ReadingTile
					label="Charge Level:"
					value={`${batteryData.percentage}%`}
				/>
				<ReadingTile
					label="Battery Status:"
					value={formatBatteryState(batteryData.state)}
				/>
				{batteryData.state === BatteryState.Discharging && (
					<ReadingTile
						label="Estimated Time Remaining:"
						value={`${batteryData.timeToEmpty} seconds`}
					/>
				)}
				{batteryData.state === BatteryState.Charging && (
					<ReadingTile
						label="Estimated Time To Full:"
						value={`${batteryData.timeToFull} seconds`}
					/>
				)}
				<ReadingTile label="Battery Health:" value={`${batteryData.health}%`} />
				<ReadingTile
					label="Battery Discharging Rate:"
					value={`${batteryData.rate}W`}
				/>
				<ReadingTile label="Voltage:" value={`${batteryData.voltage}V`} />
				<ReadingTile label="Model:" value={batteryData.model} />
				<ReadingTile label="Battery Vendor:" value={batteryData.vendor} />
				<ReadingTile label="Serial Number:" value={batteryData.serial} />
			</div>
		</div>
	)
}
