String.prototype.isJSON = function() {
    var _self = this;
    if ( /^\s*$/.test(_self) ) return false;
    _self = _self.replace(/\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4})/g, '@');
    _self = _self.replace(/"[^"\\\n\r]*"|true|false|null|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?/g, ']');
    _self = _self.replace(/(?:^|:|,)(?:\s*\[)+/g, '');
    return (/^[\],:{}\s]*$/).test(_self);
};


var ws = new WebSocket("wss://127.0.0.1:3001");
var wsMessage = document.getElementById('ws-message');
var wsJSON = document.getElementById('ws-JSON');
var wsSubmitMessage = document.getElementById('ws-submit-message');
var wsSubmitJSON = document.getElementById('ws-submit-JSON');
var resContainer = document.getElementById('res-container');

window.addEventListener("DOMCContentLoaded", function() {

    wsSubmitMessage.onclick = function(e) {
	ws.send(wsMessage.value);
	console.log("Message sent : " + wsMessage.value);
    };

    wsSubmitJSON.onclick = function(e) {
	ws.send(wsJSON.value);
	console.log("JSON sent : ", JSON.parse(wsJSON.value));
    };

    ws.onopen = function() {
	console.log("Connection is open !");
    };

    ws.onmessage = function(e) {
	var receivedData = e.data;
	if (receivedData.isJSON())
	    console.log("JSON received : ", JSON.parse(receivedData));
	else
	    console.log("Message received : " + receivedData);
    };

    ws.onclose = function() {
	console.log("Connection is closed...");
    };
});
