<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CueListView from "./components/cueListView.svelte";
  let path: string = "";
  let isProjectOpened = false;
  import type { Cue, CueList, OpenedProject } from "./main";

  let opened_project: OpenedProject;

  function open_project() {
    invoke("load_project", { path: path }).then((data) => {
      console.log(data);
      //@ts-ignore
      opened_project = JSON.parse(data);
      isProjectOpened = true;
    });
  }
</script>

{#if isProjectOpened}
  <main class="container">
    <CueListView {opened_project} />
    <div class="side-view"></div>
  </main>
{:else}
  <input type="text" bind:value={path} />
  <button on:click={open_project}>Open</button>
{/if}
