<script>
export default {
  data() {
    return {
      count: 0,
    };
  },
};
</script>

<template>
  <div
    class="flex h-screen w-screen flex-col items-center justify-center gap-24 z-10"
  >
    <img class="h-20 opacity-70 invert" id="logo" src="../assets/logo.svg" />
    <div
      class="flex flex-col gap-6 rounded-3xl bg-[#eee] p-8 shadow-md z-10 w-[50vmax] lg:w-[30vmax]"
    >
      <h3 class="text-center text-2xl font-extrabold">Authenticate</h3>
      <form class="flex flex-col gap-4">
        <input
          class="text-input"
          autocomplete="username"
          placeholder="Username"
          id="username"
          type="text"
        />

        <input
          class="text-input"
          autocomplete="current-password"
          placeholder="Password"
          id="password"
          type="password"
        />
      </form>

      <div class="flex justify-between items-center">
        <div class="cursor-pointer" @onclick="null">Register</div>
        <div
          class="cursor-pointer bg-[#3080C9] text-white py-3 px-10 rounded-xl"
          @onclick="null"
        >
          Login
        </div>
      </div>
    </div>
  </div>

  <img
    class="absolute bottom-0 left-[55vw] h-[75vmin] translate-y-12 -rotate-[20deg] opacity-5 z-0"
    draggable="false"
    src="../assets/gunungan.svg"
  />
</template>
