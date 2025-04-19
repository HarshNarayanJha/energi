export enum BatteryState {
	Unknown = 0,
	Charging = 1,
	Discharging = 2,
	Empty = 3,
	FullyCharged = 4,
	PendingCharge = 5,
	PendingDischarge = 6,
}

export type BatteryStateType =
	| "Unknown"
	| "Charging"
	| "Discharging"
	| "Empty"
	| "Fully Charged"
	| "Pending Charge"
	| "Pending Discharge"

export interface BatteryData {
	percentage: number
	state: BatteryState
	health: number
	temperature: number
	rate: number
	timeToEmpty: number
	timeToFull: number
	serial: string
	vendor: string
	model: string
	voltage: number
	chargeCycles: number
	historyPercentage: Array<[number, number]>
	historyRate: Array<[number, number]>
}
