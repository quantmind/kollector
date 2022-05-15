var searchIndex = JSON.parse('{\
"common":{"doc":"the common crate defines all the common structs, functions …","t":[13,13,13,13,3,3,13,6,3,3,4,13,13,18,18,13,3,3,4,13,3,3,18,18,13,18,18,8,16,18,18,4,13,3,13,3,3,13,11,12,12,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,12,11,11,11,12,12,12,11,11,12,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,12,12,12,12],"n":["Ask","BadContent","BadStatus","Bid","Book","BookSnapshot","BookSnapshot","CfgBuilder","Context","Error","ErrorKind","Exit","Failure","HAS_PAYLOAD","HAS_PAYLOAD","Heartbeat","HttpClient","InconsistentBook","InnerMessage","InvalidContentMatch","L2","L2Iterator","METHOD","METHOD","Network","PATH","PATH","Request","Response","SIGNED","SIGNED","Side","WsConnected","WsConsumer","WsDisconnected","WsInfo","WsPayload","WsPayload","as_result","asks","asset","at","best","best_price","best_price_f32","bids","book","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cfg","clear","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","context","create_config","depth_volume","description","deserialize","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","get_max_depth","get_side","get_side_mut","get_total_depth","get_url","info","init_logging","into","into","into","into","into","into","into","into","into","into","into","into","into","into","into_iter","is_consistent","is_empty","is_empty","iter","kind","len","logger","name","name","name","name","new","new","new","new","new","new","next","price_at","receiver","request","run","send","sender","sender","sequence","set","set_str","timestamp","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","trim","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","unsigned","update","url","url","url","value","volume_and_imbalance","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","worse","worse_price","worse_price_f32","wrap_result","write","0","0","0","0"],"q":["common","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","common::InnerMessage","","",""],"d":["","","","","A <code>Book</code> represents a level 2 order book data structure","","Orderbook snapshot","","Worker Context","","","clean exit","exit with failure","","","heartbeat message","","","Internal message enum","","One side of a Level 2 Order book","","","","","","","","","","","","websocket message","Websocket consumer","websocket disconnect","","Websocket payload","websocket payload","Convert the Book into a Result","level 2 ask prices &amp; sizes","asset name","","Returns the (price, volume) tuple at the best price if …","","","level 2 bid prices &amp; sizes","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","configuration","Clear the book","","","","","","","","","","","","","","","","","","","","","Calculate the cumulative volume up to a given depth","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Initialise slog","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","Check if the book is consistent","Returns true if this orderbook side is empty","","","","Returns the depth levels","logging","name of the worker","","gateway name","","","","","","","Create a new websocket consumer","","","Use this to receive messages from another worker","","","","Use this to send messages to another worker","","","Set a new price/volume into the book side","Set a new price/volume into the book side","last timestamp the book was updated","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Update the order side with a vector of price/volume tuples","full url given a path","","websocket url","message payload","Calculate the total volume up to <code>depth</code> and the book …","","","","","","","","","","","","","","","worse price in the orderbook side","","","","schedule a write into the websocket","","","",""],"i":[1,2,2,1,0,0,3,0,0,0,0,3,3,4,4,3,0,0,0,2,0,0,4,4,2,4,4,0,4,4,4,0,3,0,3,0,0,3,5,5,5,6,6,6,6,5,7,8,1,9,2,10,6,11,5,12,13,14,7,3,15,8,1,9,2,10,6,11,5,12,13,14,7,3,15,12,5,2,6,5,12,13,14,7,3,15,2,6,5,12,13,14,7,3,15,15,0,6,11,2,2,10,10,6,11,11,5,13,14,7,3,8,1,9,2,10,6,11,5,12,13,14,7,3,15,5,5,5,5,15,15,0,8,1,9,2,10,6,11,5,12,13,14,7,3,15,9,5,6,5,6,10,6,12,12,13,14,7,8,10,11,5,12,15,9,6,12,8,15,12,12,15,7,6,6,5,2,6,5,12,13,14,7,3,15,10,11,0,8,1,9,2,10,6,11,5,12,13,14,7,3,15,8,1,9,2,10,6,11,5,12,13,14,7,3,15,8,1,9,2,10,6,11,5,12,13,14,7,3,15,8,6,8,13,14,14,5,8,1,9,2,10,6,11,5,12,13,14,7,3,15,6,6,6,0,15,16,17,18,19],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0],["str",0]],["result",4,[["inconsistentbook",3]]]],null,null,[[["",0],["usize",0]],["option",4]],[[["",0]],["option",4]],[[["",0]],["option",4,[["decimal",3]]]],[[["",0]],["option",4,[["f32",0]]]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],null,[[["",0]]],[[["",0]],["errorkind",4]],[[["",0]],["l2",3]],[[["",0]],["book",3]],[[["",0]],["context",3]],[[["",0]],["wsinfo",3]],[[["",0]],["wspayload",3]],[[["",0]],["booksnapshot",3]],[[["",0]],["innermessage",4]],[[["",0]],["wsconsumer",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],null,[[],["cfgbuilder",6]],[[["",0],["usize",0],["f32",0]],["f32",0]],[[["",0]],["str",0]],[[],["result",4]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["usize",0]],[[["",0],["side",4]],["l2",3]],[[["",0],["side",4]],["l2",3]],[[["",0]],["usize",0]],[[["",0]],["str",0]],[[["",0]],["wsinfo",3]],[[["config",3]],["result",6,[["logger",3]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["bool",0]],[[["",0]],["bool",0]],[[["",0]],["bool",0]],[[["",0]],["l2iterator",3]],[[["",0]],["errorkind",4]],[[["",0]],["usize",0]],null,null,null,null,null,[[["str",0]]],[[["method",3],["url",3],["statuscode",3],["errorkind",4],["string",3],["string",3]]],[[["str",0],["str",0]]],[[["str",0]]],[[["str",0],["option",4,[["config",3]]]]],[[["context",3],["str",0]]],[[["",0]],["option",4]],[[["",0],["usize",0]],["option",4,[["decimal",3]]]],null,[[["",0],["option",4,[["logger",3]]]]],[[["",0]]],[[["",0]]],null,null,null,[[["",0],["decimal",3],["decimal",3]]],[[["",0],["str",0],["str",0]]],null,[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[["string",3],["usize",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0],["",0],["url",3],["str",0]],["requestbuilder",3]],[[["",0]]],[[["",0],["str",0]],["string",3]],null,null,null,[[["",0],["usize",0],["f32",0]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["option",4]],[[["",0]],["option",4,[["decimal",3]]]],[[["",0]],["option",4,[["f32",0]]]],[[["context",3],["result",6]]],[[["",0],["serialize",8]]],null,null,null,null],"p":[[4,"Side"],[4,"ErrorKind"],[4,"InnerMessage"],[8,"Request"],[3,"Book"],[3,"L2"],[3,"BookSnapshot"],[3,"HttpClient"],[3,"L2Iterator"],[3,"Error"],[3,"InconsistentBook"],[3,"Context"],[3,"WsInfo"],[3,"WsPayload"],[3,"WsConsumer"],[13,"WsConnected"],[13,"WsDisconnected"],[13,"WsPayload"],[13,"BookSnapshot"]]},\
"gateways":{"doc":"Connect and map remote exchanges messages","t":[3,3,13,8,4,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,10,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,10,11,11,12],"n":["Binance","Bitstamp","Book","Gateway","WsUpdate","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","name","name","name","new","new","on_book_snapshot","on_book_snapshot","on_book_snapshot","on_websocket_message","on_websocket_message","on_websocket_message","setup","setup","subscribe","subscribe","subscribe","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","unsubscribe","unsubscribe","unsubscribe","vzip","vzip","vzip","ws_consumer","ws_consumer","ws_consumer","0"],"q":["gateways","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","gateways::WsUpdate"],"d":["Binance Gateway","Bitstamp Gateway","","A Gateway trait","A websocket update","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","Create a new Binance gateway","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,0,0,2,3,1,2,3,1,2,3,1,2,3,1,4,2,3,2,3,2,4,4,4,2,3,4,4,4,2,3,2,3,1,2,3,1,2,3,1,4,2,3,2,3,1,4,2,3,5],"f":[null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[]],[[]],[[]],[[]],[[]],[[]],[[["",0]],["str",0]],[[["",0]],["str",0]],[[["",0]],["str",0]],[[["context",3],["usize",0]]],[[["context",3],["usize",0]]],[[["",0],["booksnapshot",3]],["option",4,[["book",3]]]],[[["",0],["booksnapshot",3]],["option",4,[["book",3]]]],[[["",0],["booksnapshot",3]],["option",4,[["book",3]]]],[[["",0],["value",4]],["option",4,[["wsupdate",4]]]],[[["",0],["value",4]],["option",4,[["wsupdate",4]]]],[[["",0],["value",4]],["option",4,[["wsupdate",4]]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]]],[[["",0]]],[[["",0]]],[[]],[[]],[[]],[[["",0]],["wsconsumer",3]],[[["",0]],["wsconsumer",3]],[[["",0]],["wsconsumer",3]],null],"p":[[4,"WsUpdate"],[3,"Binance"],[3,"Bitstamp"],[8,"Gateway"],[13,"Book"]]},\
"service":{"doc":"","t":[3,11,11,12,11,11,11,11,12,11,11,11,11,11,11,11,11],"n":["Kollector","borrow","borrow_mut","context","from","handle_ctrlc","into","into_request","max_depth","new","run","spawn_gateway","spawn_grpc","try_from","try_into","type_id","vzip"],"q":["service","","","","","","","","","","","","","","","",""],"d":["The Kollector is the main the main service","","","","Returns the argument unchanged.","Add Ctrl-C handler","Calls <code>U::from(self)</code>.","","","Create a new Kollector service","Main coroutine","Spawn a gateway","Spawn the grpc server","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[null,[[["",0]],["",0]],[[["",0]],["",0]],null,[[]],[[["",0]]],[[]],[[],["request",3]],null,[[["usize",0]]],[[["",0],["str",0]]],[[["",0],["box",3,[["gateway",8]]]]],[[["",0]]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]]],"p":[[3,"Kollector"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};