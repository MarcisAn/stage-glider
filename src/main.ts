import "./app.pcss";
import "./styles.css";
import App from "./App.svelte";

export interface OpenedProject {
  name: string;
  cue_lists: CueList[];
}
export interface CueList {
  name: string;
  cues: Cue[];
}
export interface Cue {
  name: string;
  action: Action;
}
export interface Action {
  module: String;
  action: String;
  params: String;
}
export interface ModuleInfo {
  name: String;
  discription: String;
  actions: Action[];
}
export interface Action {
  name: String;
}

const app = new App({
  //@ts-ignore
  target: document.getElementById("app"),
});

export default app;
