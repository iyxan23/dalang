<script setup>
import { RouterView } from "vue-router";
import { inject, onMounted } from "vue";

const connection = inject("con");

onMounted(() => {
  console.debug("connecting");

  if (!connection.connected()) {
    connection.connect();
  }

  connection.addEventListener("open", () => {
    console.debug("connection opened");
    // do things
  });
});
</script>

<template>
  <RouterView />
</template>

<style>
@import url("https://fonts.googleapis.com/css2?family=Nunito:wght@400;700;800&display=swap");

html,
body {
  padding: 0;
  margin: 0;
  background-color: #0b3048;

  overflow: hidden;
}
</style>
