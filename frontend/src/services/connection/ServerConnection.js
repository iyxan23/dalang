import { encode, decode } from "msgpack-lite";

export default class ServerConnection {
  #listeners = {};
  #ws = null;
  #events = {
    open: new Event("open"),
    close: new Event("close"),
  };

  #url = undefined;

  connect() {
    if (this.connected()) {
      console.warn("Websocket already connected.");
      return;
    }

    this.#ws = new WebSocket(this.#url, "dalang");

    this.#ws.onmessage = this.messageListener;
    this.#ws.onopen = () => dispatchEvent(this.#events.open);

    this.#ws.onclose = () => {
      dispatchEvent(this.#events.close);
      this.#ws = null;
    };

    this.#ws.onerror = (event) => {
      console.error(event);
    };
  }

  connected() {
    return this.#ws != null;
  }

  constructor(dalangServer) {
    this.#url = dalangServer;
  }

  #messageListener(event) {
    // decode the data
    const { o: opcode, c: category, d: data } = decode(event.data);

    if (this.#listeners[[opcode, category]] != undefined) {
      for (const listener of this.#listeners[[opcode, category]]) {
        listener.callback(data);
      }

      // remove any onetime listeners
      this.#listeners[[opcode, category]].filter((elem) => !elem.onetime);
    }
  }

  // Registers a server message listener if the opcode and the category matches the one
  // being sent by the server.
  //
  // Will execute the given callback with one argument: data: any?. Which is the data
  // of the message from the server. It may be null.
  registerMsgListener(callback, opcode, category, onetime = false) {
    if (!this.connected()) {
      console.warn("Websocket not connected");
      return;
    }

    if (this.#listeners[[opcode, category]] == undefined) {
      this.#listeners[[opcode, category]] = [{ callback, onetime }];
    } else {
      this.#listeners[[opcode, category]].push({ callback, onetime });
    }
  }

  // Sends a message to the server. The data is optional, depending on the opcode used
  send(opcode, category, data) {
    if (!this.connected()) {
      console.warn("Websocket not connected");
      return;
    }

    // encode the data
    const obj = encode({ o: opcode, c: category, d: data });

    this.#ws.send(obj);
  }

  close() {
    if (!this.connected()) {
      console.warn(
        "Trying to close WebSocket connection when it's not even connected"
      );

      return;
    }

    this.#ws.close();
  }
}
