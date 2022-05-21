/**
 * @fileoverview gRPC-Web generated client stub for orderbook
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.orderbook = require('./orderbook_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.orderbook.OrderbookAggregatorClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?grpc.web.ClientOptions} options
 * @constructor
 * @struct
 * @final
 */
proto.orderbook.OrderbookAggregatorPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options.format = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.orderbook.BookRequest,
 *   !proto.orderbook.Summary>}
 */
const methodDescriptor_OrderbookAggregator_BookSummary = new grpc.web.MethodDescriptor(
  '/orderbook.OrderbookAggregator/BookSummary',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.orderbook.BookRequest,
  proto.orderbook.Summary,
  /**
   * @param {!proto.orderbook.BookRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.orderbook.Summary.deserializeBinary
);


/**
 * @param {!proto.orderbook.BookRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.orderbook.Summary>}
 *     The XHR Node Readable Stream
 */
proto.orderbook.OrderbookAggregatorClient.prototype.bookSummary =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/orderbook.OrderbookAggregator/BookSummary',
      request,
      metadata || {},
      methodDescriptor_OrderbookAggregator_BookSummary);
};


/**
 * @param {!proto.orderbook.BookRequest} request The request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.orderbook.Summary>}
 *     The XHR Node Readable Stream
 */
proto.orderbook.OrderbookAggregatorPromiseClient.prototype.bookSummary =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/orderbook.OrderbookAggregator/BookSummary',
      request,
      metadata || {},
      methodDescriptor_OrderbookAggregator_BookSummary);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.orderbook.Empty,
 *   !proto.orderbook.ServiceInfo>}
 */
const methodDescriptor_OrderbookAggregator_Info = new grpc.web.MethodDescriptor(
  '/orderbook.OrderbookAggregator/Info',
  grpc.web.MethodType.UNARY,
  proto.orderbook.Empty,
  proto.orderbook.ServiceInfo,
  /**
   * @param {!proto.orderbook.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.orderbook.ServiceInfo.deserializeBinary
);


/**
 * @param {!proto.orderbook.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.RpcError, ?proto.orderbook.ServiceInfo)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.orderbook.ServiceInfo>|undefined}
 *     The XHR Node Readable Stream
 */
proto.orderbook.OrderbookAggregatorClient.prototype.info =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/orderbook.OrderbookAggregator/Info',
      request,
      metadata || {},
      methodDescriptor_OrderbookAggregator_Info,
      callback);
};


/**
 * @param {!proto.orderbook.Empty} request The
 *     request proto
 * @param {?Object<string, string>=} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.orderbook.ServiceInfo>}
 *     Promise that resolves to the response
 */
proto.orderbook.OrderbookAggregatorPromiseClient.prototype.info =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/orderbook.OrderbookAggregator/Info',
      request,
      metadata || {},
      methodDescriptor_OrderbookAggregator_Info);
};


module.exports = proto.orderbook;

