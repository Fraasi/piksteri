<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  // import { join } from "@tauri-apps/api/path"
  import { convertFileSrc } from "@tauri-apps/api/core";

  let numOfImages = 0;
  let selectedFolder = "";
  let currentImgUrl = "";
  let intervalId: number;
  let images: string[] = [];
  let isRunning = false;


  const extensions = new Set([
    "bmp",
    "djvu",
    "gif",
    "icns",
    "ico",
    "jpeg",
    "jpg",
    "pict",
    "png",
    "psd",
    "raw",
    "rgb",
    "rgba",
    "svg",
    "tif",
    "tiff",
    "webp",
  ]);

  function isImage(filePath: string): boolean {
    return extensions.has(filePath.split(".").at(-1)?.toLowerCase() as string);
  }

  // when using `"withGlobalTauri": true`, you may use: const { open } = window.__TAURI__.dialog
  // Open a dialog
  async function selectFolder() {
    try {
      selectedFolder = (await open({
        multiple: false,
        directory: true,
        // recursive: true, // https://github.com/tauri-apps/tauri/issues/4851
      })) as string;

      const files: string[] = await invoke("get_files", { path: selectedFolder });
      images = files
        .filter((file) => isImage(file))
        .map((file) => convertFileSrc(file));

      numOfImages = images.length;
      if (numOfImages > 0) {
        clearInterval(intervalId);
        startSlideShow();
      }
    } catch (e) {
      console.error(e);
    }
  }

  function startSlideShow() {
    // start with random image & go in order from there
    let index = Math.floor(Math.random() * numOfImages);
    currentImgUrl = images[index];
    intervalId = setInterval(() => {
      if (index >= numOfImages) index = 0;
      currentImgUrl = images[index];
      // console.log("image url:", currentImgUrl);
      index++;
    }, 5000);
  }

  function toggleSildeShow() {
    console.log("toggleSildeShow", isRunning);
    if (isRunning) {
      isRunning = false;
      clearInterval(intervalId);
    } else {
      isRunning = true;
      startSlideShow();
    }
  }
</script>

<div class="container">
  <button type="button" on:click={selectFolder}>open Directory</button>
  {#if selectedFolder}
    <p>Selected folder: {selectedFolder} - Number of images: {numOfImages}</p>
  {/if}
  {#if currentImgUrl}
    <img
      on:click={toggleSildeShow}
      id="current-img"
      src={currentImgUrl}
      alt="{currentImgUrl} file is not an image"
    />
  {:else}
    <p>No folder selected</p>
  {/if}
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #f6f6f6;
    background-color: #2f2f2f;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    width: 100vw;
    height: 100vh;
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
    outline: none;
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #0f0f0f69;
  }

  img#current-img {
    /* display: block; */
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
</style>
