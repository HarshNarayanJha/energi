import {
  CartesianGrid,
  Line,
  LineChart,
  ReferenceLine,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts"

interface LineGraphProps {
  data: Array<[number, number]>
  formatUnit: string
}

export default function LineGraph({ data, formatUnit }: LineGraphProps) {
  return (
    <LineChart
      className="mx-auto max-w-full h-fit max-h-fit"
      margin={{ top: 5, right: 30, left: 30, bottom: 120 }}
      width={1500}
      height={700}
      data={data.map(([time, value]) => ({
        time: new Date(time * 1000).toLocaleTimeString(),
        value,
      }))}
    >
      <CartesianGrid strokeDasharray="3 3" strokeOpacity={0.2} />
      <XAxis dataKey="time" angle={-45} tickMargin={35} />
      <YAxis
        dataKey="value"
        tickFormatter={(value: number, _index: number) =>
          `${value.toFixed(1)}${formatUnit}`
        }
      />
      <Tooltip />
      <Line
        type="monotoneX"
        dataKey="value"
        stroke="#8884d8"
        strokeWidth={3}
        animationBegin={1000}
      />
      <ReferenceLine
        y={data.reduce(([at, av], [t, v]) => [t + at, v + av])[1] / data.length}
        label="Average"
        stroke="brown"
        strokeDasharray="3 3"
      />
    </LineChart>
  )
}
