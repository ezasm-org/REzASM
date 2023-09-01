use crate::instructions::targets::input_target::Input;
use crate::instructions::targets::output_target::Output;
use crate::simulation::registry;
use crate::simulation::simulator::Simulator;
use crate::util::error::EzasmError;
use crate::util::raw_data::RawData;

pub trait InputOutput: Input + Output {}

#[derive(Debug, Clone)]
pub enum InputOutputTarget {
    DereferenceInputOutput(usize, i64),
    RegisterInputOutput(usize),
}

impl InputOutput for InputOutputTarget {}

impl Input for InputOutputTarget {
    fn get(&self, simulator: &Simulator) -> Result<RawData, EzasmError> {
        let data = self.register_data(simulator);
        match self {
            InputOutputTarget::DereferenceInputOutput(_, x) => simulator.get_memory().read(
                (match data {
                    Ok(x) => x,
                    Err(error) => return Err(error),
                }
                .int_value()
                    + x) as usize,
            ),
            InputOutputTarget::RegisterInputOutput(_r) => data,
        }
    }
}

impl Output for InputOutputTarget {
    fn set(&mut self, simulator: &mut Simulator, data: RawData) -> Result<(), EzasmError> {
        match self {
            InputOutputTarget::DereferenceInputOutput(r, _) => simulator
                .get_registers_mut()
                .get_register_by_number_mut(r.clone())
                .map(|r| r.set_data(data)),
            InputOutputTarget::RegisterInputOutput(r) => simulator
                .get_registers_mut()
                .get_register_by_number_mut(r.clone())
                .map(|r| r.set_data(data)),
        }
    }
}

impl InputOutputTarget {
    fn register_data(&self, simulator: &Simulator) -> Result<RawData, EzasmError> {
        let register = match self {
            InputOutputTarget::DereferenceInputOutput(r, _) => {
                simulator.get_registers().get_register_by_number(r.clone())
            }
            InputOutputTarget::RegisterInputOutput(r) => {
                simulator.get_registers().get_register_by_number(r.clone())
            }
        };
        match register {
            Ok(r) => Ok(r.get_data().clone()),
            Err(error) => Err(error),
        }
    }

    pub fn new_dereference(register: &String) -> Result<InputOutputTarget, EzasmError> {
        InputOutputTarget::new_dereference_offset(register, 0)
    }

    pub fn new_dereference_offset(
        register: &String,
        offset: i64,
    ) -> Result<InputOutputTarget, EzasmError> {
        registry::get_register_number(register)
            .map(|r| InputOutputTarget::DereferenceInputOutput(r, offset))
    }

    pub fn new_register(register: &String) -> Result<InputOutputTarget, EzasmError> {
        registry::get_register_number(register).map(|r| InputOutputTarget::RegisterInputOutput(r))
    }
}
