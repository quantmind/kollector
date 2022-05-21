import { OrderbookAggregatorClient } from "./proto/orderbook_grpc_web_pb";

const {
  BookRequest,
  Summary,
  Level,
  Empty,
} = require("./proto/orderbook_pb.js");

class GrpcService {
  cli: OrderbookAggregatorClient;

  constructor() {
    // @ts-ignore
    this.cli = new OrderbookAggregatorClient(STREAMING_URL, null, {});
  }

  info(onData: any) {
    const request = new Empty();
    this.cli.info(request, {}, (_: any, response: any) => {
      onData({
        pairs: response.getPairsList(),
      });
    });
  }

  streamPair(pair: string, onData: any) {
    const request = new BookRequest();
    request.setPair(pair);
    const stream = this.cli.bookSummary(request, {});
    stream.on("data", (summary: typeof Summary) => {
      onData({
        spread: summary.getSpread(),
        asks: summary.getAsksList().map(level_record("asks")),
        bids: summary.getBidsList().map(level_record("bids")),
      });
    });
    return stream;
  }
}

const level_record =
  (side: string) =>
  (level: typeof Level): Record<string, string | number> => ({
    exchange: level.getExchange(),
    price: level.getPrice(),
    amount: level.getAmount(),
    group: `${level.getExchange()} - ${side}`,
    side,
  });

export default GrpcService;
