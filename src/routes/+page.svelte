<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { appDataDir, join, homeDir } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import  { readDir }  from '@tauri-apps/plugin-fs'
  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }

  // when using `"withGlobalTauri": true`, you may use: const { open } = window.__TAURI__.dialog;
  // Open a dialog
  async function openDir() {
    try {
      const dir = await open({
        multiple: false,
        directory: true,
      });
      console.log(dir)
      const files = await readDir(dir as string)
      console.log(files.length)
      for (let file of files) {
        // check also if image
        if (file.isFile) console.log(file.name)
      }


      // Prints file path or URI
    } catch (e) {
      console.error(e);
    }
  }
  let imgUrl = "";
  async function addUrl() {
    // hack: Load small files via fs functions as binary
    // https://github.com/tauri-apps/tauri/issues/3725#issuecomment-2147476960

    const appDataDirPath = await appDataDir();
    console.log("appDataDirPath", appDataDirPath);

    // cant read from d, 'cos is in windows side?
    // const filePath = await join('d', "Pics/2020-12-28_2143.png");
    const filePath = await join(await homeDir(), 'Code/github/piksteri/static', 'svelte.svg')
    console.log("filePath", filePath);

    const u = convertFileSrc(filePath);
    console.log("u", u);
    imgUrl = u
    // imgUrl = '/svelte.svg'
  }
</script>

<div class="container">
  <h1>Tauri!</h1>

  <button type="button" on:click={openDir}>open Directory</button>
  <button type="button" on:click={addUrl}>add image</button>
  {#if imgUrl}
    <img src={imgUrl} style="width: 100px" alt="pic" />
  {/if}
  <div class="row">
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <p>Click on the Tauri and SvelteKit logos to learn more.</p>

  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  <p>{greetMsg}</p>
</div>

<style>
  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
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

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  input,
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

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
