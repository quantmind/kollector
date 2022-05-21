/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.orderbook.BookRequest', null, global);
goog.exportSymbol('proto.orderbook.Empty', null, global);
goog.exportSymbol('proto.orderbook.Level', null, global);
goog.exportSymbol('proto.orderbook.ServiceInfo', null, global);
goog.exportSymbol('proto.orderbook.Summary', null, global);

/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.orderbook.Empty = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.orderbook.Empty, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.orderbook.Empty.displayName = 'proto.orderbook.Empty';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.orderbook.Empty.prototype.toObject = function(opt_includeInstance) {
  return proto.orderbook.Empty.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.orderbook.Empty} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.Empty.toObject = function(includeInstance, msg) {
  var f, obj = {
    pair: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.orderbook.Empty}
 */
proto.orderbook.Empty.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.orderbook.Empty;
  return proto.orderbook.Empty.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.orderbook.Empty} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.orderbook.Empty}
 */
proto.orderbook.Empty.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setPair(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.orderbook.Empty.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.orderbook.Empty.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.orderbook.Empty} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.Empty.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPair();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string pair = 1;
 * @return {string}
 */
proto.orderbook.Empty.prototype.getPair = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.orderbook.Empty.prototype.setPair = function(value) {
  jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.orderbook.BookRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.orderbook.BookRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.orderbook.BookRequest.displayName = 'proto.orderbook.BookRequest';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.orderbook.BookRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.orderbook.BookRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.orderbook.BookRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.BookRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    pair: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.orderbook.BookRequest}
 */
proto.orderbook.BookRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.orderbook.BookRequest;
  return proto.orderbook.BookRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.orderbook.BookRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.orderbook.BookRequest}
 */
proto.orderbook.BookRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setPair(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.orderbook.BookRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.orderbook.BookRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.orderbook.BookRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.BookRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPair();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string pair = 1;
 * @return {string}
 */
proto.orderbook.BookRequest.prototype.getPair = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.orderbook.BookRequest.prototype.setPair = function(value) {
  jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.orderbook.ServiceInfo = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.orderbook.ServiceInfo.repeatedFields_, null);
};
goog.inherits(proto.orderbook.ServiceInfo, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.orderbook.ServiceInfo.displayName = 'proto.orderbook.ServiceInfo';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.orderbook.ServiceInfo.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.orderbook.ServiceInfo.prototype.toObject = function(opt_includeInstance) {
  return proto.orderbook.ServiceInfo.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.orderbook.ServiceInfo} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.ServiceInfo.toObject = function(includeInstance, msg) {
  var f, obj = {
    pairsList: jspb.Message.getRepeatedField(msg, 1),
    maxDepth: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.orderbook.ServiceInfo}
 */
proto.orderbook.ServiceInfo.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.orderbook.ServiceInfo;
  return proto.orderbook.ServiceInfo.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.orderbook.ServiceInfo} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.orderbook.ServiceInfo}
 */
proto.orderbook.ServiceInfo.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.addPairs(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readUint64());
      msg.setMaxDepth(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.orderbook.ServiceInfo.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.orderbook.ServiceInfo.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.orderbook.ServiceInfo} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.ServiceInfo.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPairsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      1,
      f
    );
  }
  f = message.getMaxDepth();
  if (f !== 0) {
    writer.writeUint64(
      2,
      f
    );
  }
};


/**
 * repeated string pairs = 1;
 * @return {!Array<string>}
 */
proto.orderbook.ServiceInfo.prototype.getPairsList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 1));
};


/** @param {!Array<string>} value */
proto.orderbook.ServiceInfo.prototype.setPairsList = function(value) {
  jspb.Message.setField(this, 1, value || []);
};


/**
 * @param {!string} value
 * @param {number=} opt_index
 */
proto.orderbook.ServiceInfo.prototype.addPairs = function(value, opt_index) {
  jspb.Message.addToRepeatedField(this, 1, value, opt_index);
};


proto.orderbook.ServiceInfo.prototype.clearPairsList = function() {
  this.setPairsList([]);
};


/**
 * optional uint64 max_depth = 2;
 * @return {number}
 */
proto.orderbook.ServiceInfo.prototype.getMaxDepth = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/** @param {number} value */
proto.orderbook.ServiceInfo.prototype.setMaxDepth = function(value) {
  jspb.Message.setProto3IntField(this, 2, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.orderbook.Summary = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.orderbook.Summary.repeatedFields_, null);
};
goog.inherits(proto.orderbook.Summary, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.orderbook.Summary.displayName = 'proto.orderbook.Summary';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.orderbook.Summary.repeatedFields_ = [2,3];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.orderbook.Summary.prototype.toObject = function(opt_includeInstance) {
  return proto.orderbook.Summary.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.orderbook.Summary} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.Summary.toObject = function(includeInstance, msg) {
  var f, obj = {
    spread: +jspb.Message.getFieldWithDefault(msg, 1, 0.0),
    bidsList: jspb.Message.toObjectList(msg.getBidsList(),
    proto.orderbook.Level.toObject, includeInstance),
    asksList: jspb.Message.toObjectList(msg.getAsksList(),
    proto.orderbook.Level.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.orderbook.Summary}
 */
proto.orderbook.Summary.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.orderbook.Summary;
  return proto.orderbook.Summary.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.orderbook.Summary} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.orderbook.Summary}
 */
proto.orderbook.Summary.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setSpread(value);
      break;
    case 2:
      var value = new proto.orderbook.Level;
      reader.readMessage(value,proto.orderbook.Level.deserializeBinaryFromReader);
      msg.addBids(value);
      break;
    case 3:
      var value = new proto.orderbook.Level;
      reader.readMessage(value,proto.orderbook.Level.deserializeBinaryFromReader);
      msg.addAsks(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.orderbook.Summary.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.orderbook.Summary.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.orderbook.Summary} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.Summary.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSpread();
  if (f !== 0.0) {
    writer.writeDouble(
      1,
      f
    );
  }
  f = message.getBidsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.orderbook.Level.serializeBinaryToWriter
    );
  }
  f = message.getAsksList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      3,
      f,
      proto.orderbook.Level.serializeBinaryToWriter
    );
  }
};


/**
 * optional double spread = 1;
 * @return {number}
 */
proto.orderbook.Summary.prototype.getSpread = function() {
  return /** @type {number} */ (+jspb.Message.getFieldWithDefault(this, 1, 0.0));
};


/** @param {number} value */
proto.orderbook.Summary.prototype.setSpread = function(value) {
  jspb.Message.setProto3FloatField(this, 1, value);
};


/**
 * repeated Level bids = 2;
 * @return {!Array<!proto.orderbook.Level>}
 */
proto.orderbook.Summary.prototype.getBidsList = function() {
  return /** @type{!Array<!proto.orderbook.Level>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.orderbook.Level, 2));
};


/** @param {!Array<!proto.orderbook.Level>} value */
proto.orderbook.Summary.prototype.setBidsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.orderbook.Level=} opt_value
 * @param {number=} opt_index
 * @return {!proto.orderbook.Level}
 */
proto.orderbook.Summary.prototype.addBids = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.orderbook.Level, opt_index);
};


proto.orderbook.Summary.prototype.clearBidsList = function() {
  this.setBidsList([]);
};


/**
 * repeated Level asks = 3;
 * @return {!Array<!proto.orderbook.Level>}
 */
proto.orderbook.Summary.prototype.getAsksList = function() {
  return /** @type{!Array<!proto.orderbook.Level>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.orderbook.Level, 3));
};


/** @param {!Array<!proto.orderbook.Level>} value */
proto.orderbook.Summary.prototype.setAsksList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 3, value);
};


/**
 * @param {!proto.orderbook.Level=} opt_value
 * @param {number=} opt_index
 * @return {!proto.orderbook.Level}
 */
proto.orderbook.Summary.prototype.addAsks = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 3, opt_value, proto.orderbook.Level, opt_index);
};


proto.orderbook.Summary.prototype.clearAsksList = function() {
  this.setAsksList([]);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.orderbook.Level = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.orderbook.Level, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.orderbook.Level.displayName = 'proto.orderbook.Level';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.orderbook.Level.prototype.toObject = function(opt_includeInstance) {
  return proto.orderbook.Level.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.orderbook.Level} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.Level.toObject = function(includeInstance, msg) {
  var f, obj = {
    exchange: jspb.Message.getFieldWithDefault(msg, 1, ""),
    price: +jspb.Message.getFieldWithDefault(msg, 2, 0.0),
    amount: +jspb.Message.getFieldWithDefault(msg, 3, 0.0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.orderbook.Level}
 */
proto.orderbook.Level.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.orderbook.Level;
  return proto.orderbook.Level.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.orderbook.Level} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.orderbook.Level}
 */
proto.orderbook.Level.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setExchange(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setPrice(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setAmount(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.orderbook.Level.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.orderbook.Level.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.orderbook.Level} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.orderbook.Level.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getExchange();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getPrice();
  if (f !== 0.0) {
    writer.writeDouble(
      2,
      f
    );
  }
  f = message.getAmount();
  if (f !== 0.0) {
    writer.writeDouble(
      3,
      f
    );
  }
};


/**
 * optional string exchange = 1;
 * @return {string}
 */
proto.orderbook.Level.prototype.getExchange = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.orderbook.Level.prototype.setExchange = function(value) {
  jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional double price = 2;
 * @return {number}
 */
proto.orderbook.Level.prototype.getPrice = function() {
  return /** @type {number} */ (+jspb.Message.getFieldWithDefault(this, 2, 0.0));
};


/** @param {number} value */
proto.orderbook.Level.prototype.setPrice = function(value) {
  jspb.Message.setProto3FloatField(this, 2, value);
};


/**
 * optional double amount = 3;
 * @return {number}
 */
proto.orderbook.Level.prototype.getAmount = function() {
  return /** @type {number} */ (+jspb.Message.getFieldWithDefault(this, 3, 0.0));
};


/** @param {number} value */
proto.orderbook.Level.prototype.setAmount = function(value) {
  jspb.Message.setProto3FloatField(this, 3, value);
};


goog.object.extend(exports, proto.orderbook);
