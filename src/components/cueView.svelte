<script lang="ts">
  export let cue: Cue;
  export let index: number;
  export let is_next_cue: boolean;
  import type { Cue, CueList, OpenedProject } from "../main";
</script>

<div class={is_next_cue ? "next-cue" : "cue"}>
  <span class="cue_index">{index + 1}</span>
  <span class="cue_name">{cue.name}</span>
</div>

<style lang="scss">
  .next-cue {
    background: rgb(255, 255, 255);
    background: linear-gradient(
      0deg,
      rgba(255, 255, 255, 0) 40%,
      rgba(119, 223, 255, 0.493) 80%,
      rgb(85, 173, 255) 100%
    );
    @extend .cue;
  }
  .cue {
    background-color: var(--foreground);
    display: flex;
    gap: 20px;
    font-size: 1.3rem;
  }
</style>
