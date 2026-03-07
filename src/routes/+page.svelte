<script>
  // @ts-nocheck
  import debounce from "debounce";
  import { browser } from "$app/environment";
  import { invoke } from "@tauri-apps/api/core";
  import { store, update, updatePlayer } from "shared/match-data.svelte";
  import operations from "shared/operations";
  import { socket } from "shared/socket";
  import { onMount } from "svelte";
  import { get } from "svelte/store";


  let one;
  let two;
  let websocket;

  /**
   * There are three moving parts to this. 
   * 1. We have the "authority" layer, which holds the state of match data
   * this includes player's names, their score count, their absolver style, etc. 
   * This comes down to us through a websocket that the application has set up.
   * 2. There's a local "copy" of the authority layer's state. And the local copy updates
   * when the authority layer updates its state. 
   * 3. Any call to update the data has to be made through the authority layer. 
  */

  onMount(() => {
    websocket = new WebSocket("ws://localhost:9001");

    websocket.onmessage = (message) => {
      console.log("we're updating, you fuck", { data : message.data });
      update(JSON.parse(message.data));
    };

  });

  /** Update the stored data by calling into Rust */
  const refresh = async (updated) => {    
    await invoke("update", { data : $store })
  }

  const chungify = () => {
    const chungus = "CHUNGUSS";
    const current = get(store);

    current.player1.name = chungus;

    update(current);
    refresh(current);
  }

  const write = (index, name) => {
    console.log("updating name", { index, name })
    operations.update.name(store, index, name);
    refresh(get(store));
  }

</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <h1> Player One: </h1>
    <input bind:value={one} oninput={debounce(() => write(0, one), 400)} />
  </div>

  <div class="row">
    <h1> Player Two: </h1>
    <input bind:value={two} oninput={debounce(() => write(1, two), 400)} />
  </div>


  <div>
    <h1> The whole fuckin' thing brother: </h1>
    <div class="massive">{JSON.stringify($store)}</div>
  </div>

</main>

<style>
.massive { 
  font-size: 2rem;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
