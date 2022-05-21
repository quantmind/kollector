import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import FormControl from "@mui/material/FormControl";
import FormHelperText from "@mui/material/FormHelperText";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import * as Plot from "@observablehq/plot";
import { extent, format } from "d3";
import React from "react";
import { useSearchParams } from "react-router-dom";
import GrpcService from "./service";
import { PlotReact } from "./Viz";

const formatImbalance = format(".3f");

interface Info {
  pairs: string[];
}

interface Level {
  exchange: string;
  price: number;
  amount: number;
  side: string;
  group: string;
}

interface Summary {
  spread: number;
  asks: Level[];
  bids: Level[];
}

const Main = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [info, setInfo] = React.useState<Info | null>(null);
  const grpc = new GrpcService();
  const getInfo = info ? true : false;

  const handleChange = (e: any) => {
    setSearchParams({ pair: e.target.value });
  };

  React.useEffect(() => {
    grpc.info(setInfo);
  }, [getInfo]);

  if (!info) return null;

  const pair = searchParams.get("pair") || info.pairs[0];

  return (
    <Container>
      <Typography component="h1" align="center" pt={2}>
        Orderbook stream
      </Typography>
      <Typography component="div" pt={1} align="center">
        <FormControl sx={{ m: 1, minWidth: 120 }}>
          <Select value={pair} label="Age" onChange={handleChange}>
            {info.pairs.map((name: string) => (
              <MenuItem value={name} key={name}>
                {name}
              </MenuItem>
            ))}
          </Select>
          <FormHelperText>Crypto pair</FormHelperText>
        </FormControl>
      </Typography>
      <Inner pair={pair} grpc={grpc} />
    </Container>
  );
};

const Inner = ({ pair, grpc }: { pair: string; grpc: GrpcService }) => {
  const theme = useTheme();
  const plot = React.useRef<any>(null);
  const [summary, setData] = React.useState<Summary | null>(null);

  const storeChart = (chart: any) => {
    plot.current = chart;
  };

  React.useEffect(() => {
    const stream = grpc.streamPair(pair, setData);
    return () => {
      stream.cancel();
    };
  }, [pair]);

  if (!summary) return null;
  const plotData = summary.bids.concat(summary.asks);
  const [askAmount, asks] = cumsum(summary.asks);
  const [bidAmount, bids] = cumsum(summary.bids);
  const imbalance = formatImbalance(
    (bidAmount - askAmount) / (bidAmount + askAmount)
  );
  plotData.sort(by_price);
  const groups = Array.from(new Set(plotData.map((d: Level) => d.group)));
  groups.sort();

  // plot options
  const options = {
    title: "Orderbooks",
    marginLeft: 100,
    marginTop: 50,
    marginBottom: 50,
    style: {
      background: theme.palette.background.default,
      color: theme.palette.text.primary,
      fontSize: 14,
    },
    x: {
      grid: true,
    },
    y: {
      grid: true,
      domain: price_domain(summary.asks, summary.bids),
    },
    color: {
      type: "ordinal",
      scheme: "spectral",
      legend: true,
    },
    marks: [
      Plot.ruleY(plotData, {
        x: "amount",
        y: "price",
        stroke: "group",
      }),
      Plot.dot(plotData, { x: "amount", y: "price", fill: "group", r: 6 }),
      Plot.areaX(asks, {
        y: "price",
        x2: "amount",
        fill: "red",
        curve: "step",
        fillOpacity: 0.2,
      }),
      Plot.lineX(asks, {
        y: "price",
        x: "amount",
        stroke: "red",
        curve: "step",
        strokeOpacity: 0.5,
      }),
      Plot.areaX(bids, {
        y: "price",
        x2: "amount",
        fill: "green",
        curve: "step",
        fillOpacity: 0.2,
      }),
      Plot.lineX(bids, {
        y: "price",
        x: "amount",
        stroke: "green",
        curve: "step",
        strokeOpacity: 0.5,
      }),
    ],
  };

  return (
    <>
      <Typography component="p" align="center" pt={2}>
        Spread: {summary.spread} - Book imbalance: {imbalance}
      </Typography>
      <Box pt={2}>
        <PlotReact
          height="70%"
          maxHeight={800}
          options={options}
          onCreate={storeChart}
        />
      </Box>
    </>
  );
};

const price = (l: Level) => l.price;
const by_price = (l1: Level, l2: Level) => (l1.price > l2.price ? 1 : -1);

// keep the plt stable rather than bouncing around
const price_domain = (asks: Level[], bids: Level[]) => {
  const [a1, a2] = extent(asks, price) as [number, number];
  const [b1, b2] = extent(bids, price) as [number, number];
  const mid = (a1 + b2) / 2;
  const d = 1.05 * Math.max(a2 - mid, mid - b1);
  return [mid - d, mid + d];
};

const cumsum = (levels: Level[]): [number, any] => {
  let amount = 0;
  let depth = levels.map((l: Level) => {
    amount += l.amount;
    return { amount, price: l.price };
  });
  return [amount, depth];
};

export default Main;
