import Box from "@mui/material/Box";
import debounce from "lodash.debounce";
import React from "react";

export interface Size {
  width: number;
  height: number;
}

export interface PlotReactProps<T, O> {
  options: O;
  height?: string | number;
  width?: number;
  maxHeight?: number;
  wait?: number;
  onDelete?: (chart: T) => void;
  onCreate?: (chart: T) => void;
}

export interface InnerPlotReactProps<T, O> extends PlotReactProps<T, O> {
  createChart: (el: HTMLElement, options: O, size: Size) => T;
  resizeChart: (el: HTMLElement, options: O, size: Size, chart: T) => void;
  destroy: (chart: T) => void;
}

// Generic Charting React Element
//
// Supports
// - uPlot
// - Observable Plot
//
//  Allow to resize window and set maximum height and or aspect ratio
export const Chart = <T, O>(props: InnerPlotReactProps<T, O>) => {
  const {
    createChart,
    resizeChart,
    destroy,
    height = "70%",
    width = 0,
    wait = 100,
    maxHeight = 10000,
    onCreate,
    onDelete,
    options,
  } = props;
  const resizable = height.constructor.name == "String";
  const heightStr = height as string;
  const heightPct = resizable
    ? +heightStr.substring(0, heightStr.length - 1)
    : 0;
  const chartRef = React.useRef<T | null>(null);
  const targetRef = React.useRef<HTMLDivElement>(null);
  const [size, setSize] = React.useState<Size>({
    width,
    height: resizable ? 0 : (height as number),
  });

  const plotSize = (): Size | undefined => {
    const s: Size = { height: 0, width: targetRef.current?.offsetWidth || 0 };
    if (size.width !== s.width) {
      s.height = Math.min(Math.round(0.01 * heightPct * s.width), maxHeight);
      return s;
    }
  };
  // Create the chart
  const create = () => {
    chartRef.current = createChart(
      targetRef.current as HTMLElement,
      options,
      resizable ? plotSize() || size : size
    );

    if (onCreate) onCreate(chartRef.current);
  };

  const doDestroy = (chart: T | null) => {
    if (chart) {
      if (onDelete) onDelete(chart as T);
      destroy(chart as T);
      chartRef.current = null;
    }
  };

  //
  React.useEffect(() => {
    if (resizable) {
      const newSize = plotSize();
      if (newSize) {
        setSize(newSize);
        return;
      }
    }
    create();
    const current = chartRef.current;

    let handleResize: any = null;

    if (resizable) {
      handleResize = debounce(() => {
        const newSize = plotSize();
        if (newSize) {
          setSize(newSize);
        }
      }, wait);
      window.addEventListener("resize", handleResize);
    }

    return () => {
      if (handleResize) {
        window.removeEventListener("resize", handleResize);
      }
      doDestroy(current);
    };
  }, [size, options]);

  return (
    <Box
      sx={{ width: "100%", height: `${size.height}px` }}
      ref={targetRef}
    ></Box>
  );
};
