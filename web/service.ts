import { OrderbookAggregatorClient } from "./proto/orderbook_grpc_web_pb";

const { Empty } = require("./proto/orderbook_pb.js");

class Stream {
  cli: OrderbookAggregatorClient;

  constructor() {
    this.cli = new OrderbookAggregatorClient("http://localhost:90", null, {});
  }

  start(on_data: any) {
    const request = new Empty();
    const stream = this.cli.bookSummary(request, {});
    stream.on("data", on_data);
  }
}

export default Stream;
