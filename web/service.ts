import { OrderbookAggregatorClient } from "./proto/orderbook_grpc_web_pb";

const { BookRequest, Summary, Level } = require("./proto/orderbook_pb.js");

const streamPair = (pair: string, on_data: any) => {
  // @ts-ignore
  const cli = new OrderbookAggregatorClient(STREAMING_URL, null, {});
  const request = new BookRequest();
  request.setPair(pair);
  const stream = cli.bookSummary(request, {});
  stream.on("data", (summary: typeof Summary) => {
    on_data({
      spread: summary.getSpread(),
      asks: summary.getAsksList().map(level_record("asks")),
      bids: summary.getBidsList().map(level_record("bids")),
    });
  });
  return stream;
};

const level_record =
  (side: string) =>
  (level: typeof Level): Record<string, string | number> => ({
    exchange: level.getExchange(),
    price: level.getPrice(),
    amount: level.getAmount(),
    group: `${level.getExchange()} - ${side}`,
    side,
  });

export default streamPair;
