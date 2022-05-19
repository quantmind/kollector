import Container from "@mui/material/Container";
import Typography from "@mui/material/Typography";
import React from "react";
import Stream from "./service";

const Main = () => {
  return (
    <Container>
      <Typography component="h1" align="center" pt={2}>
        Orderbook stream
      </Typography>
      <Inner />
    </Container>
  );
};

const Inner = () => {
  const [data, setData] = React.useState<any>({
    spread: 0,
    bids: [],
    asks: [],
  });

  const on_data = (msg: any) => {
    setData({
      spread: msg.getSpread(),
      asks: msg.getAsksList(),
      bids: msg.getBidsList(),
    });
  };

  React.useEffect(() => {
    const cli = new Stream();
    cli.start(on_data);
  });

  return (
    <>
      <Typography component="p" align="center" pt={2}>
        Spread: {data.spread}
      </Typography>
      <Typography component="p" align="center" pt={1}>
        Bids: {data.bids.length}
      </Typography>
      <Typography component="p" align="center" pt={1}>
        Asks: {data.asks.length}
      </Typography>
    </>
  );
};

export default Main;
