import { encode, decode } from "msgpack-lite";

export default class ServerConnection extends EventTarget {
  #listeners = {};
  #ws = null;
  #url = undefined;

  connect() {
    if (this.connected()) {
      console.warn("Websocket already connected.");
      return;
    }

    this.#ws = new WebSocket(this.#url, "dalang");

    this.#ws.onmessage = this.#messageListener;
    this.#ws.onopen = () => {
      this.dispatchEvent(new Event("open"));
    };

    this.#ws.onclose = () => {
      console.debug("connection closed");

      this.dispatchEvent(new Event("close"));
      this.#ws = null;
    };

    this.#ws.onerror = (event) => {
      console.error("connection error", event);
      this.#ws = null;
    };
  }

  connected() {
    if (this.#ws === null) return false;
    if (this.#ws.readyState === 1) return true;

    return false;
  }

  constructor(dalangServer) {
    super();
    this.#url = dalangServer;
  }

  async #messageListener(event) {
    if (!(event.data instanceof Blob)) {
      console.warn("Got a text message, ignoring: ", event.data);
      return;
    }

    const msg = new Uint8Array(await event.data.arrayBuffer());
    console.debug("retrieved", event);
    console.debug("data", msg);

    // decode the data
    const [opcode, category, data] = decode(msg);

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
    const obj = encode([opcode | (category << 16), data]);

    console.debug("sending", obj);
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
