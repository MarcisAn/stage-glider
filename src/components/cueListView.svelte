<script lang="ts">
  export let opened_project: OpenedProject;
  import type {
    Cue,
    CueList,
    OpenedProject,
    Action,
    ModuleInfo,
  } from "../main";
  import CueView from "./cueView.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import * as Select from "../lib/components/ui/select";
  import "../styles/cueList.scss";
  import { Separator } from "../lib/components/ui/separator";
  import { Button } from "../lib/components/ui/button";
  import * as Dialog from "../lib/components/ui/dialog";

  let cue_to_run_next: number = 0;

  let list = opened_project.cue_lists[0];

  function go_on_list() {
    invoke("go_on_cue_list", {
      listId: 0,
      cueId: cue_to_run_next,
      projectData: JSON.stringify(opened_project),
    }).then((data) => {
      console.log(data);
      //check if already not on the last cue
      if (opened_project.cue_lists[0].cues.length - 1 != cue_to_run_next) {
        //increment the cue
        cue_to_run_next += 1;
      } else {
        cue_to_run_next = 0;
      }
    });
  }
  let all_actions: ModuleInfo = [];
  invoke("get_all_actions").then((data) => {
    console.log(data);
    //@ts-ignore
    all_actions = JSON.parse(data);
  });
</script>

<div class="cue-list-view">
  <h1>Cue list <strong>{list.name}</strong></h1>
  <Separator class="my-2 white" />
  <Button on:click={go_on_list} variant="outline">GO</Button>
  {#each list.cues as cue, index}
    <CueView {cue} {index} is_next_cue={cue_to_run_next == index} />
  {/each}
  <Dialog.Root>
    <Dialog.Trigger class="add-cue-btn">Add cue</Dialog.Trigger>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>Add action</Dialog.Title>
        <Dialog.Description>
          {#each all_actions as module_info}
            <div class="flex-col flex gap-4">
              {module_info.name}
              {#each module_info.actions as action}
                <Button>{action.name}</Button>
              {/each}
            </div>
          {/each}
        </Dialog.Description>
      </Dialog.Header>
    </Dialog.Content>
  </Dialog.Root>
</div>
