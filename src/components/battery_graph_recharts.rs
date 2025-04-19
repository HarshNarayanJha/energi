use dioxus::prelude::*;

#[component]
pub fn BatteryGraphRecharts() -> Element {
    rsx! {
        // div { dangerous_inner_html: generate_test_react() }
        div { id: "container" }
        script { r#type: "module",

            r#"
            import * as d3 from "https://cdn.jsdelivr.net/npm/d3@7/+esm";

            const width = 1500;
            const height = 800;
            const marginTop = 20;
            const marginRight = 20;
            const marginBottom = 30;
            const marginLeft = 40;

            const data = [
                {{ date: new Date("2023-01-01T00:00"), level: 90 }},
                {{ date: new Date("2023-01-01T06:00"), level: 75 }},
                {{ date: new Date("2023-01-01T12:00"), level: 50 }},
                {{ date: new Date("2023-01-01T18:00"), level: 30 }},
                {{ date: new Date("2023-01-02T00:00"), level: 15 }},
                {{ date: new Date("2023-01-02T06:00"), level: 80 }},
                {{ date: new Date("2023-01-02T12:00"), level: 65 }},
                {{ date: new Date("2023-01-02T18:00"), level: 45 }},
                {{ date: new Date("2023-01-03T00:00"), level: 25 }},
                {{ date: new Date("2023-01-03T06:00"), level: 10 }},
                {{ date: new Date("2023-01-03T12:00"), level: 95 }},
            ];

            // Set up scales
            const x = d3.scaleTime()
                .domain(d3.extent(data, d => d.date))
                .range([marginLeft, width - marginRight])
                .nice();

            const y = d3.scaleLinear()
                .domain([0, 100])
                .range([height - marginBottom, marginTop])
                .nice();

            const line = d3.line()
                .x(d => x(d.date))
                .y(d => y(d.level))
                .curve(d3.curveLinear);

            const svg = d3.create("svg")
                .attr("width", width)
                .attr("height", height)
                .attr("viewBox", [0, 0, width, height])
                .attr("style", "max-width: 100%; height: auto; font: 10px sans-serif; margin: auto;");

            // Add gradient for battery level
            const gradient = svg.append("defs")
                .append("linearGradient")
                .attr("id", "battery-gradient")
                .attr("gradientUnits", "userSpaceOnUse")
                .attr("x1", 0)
                .attr("y1", y(0))
                .attr("x2", 0)
                .attr("y2", y(100));

            gradient.append("stop")
                .attr("offset", "0%")
                .attr("stop-color", "red");

            gradient.append("stop")
                .attr("offset", "50%")
                .attr("stop-color", "orange");

            gradient.append("stop")
                .attr("offset", "100%")
                .attr("stop-color", "green");

            // Add gridlines
            svg.append("g")
                .attr("stroke", "currentColor")
                .attr("stroke-opacity", 0.1)
                .call(g => g.append("g")
                    .selectAll("line")
                    .data(y.ticks())
                    .join("line")
                    .attr("y1", d => y(d))
                    .attr("y2", d => y(d))
                    .attr("x1", marginLeft)
                    .attr("x2", width - marginRight))
                .call(g => g.append("g")
                    .selectAll("line")
                    .data(x.ticks())
                    .join("line")
                    .attr("x1", d => x(d))
                    .attr("x2", d => x(d))
                    .attr("y1", marginTop)
                    .attr("y2", height - marginBottom));

            // Add x-axis
            svg.append("g")
                .attr("transform", `translate(0,${{height - marginBottom}})`)
                .call(d3.axisBottom(x).ticks(width / 80).tickSizeOuter(0))
                .call(g => g.append("text")
                    .attr("x", width - marginRight)
                    .attr("y", -10)
                    .attr("fill", "currentColor")
                    .attr("text-anchor", "end")
                    .text("Time →"));

            // Add y-axis
            svg.append("g")
                .attr("transform", `translate(${{marginLeft}},0)`)
                .call(d3.axisLeft(y)
                    .ticks(null, "%")
                    .tickFormat(d => `${{d}}%`))
                .call(g => g.select(".domain").remove())
                .call(g => g.select(".tick:last-of-type text").clone()
                    .attr("x", 3)
                    .attr("text-anchor", "start")
                    .attr("font-weight", "bold")
                    .text("Battery %"));

            // Add battery level area
            svg.append("path")
                .datum(data)
                .attr("fill", "url(#battery-gradient)")
                .attr("fill-opacity", 0.2)
                .attr("stroke", "none")
                .attr("d", d3.area()
                    .x(d => x(d.date))
                    .y0(y(0))
                    .y1(d => y(d.level))
                    .curve(d3.curveLinear));

            // Add battery line
            svg.append("path")
                .datum(data)
                .attr("fill", "none")
                .attr("stroke", "url(#battery-gradient)")
                .attr("stroke-width", 2.5)
                .attr("stroke-linejoin", "round")
                .attr("stroke-linecap", "round")
                .attr("d", line);

            // Add data points
            svg.append("g")
                .selectAll("circle")
                .data(data)
                .join("circle")
                .attr("cx", d => x(d.date))
                .attr("cy", d => y(d.level))
                .attr("r", 4)
                .attr("fill", d => d.level > 70 ? "green" : d.level > 30 ? "orange" : "red")
                .attr("stroke", "white");

            // Add title
            svg.append("text")
                .attr("x", width / 2)
                .attr("y", marginTop / 2)
                .attr("text-anchor", "middle")
                .attr("font-weight", "bold")
                .attr("font-size", "14px")
                .text("Battery Level History");

            container.append(svg.node());
            "#
        }
        div { "See what's above?" }
    }
}
