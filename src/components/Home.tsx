import { invoke } from "@tauri-apps/api/core"
import prettyMs from "pretty-ms"
import { useEffect, useState } from "react"
import {
  type BatteryData,
  BatteryState,
  type BatteryStateType,
} from "../types/battery_data"
import ActionButton from "./ActionButton"
import LineGraph from "./LineGraph"
import ReadingTile from "./ReadingTile"

export default function Home() {
  const [batteryData, setBatteryData] = useState<BatteryData | null>(null)

  const fetchBatteryData = async () => {
    try {
      const data: BatteryData = await invoke("battery_data")
      console.log(data)
      setBatteryData(data)
    } catch (err) {
      console.log(err)
      setBatteryData(null)
    }
  }

  // biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
  useEffect(() => {
    fetchBatteryData()
  }, [])

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

  if (!batteryData) {
    return <span className="text-center">Loading...</span>
  }

  return (
    <div id="home" className="p-8">
      <div className="flex flex-row justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-4xl font-bold tracking-tight mb-2 text-gray-900 dark:text-white">
            Energi
          </h1>
          <p className="mb-8 text-gray-800 dark:text-gray-200">
            Quick stats about your battery
          </p>
        </div>
        <div>
          <ActionButton onClick={() => fetchBatteryData()}>
            Refresh
          </ActionButton>
        </div>
      </div>

      <h2 className="text-xl font-bold mb-4 dark:text-gray-100">
        Bird's Eye View of Your Battery
      </h2>
      <div id="stats" className="flex flex-row flex-wrap gap-4">
        <ReadingTile
          label="Charge Level:"
          value={`${batteryData.percentage.toFixed(1)}%`}
        />
        <ReadingTile
          label="Battery Status:"
          value={formatBatteryState(batteryData.state)}
        />
        {batteryData.state === BatteryState.Discharging && (
          <ReadingTile
            label="Estimated Time Remaining:"
            value={`${prettyMs(batteryData.timeToEmpty * 1000, { compact: true })}`}
          />
        )}
        {batteryData.state === BatteryState.Charging && (
          <ReadingTile
            label="Estimated Time To Full:"
            value={`${prettyMs(batteryData.timeToFull * 1000, { compact: true })}`}
          />
        )}
        <ReadingTile
          label="Battery Health:"
          value={`${batteryData.health.toFixed(2)}%`}
        />
        <ReadingTile
          label="Battery Discharging Rate:"
          value={`${batteryData.rate.toFixed(2)} W`}
        />
        <ReadingTile
          label="Charge Cycles:"
          value={`${batteryData.chargeCycles}`}
        />
        <ReadingTile
          label="Voltage:"
          value={`${batteryData.voltage.toFixed(2)}V`}
        />
        <ReadingTile label="Model:" value={batteryData.model} />
        <ReadingTile label="Battery Vendor:" value={batteryData.vendor} />
        <ReadingTile label="Serial Number:" value={batteryData.serial} />
      </div>

      <div className="mt-8 rounded-lg p-6">
        <h2 className="text-2xl font-bold ps-4 mb-8 dark:text-gray-100">
          Battery Percentage Graph
        </h2>

        <LineGraph data={batteryData.historyPercentage} formatUnit="%" />
      </div>
      <div className="mt-8 rounded-lg p-6">
        <h2 className="text-2xl font-bold ps-4 mb-8 dark:text-gray-100">
          Battery Energy Consumption Graph
        </h2>
        <LineGraph data={batteryData.historyRate} formatUnit=" W" />
      </div>
    </div>
  )
}

// Dummy battery data
// {
//   percentage: 59,
//   state: BatteryState.Charging,
//   health: 90,
//   temperature: 25,
//   rate: 99,
//   timeToEmpty: 3600,
//   timeToFull: 7200,
//   serial: "ABCD",
//   vendor: "HP",
//   model: "HP",
//   voltage: 9.1,
//   chargeCycles: 300,
//   historyPercentage: [
//     [1745040899, 63],
//     [1745041865, 78],
//     [1745041942, 79],
//     [1745042020, 80],
//     [1745042101, 81],
//     [1745042183, 82],
//     [1745042267, 83],
//     [1745045518, 75],
//     [1745045659, 74],
//     [1745045828, 73],
//     [1745045969, 72],
//     [1745046110, 71],
//     [1745046251, 70],
//     [1745046392, 69],
//     [1745046561, 68],
//     [1745046646, 67],
//     [1745047266, 60],
//     [1745047351, 59],
//     [1745047436, 58],
//     [1745047595, 57],
//     [1745047661, 56],
//   ],
//   historyRate: [
//     [1687891200, 12],
//     [1687894800, 15],
//     [1687898400, 10],
//     [1687902000, 8],
//     [1687905600, 11],
//   ],
// }
