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
  <div id="main">
    <img id="logo" src="../assets/logo.svg" />
    <div id="card">
      <h3>Authenticate</h3>
      <div id="inputs">
        <input placeholder="Username" id="username" type="text" />
        <input placeholder="Password" id="password" type="password" />
      </div>

      <div id="buttons">
        <div @onclick="null" id="register">Register</div>
        <div @onclick="null" id="login">Login</div>
      </div>
    </div>
  </div>

  <img draggable="false" id="gunungan" src="../assets/gunungan.svg" />
</template>

<style scoped>
#logo {
  width: 3rem;

  opacity: 0.7;
  filter: invert(1);
}

#main {
  height: 100vh;
  width: 100vw;
  position: absolute;
  top: 0;
  left: 0;

  display: flex;
  flex-direction: column;

  justify-content: center;
  align-items: center;

  gap: 4rem;
  z-index: 5;
}

#main > #card {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 1.5rem;

  width: 25vmax;

  font-family: "Nunito", sans-serif;

  padding: 2rem;
  background: #eeeeee;
  box-shadow: 0px 0px 1.5rem 0px #00000040;
  border-radius: 1.5rem;
}

#main > #card > h3 {
  font-weight: 800;
  font-size: 1.5rem;
  text-align: center;
  margin: 0;
  color: #3a3a3a;
}

#main > #card > #inputs {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

#main > #card > #inputs > input {
  padding: 1rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
}

#main > #card > #inputs > input:focus-visible {
  outline: none;
}

#main > #card > #inputs > input::placeholder {
  color: #c2c2c2;
}

#main > #card > #buttons {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;

  color: #3080c9;
}

#main > #card > #buttons > * {
  cursor: pointer;
}

#main > #card > #buttons > #login {
  border-radius: 0.75rem;
  padding: 1rem 2.5rem;
  background-color: #3080c9;
  color: white;
}

#main > #card > #buttons > #login:hover {
  background-color: #4c95db;
}

#gunungan {
  position: absolute;

  left: 50vw;
  bottom: -10vmin;

  height: 90vmin;

  transform: rotate(-20deg);
  opacity: 5%;

  z-index: 0;
}
</style>
