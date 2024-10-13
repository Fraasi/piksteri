<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  // import { join } from "@tauri-apps/api/path"
  import { convertFileSrc } from "@tauri-apps/api/core";
  import Siema from "siema";
  import { onMount } from "svelte";

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
      // selectedFolder = (await open({
      //   multiple: false,
      //   directory: true,
      //   // recursive: true, // https://github.com/tauri-apps/tauri/issues/4851
      // })) as string;

      const files: string[] = await invoke("get_files", {
        path: selectedFolder || "/d/Downs",
      });
      images = files
        .filter((file) => isImage(file))
        .map((file) => convertFileSrc(file));

      numOfImages = images.length;
      runAfterFolderSelect()
      // if (numOfImages > 0) {
      //   clearInterval(intervalId);
      //   startSlideShow();
      // }
    } catch (e) {
      console.error(e);
    }
  }

  // function startSlideShow() {
  //   // start with random image & go in order from there
  //   let index = Math.floor(Math.random() * numOfImages);
  //   currentImgUrl = images[index];
  //   intervalId = setInterval(() => {
  //     if (index >= numOfImages) index = 0;
  //     currentImgUrl = images[index];
  //     // console.log("image url:", currentImgUrl);
  //     index++;
  //   }, 5000);
  // }

  // function toggleSildeShow() {
  //   console.log("toggleSildeShow", isRunning);
  //   if (isRunning) {
  //     isRunning = false;
  //     clearInterval(intervalId);
  //   } else {
  //     isRunning = true;
  //     startSlideShow();
  //   }
  // }

  // siema
  let slider: Siema;
  let prev: () => void;
  let next: () => void;
  let select = 0;

  let runAfterFolderSelect = () => {
    slider = new Siema({
      selector: ".siema",
      duration: 200,
      easing: "ease-in-out",
      perPage: 1,
      startIndex: 0,
      draggable: true,
      multipleDrag: true,
      threshold: 20,
      loop: true,
      rtl: false,
      onInit: () => {},
      onChange: () => {},
    }); //end Siema constructor
    prev = () => {
      slider.prev();
      console.log('prev')
      if (select > 0) {
        select--;
      }
    };
    next = () => {
      slider.next();
      console.log('next')

      if (select >= 0) {
        select++;
      }
    };
    console.log('onMount: ', slider)
    setInterval(() => slider.next(), 1000)
  }; //end onMount

</script>

<div class="container">
  <button type="button" on:click={selectFolder}>open Directory</button>
  <button on:click={prev}> prev </button>
  <button on:click={next}> next </button>
  {#if selectedFolder}
    <p>Selected folder: {selectedFolder} - Number of images: {numOfImages}</p>
  {/if}
  <div class="siema">
    {#if images.length > 0}
    <div class="siema__sub">
      {#each images as image}
        <div class="siema__slide">
          <img src={image} alt="{image} file is not an image" />
        </div>
      {/each}
    </div>
  {/if}
  </div>
  <!-- {#if currentImgUrl}
    <img
      on:click={toggleSildeShow}
      id="current-img"
      src={currentImgUrl}
      alt="{currentImgUrl} file is not an image"
    /> -->
  <!-- {:else}
    <p>No folder selected</p> -->
  <!-- {/if} -->
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .container {
    margin: 0;
    width: 100vw;
    height: 100vh;
  }

  .siema__slide > img{
    /* width: 100%; */
    height: 100%;
    object-fit: contain;
  }
</style>
