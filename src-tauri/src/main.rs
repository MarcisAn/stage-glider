#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use pyo3::types::PyTuple;
use pyo3::{prelude::*, PyNativeType};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    cue_lists: Vec<CueList>,
}
#[derive(Serialize, Deserialize, Debug)]
struct CueList {
    name: String,
    cues: Vec<Cue>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Cue {
    name: String,
    action: Action,
}
#[derive(Serialize, Deserialize, Debug)]
struct Action {
    module: String,
    action: String,
    params: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModuleInfo {
    name: String,
    description: String,
    actions: Vec<ActionInfo>,
}
#[derive(Serialize, Deserialize, Debug)]
struct ActionInfo {
    name: String,
}

#[tauri::command]
fn go_on_cue_list(listId: u32, cueId: u32, projectData: &str) {
    let project: Project = serde_json::from_str(&projectData).unwrap();

    let action = &project.cue_lists[listId as usize].cues[cueId as usize].action;

    let file_content =
        fs::read_to_string("E:/modules/".to_string() + &action.module + ".py").unwrap();

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(py, &file_content, "", "")
            .unwrap()
            .getattr(&*action.action)
            .unwrap()
            .into();

        fun.call0(py);
    });
}

#[tauri::command]
fn load_project(path: &str) -> String {
    let contents =
        fs::read_to_string("E:/test.json").expect("Should have been able to read the file");
    println!("The content of the project file:\n{contents}");
    let p: Project = serde_json::from_str(&contents).unwrap();
    println!("{:?}", p);
    contents
}
#[tauri::command]
fn get_all_actions() -> String {
    let paths = fs::read_dir("E:/modules").unwrap();

    let mut modules: Vec<ModuleInfo> = vec![];

    for path in paths {
        let file_content = fs::read_to_string(path.unwrap().path().display().to_string()).unwrap();
        println!("{}", file_content);
        Python::with_gil(|py| {
            let res: String = PyModule::from_code(py, &file_content, "", "")
                .unwrap()
                .getattr("__info")
                .unwrap()
                .call0()
                .unwrap()
                .extract()
                .unwrap();
            println!("{}", res);
            modules.push(serde_json::from_str(&res).unwrap());
        });
    }
    return serde_json::to_string(&modules).unwrap();
}

fn main() {
    //load_project("C:/Users/marcis/Desktop/test.json");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_project,
            go_on_cue_list,
            get_all_actions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
