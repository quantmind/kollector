// plot does not have @types yet, this is just an hack to make typescript happy
declare module "@observablehq/plot" {
  let plot: any;
  let areaX: any;
  let lineX: any;
  let legend: any;
  let barX: any;
  let barY: any;
  let rectX: any;
  let rectY: any;
  let ruleX: any;
  let ruleY: any;
  let dot: any;

  export { plot, areaX, lineX, dot, legend, barX, barY, ruleX, ruleY, rectX, rectY };
}
