import { createApp } from "vue";
import { createPinia } from "pinia";
import ServerConnection from "./services/connection/ServerConnection";

import App from "./App.vue";
import router from "./router";

import "./assets/main.css";

const app = createApp(App);

app.use(createPinia());
app.use(router);

// dalang runs on port 2821
let connection = new ServerConnection(
  "wss://" + window.location.hostname + ":2821"
);

app.provide("con", connection);

app.mount("#app");
