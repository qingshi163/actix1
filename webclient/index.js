
(function (){
    let socket = null;
    function log(msg) {
        let node = document.createElement("div");
        let content = document.createTextNode(msg);
        node.appendChild(content);
        document.querySelector("#log").appendChild(node);
    }
    function connect() {
        disconnect();
        let wsUri = "ws://" + window.location.host + "/ws";
        socket = new WebSocket(wsUri);
        console.log("Connecting to: " + wsUri);
        socket.addEventListener("open", (e) => {
            log("Connected");
        });
        socket.addEventListener("message", (e) => {
            log("RECV: " + e.data);
        });
        socket.addEventListener("close", (e) => {
            log("Disconnected");
        });
    }
    function disconnect() {
        if (socket == null) return;
        socket.close();
        socket = null;
    }
    function send(data) {
        if (socket == null) {
            log("No connection");
            return;
        }
        socket.send(data);
    }
    document.querySelector("#connect").addEventListener("click", (e) => {
        connect();
    });
    document.querySelector("#disconnect").addEventListener("click", (e) => {
        disconnect();
    });
    document.querySelector("#send").addEventListener("click", (e) => {
        let text = document.querySelector("#text").value;
        send(text);
    });
})();