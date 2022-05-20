import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import FormControl from "@mui/material/FormControl";
import FormHelperText from "@mui/material/FormHelperText";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { useTheme } from "@mui/material/styles";
import Typography from "@mui/material/Typography";
import * as Plot from "@observablehq/plot";
import React from "react";
import { useSearchParams } from "react-router-dom";
import streamPair from "./service";
import { PlotReact } from "./Viz";

const pairs = ["btcusdt", "ethusdt", "ethbtc"];

const Main = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const pair = searchParams.get("pair") || pairs[0];

  const handleChange = (e: any) => {
    setSearchParams({ pair: e.target.value });
  };

  return (
    <Container>
      <Typography component="h1" align="center" pt={2}>
        Orderbook stream
      </Typography>
      <Typography component="div" pt={1} align="center">
        <FormControl sx={{ m: 1, minWidth: 120 }}>
          <Select value={pair} label="Age" onChange={handleChange}>
            {pairs.map((name: string) => (
              <MenuItem value={name} key={name}>
                {name}
              </MenuItem>
            ))}
          </Select>
          <FormHelperText>Crypto pair</FormHelperText>
        </FormControl>
      </Typography>
      <Inner pair={pair} />
    </Container>
  );
};

interface Level {
  exchange: string;
  price: number;
  amount: number;
  side: string;
  group: string;
}

const Inner = ({ pair }: { pair: string }) => {
  const theme = useTheme();
  const plot = React.useRef<any>(null);
  const [data, setData] = React.useState<any>();

  const storeChart = (chart: any) => {
    plot.current = chart;
  };

  React.useEffect(() => {
    const stream = streamPair(pair, setData);
    return () => {
      stream.cancel();
    };
  }, [pair]);

  if (!data) return null;

  const plotData = data.bids.concat(data.asks);
  plotData.sort(by_price);
  const groups = Array.from(new Set(plotData.map((d: Level) => d.group)));
  groups.sort();

  // plot options
  const options = {
    title: "Risk",
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
    },
    color: {
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
    ],
  };

  return (
    <>
      <Typography component="p" align="center" pt={2}>
        Spread: {data.spread}
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

const by_price = (l1: Level, l2: Level) => l1.price > l2.price;

export default Main;
