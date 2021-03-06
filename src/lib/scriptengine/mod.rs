//! # scriptengine
//!
//! Facade for all different non OS scripts.
//!

pub(crate) mod generic_script;
mod os_script;
mod rsscript;
pub(crate) mod script_utils;
mod shebang_script;
mod shell_to_batch;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::types::Task;

#[derive(Debug, Clone, PartialEq)]
/// The currently supported engine types
pub(crate) enum EngineType {
    /// OS native script
    OS,
    /// Rust language
    Rust,
    /// shell to windows batch conversion
    Shell2Batch,
    /// Generic script runner
    Generic,
    /// Shebang script runner
    Shebang,
    /// Unsupported type
    Unsupported,
}

pub(crate) fn get_engine_type(task: &Task) -> EngineType {
    match task.script {
        None => EngineType::Unsupported,
        Some(ref script) => {
            match task.script_runner {
                Some(ref script_runner) => {
                    debug!("Checking script runner: {}", script_runner);

                    if script_runner == "@rust" {
                        debug!("Rust script detected.");
                        EngineType::Rust
                    } else if script_runner == "@shell" {
                        debug!("Shell to batch detected.");
                        EngineType::Shell2Batch
                    } else if task.script_extension.is_some() {
                        // if both script runner and extension is defined, we use generic script runner
                        debug!("Generic script detected.");
                        EngineType::Generic
                    } else {
                        // use default OS extension with custom runner
                        debug!("OS script with custom runner detected.");
                        EngineType::OS
                    }
                }
                None => {
                    // if no runner specified, try to extract it from script content
                    if shebang_script::is_shebang_exists(script) {
                        EngineType::Shebang
                    } else {
                        EngineType::OS
                    }
                }
            }
        }
    }
}

pub(crate) fn invoke(task: &Task, cli_arguments: &Vec<String>) -> bool {
    let engine_type = get_engine_type(&task);
    let validate = !task.is_force();

    match engine_type {
        EngineType::OS => {
            let script = task.script.as_ref().unwrap();
            let runner = task.script_runner.clone();
            os_script::execute(script, runner, cli_arguments, validate);

            true
        }
        EngineType::Rust => {
            let script = task.script.as_ref().unwrap();
            rsscript::execute(script, cli_arguments, validate);

            true
        }
        EngineType::Shell2Batch => {
            let script = task.script.as_ref().unwrap();
            shell_to_batch::execute(script, cli_arguments, validate);

            true
        }
        EngineType::Generic => {
            let script = task.script.as_ref().unwrap();
            let runner = task.script_runner.clone().unwrap();
            let extension = task.script_extension.clone().unwrap();
            generic_script::execute(script, runner, extension, None, cli_arguments, validate);

            true
        }
        EngineType::Shebang => {
            let script = task.script.as_ref().unwrap();
            let extension = task.script_extension.clone();
            shebang_script::execute(script, &extension, cli_arguments, validate);

            true
        }
        EngineType::Unsupported => false,
    }
}
