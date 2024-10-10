<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core"
  import { open } from "@tauri-apps/plugin-dialog"
  import { join } from "@tauri-apps/api/path"
  import { convertFileSrc } from "@tauri-apps/api/core"
  import { readDir } from "@tauri-apps/plugin-fs"

  let numOfImages = 0
  let folder = ""
  let currentImgUrl = "/d/Pics/2020-12-28_2143.png"
  let intervalId: number

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
  ])

  function isImage(filePath: string): boolean {
    return extensions.has(filePath.split(".").at(-1)?.toLowerCase() as string)
  }

  // when using `"withGlobalTauri": true`, you may use: const { open } = window.__TAURI__.dialog
  // Open a dialog
  async function openDir() {
    try {
      folder = (await open({
        multiple: false,
        directory: true,
        // recursive: true, // https://github.com/tauri-apps/tauri/issues/4851
      })) as string
      // console.log(dir)
      const files = await readDir(folder)
      const images: string[] = []

      for (let file of files) {
        if (file.isFile && isImage(file.name)) {
          const path = await join(folder, file.name)
          const pathConverted = convertFileSrc(path)
          images.push(pathConverted)
        }
      }

      numOfImages = images.length
      // start with random image & go in order from there
      let num = Math.floor(Math.random() * numOfImages)
      clearInterval(intervalId)
      intervalId = setInterval(() => {
        if (num >= numOfImages) num = 0
        currentImgUrl = images[num]
        console.log("image url:", currentImgUrl)
        num++
      }, 5000)
    } catch (e) {
      console.error(e)
    }
  }
</script>

<div class="container">
  <button type="button" on:click={openDir}>open Directory</button>
  {#if folder}
    <p>Selected folder: {folder} - Number of images: {numOfImages}</p>
  {/if}
  {#if currentImgUrl}
    <img
      id="current-img"
      src={currentImgUrl}
      alt="{currentImgUrl} file is not an image"
    />
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
