import { plot } from "@observablehq/plot";
import React from "react";
import { Chart, PlotReactProps, Size } from "./Chart";

type ObservablePlot = typeof plot;
type ObservablePlotOptions = any;

const PlotReact = (
  props: PlotReactProps<ObservablePlot, ObservablePlotOptions>
) => {
  const createChart = (
    el: HTMLElement,
    options: ObservablePlotOptions,
    size: Size
  ): ObservablePlot => {
    const chart = plot({ ...options, ...size });
    el.append(chart);
    return chart;
  };

  const destroy = (chart: ObservablePlot) => {
    chart.remove();
  };

  return (
    <Chart
      {...props}
      createChart={createChart}
      resizeChart={createChart}
      destroy={destroy}
    />
  );
};

export default PlotReact;
