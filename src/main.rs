extern crate core;

mod procmon_enums;

use procmon_enums::{Column, Relation, Action};

use std::process::Command;
use std::{env, thread};

const CURR_PATH: &str = ".";
const PROCMON_DIR: &str = ".\\procmon\\";
const EVENT_FILE: &str = ".\\procmon\\events.pml";
const XML_EVENT_FILE: &str = ".\\procmon\\events.xml";
const PROCMON_LIB: &str = ".\\procmon\\Procmon.ps1; ";

type ProcmonResult = Result<(), String>;

fn main() {
    let filters = create_pid_filter(&vec!["chrome.exe", "firefox.exe"]);
    match start_procmon(&filters, EVENT_FILE) {
        Ok(_) => {
            println!("Procmon started");
        }
        Err(res) => {
            panic!("Procmon failed to start\n {}", res);
        }
    };
    thread::sleep(std::time::Duration::from_secs(5));

    match stop_procmon() {
        Ok(_) => {
            println!("Procmon stopped");
        }
        Err(_) => {
            panic!("Procmon failed to stop");
        }
    };

    match convert_to_xml(&filters, EVENT_FILE, XML_EVENT_FILE) {
        Ok(_) => {
            println!("Converted to XML");
        }
        Err(res) => {
            panic!("Failed to convert to XML\n {}", res);
        }
    }
}


fn create_pid_filter(pids: &Vec<&str>) -> Vec<String> {
    pids.iter()
     .map(
         |pid|
             create_filter(Column::ProcessId, Relation::Is, &pid, Action::Include)
     ).collect::<Vec<String>>()
}


fn create_filter(column: Column, relation: Relation, value: &str, action: Action) -> String {
    format!(
        "@(New-ProcmonFilter -Column '{}' -Relation {} -Value {} -Action {}) + ",
        column.value(), relation.value(), value, action.value()
    )
}


fn clear_procmon_filters() -> ProcmonResult {
    execute_procmon_command("Clear-ProcmonFiltersRegistry").unwrap()
}


fn start_procmon(filters: &Vec<String>, of_name: &str) -> ProcmonResult {
    clear_procmon_filters()?;
    let command = format!(
        "$({fs} (Get-DefaultProcmonFilters)) | Start-Procmon -ProcmonDir {pd} -EventFile {ef}",
        fs = filters.join(""), pd = PROCMON_DIR, ef = of_name
    );
    execute_procmon_command(&command).unwrap()
}

fn stop_procmon() -> ProcmonResult {
    let command = format!(
        "Stop-Procmon -ProcmonDir {pd}",
        pd = PROCMON_DIR
    );
    execute_procmon_command(&command).unwrap()
}


fn convert_to_xml(filters: &Vec<String>, if_name: &str, of_name: &str) -> ProcmonResult {
    clear_procmon_filters()?;
    let command = format!(
        "$({fs} (Get-DefaultProcmonFilters)) | ConvertTo-ProcmonXML -ProcmonDir {pd} -EventFile {ef} -OutputFile {of}",
        fs = filters.join(""), pd = PROCMON_DIR, ef = if_name, of = of_name
    );
    execute_procmon_command(&command).unwrap()
}


// executes given command in powershell and returns the output
fn execute_procmon_command(cmd: &str) -> std::io::Result<ProcmonResult> {
    let output = Command::new("powershell")
        .arg(PROCMON_LIB)
        .arg(cmd)
        .output()?;

    Ok(
        match output.status.success() {
            true => Ok(()),
            false => Err(String::from_utf8_lossy(&output.stderr).to_string()),
        }
    )
}