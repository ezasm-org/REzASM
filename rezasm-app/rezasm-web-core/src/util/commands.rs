use crate::util::runtime::Runtime;
use lazy_static::lazy_static;
use rezasm_core::parser::lexer;
use rezasm_core::simulation::registry;
use rezasm_core::simulation::simulator::Simulator;

use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, RwLockWriteGuard};

type CallbackFnStr = fn(&str);
type CallbackFnI64 = fn(i64);
type CallbackFnUnit = fn();

fn _temp_str(_: &str) {}
fn _temp_i64(_: i64) {}
fn _temp_unit() {}

lazy_static! {
    static ref SIMULATOR: Arc<RwLock<Simulator>> = Arc::new(RwLock::new(Simulator::new()));
    static ref SHOULD_STOP: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref RUNTIME: Arc<RwLock<Runtime>> =
        Arc::new(RwLock::new(Runtime::new(SHOULD_STOP.clone())));
    static ref SIGNAL_ERROR: Arc<RwLock<CallbackFnStr>> = Arc::new(RwLock::new(_temp_str));
    static ref SIGNAL_PROGRAM_COMPLETION: Arc<RwLock<CallbackFnI64>> =
        Arc::new(RwLock::new(_temp_i64));
    static ref SIGNAL_TERMINATION: Arc<RwLock<CallbackFnUnit>> = Arc::new(RwLock::new(_temp_unit));
}

pub fn get_simulator() -> RwLockWriteGuard<'static, Simulator> {
    SIMULATOR.write().unwrap()
}

pub fn set_simulator(simulator: Simulator) {
    *SIMULATOR.write().unwrap() = simulator;
}

pub fn get_runtime() -> RwLockWriteGuard<'static, Runtime> {
    RUNTIME.write().unwrap()
}

pub fn register_callbacks(
    signal_error: CallbackFnStr,
    signal_program_completion: CallbackFnI64,
    signal_termination: CallbackFnUnit,
) {
    *SIGNAL_ERROR.write().unwrap() = signal_error;
    *SIGNAL_PROGRAM_COMPLETION.write().unwrap() = signal_program_completion;
    *SIGNAL_TERMINATION.write().unwrap() = signal_termination;
}

pub fn stop() {
    SHOULD_STOP.store(true, Ordering::SeqCst);
    get_runtime().abort();
}

pub fn reset() {
    stop();
    get_simulator().reset();
}

pub fn load(lines: &str) -> Result<(), String> {
    let mut simulator = get_simulator();

    for line_string in lines
        .lines()
        .map(|string| string.to_string())
        .collect::<Vec<String>>()
    {
        let line_parse = lexer::parse_line(&line_string.to_string(), simulator.get_word_size());

        match line_parse {
            None => { /* no-op */ }
            Some(x) => match x {
                Ok(line) => match simulator.add_line(line) {
                    Ok(_) => {}
                    Err(error) => return Err(format!("Error parsing program: {}", error)),
                },
                Err(error) => return Err(format!("Error parsing program: {}", error)),
            },
        };
    }
    Ok(())
}

fn conclude_simulator_execution() -> bool {
    let simulator = get_simulator();
    if simulator.is_error() {
        signal_error(
            format!(
                "Invalid PC: {}",
                simulator.get_registers().get_pc().get_data().int_value()
            )
            .as_str(),
        );
        false
    } else if simulator.is_done() {
        signal_program_completion(
            simulator
                .get_registers()
                .get_register(&registry::R0.to_string())
                .unwrap()
                .get_data()
                .int_value(),
        );
        false
    } else {
        true
    }
}

pub fn run() {
    SHOULD_STOP.store(false, Ordering::SeqCst);
    while !get_simulator().is_done() && !get_simulator().is_error() {
        if SHOULD_STOP.load(Ordering::SeqCst) {
            break;
        }
        match get_simulator().run_line_from_pc() {
            Ok(_) => {}
            Err(error) => {
                signal_error(format!("Program error: {}", error).as_str());
                return;
            }
        }
    }

    if conclude_simulator_execution() {
        signal_termination();
    }
}

pub fn step() {
    match get_simulator().run_line_from_pc() {
        Ok(_) => {}
        Err(error) => {
            signal_error(format!("Program error: {}", error).as_str());
            return;
        }
    }

    conclude_simulator_execution();
}

pub fn is_completed() -> bool {
    get_simulator().is_done() || get_simulator().is_error()
}

pub fn get_exit_status() -> i64 {
    get_simulator()
        .get_registers()
        .get_register(&registry::R0.to_string())
        .unwrap()
        .get_data()
        .int_value()
}

pub fn get_register_value(register: &str) -> Option<i64> {
    match get_simulator()
        .get_registers()
        .get_register(&register.to_string())
    {
        Ok(x) => Some(x.get_data().int_value()),
        Err(_) => None,
    }
}

fn signal_error(error: &str) {
    SIGNAL_ERROR.read().unwrap()(error);
}

fn signal_program_completion(exit_status: i64) {
    SIGNAL_PROGRAM_COMPLETION.read().unwrap()(exit_status);
}

fn signal_termination() {
    SIGNAL_TERMINATION.read().unwrap()();
}

pub fn initialize_runtime(runtime: tokio::runtime::Runtime) {
    *RUNTIME.write().unwrap() = Runtime::from_rt(SHOULD_STOP.clone(), runtime);
}
