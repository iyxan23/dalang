export default class ServerConnection {
  #listeners = [];
  #ws = null;
  #events = {
    open: new Event("open"),
    close: new Event("close"),
  };

  constructor(dalangServer) {
    this.#ws = new WebSocket(dalangServer, "dalang");

    this.#ws.onmessage = this.messageListener;
    this.#ws.onopen = () => dispatchEvent(this.#events.open);
    this.#ws.onclose = () => dispatchEvent(this.#events.close);
    this.#ws.onerror = (event) => {
      console.error(event);
    };
  }

  #messageListener(event) {
    // todo
  }

  // Registers a server message listener if the opcode and the category matches the one
  // being sent by the server.
  //
  // Will execute the given callback with one argument: data: any?. Which is the data
  // of the message from the server.
  registerMsgListener(callback, opcode, category) {
    this.listeners.push({ callback, opcode, category });
  }
}
