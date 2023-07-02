use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const REGISTERS_COUNT: usize = 54;

lazy_static! {
    pub static ref REGISTER_BY_STRING: HashMap<String, usize> = {
        let mut temp_map: HashMap<String, usize> = HashMap::new();

        for (i, reg) in ALL_REGISTERS
        .iter()
        .enumerate()
        {
            temp_map.insert(reg.to_string(), i);
        }
        temp_map
    };
    pub static ref REGISTER_BY_NUMBER: HashMap<usize, String> = {
        let mut temp_map: HashMap<usize, String> = HashMap::new();

        for (i, reg) in ALL_REGISTERS.iter().enumerate()
        {
            temp_map.insert(i, reg.to_string());
        }
        temp_map
    };
}

// Base registers
pub const ZERO: &str = "ZERO"; // The number zero
pub const PID: &str = "PID"; // Program identifier
pub const FID: &str = "FID"; // File Identifier
pub const PC: &str = "PC"; // Program counter
pub const SP: &str = "SP"; // Stack pointer
pub const RA: &str = "RA"; // Return address
pub const A0: &str = "A0"; // Argument 0
pub const A1: &str = "A1"; // Argument 1
pub const A2: &str = "A2"; // Argument 2
pub const R0: &str = "R0"; // Return 0
pub const R1: &str = "R1"; // Return 1
pub const R2: &str = "R2"; // Return 2
pub const BASE_REGISTERS: [&str; 12] = [ZERO, PID, FID, PC, SP, RA, A0, A1, A2, R0, R1, R2];

// Saved registers
pub const S0: &str = "S0";
pub const S1: &str = "S1";
pub const S2: &str = "S2";
pub const S3: &str = "S3";
pub const S4: &str = "S4";
pub const S5: &str = "S5";
pub const S6: &str = "S6";
pub const S7: &str = "S7";
pub const S8: &str = "S8";
pub const S9: &str = "S9";
pub const SAVED_REGISTERS: [&str; 10] = [S0, S1, S2, S3, S4, S5, S6, S7, S8, S9];

// Temporary registers
pub const T0: &str = "T0";
pub const T1: &str = "T1";
pub const T2: &str = "T2";
pub const T3: &str = "T3";
pub const T4: &str = "T4";
pub const T5: &str = "T5";
pub const T6: &str = "T6";
pub const T7: &str = "T7";
pub const T8: &str = "T8";
pub const T9: &str = "T9";
pub const TEMPORARY_REGISTERS: [&str; 10] = [T0, T1, T2, T3, T4, T5, T6, T7, T8, T9];

// Saved float registers
pub const FS0: &str = "FS0";
pub const FS1: &str = "FS1";
pub const FS2: &str = "FS2";
pub const FS3: &str = "FS3";
pub const FS4: &str = "FS4";
pub const FS5: &str = "FS5";
pub const FS6: &str = "FS6";
pub const FS7: &str = "FS7";
pub const FS8: &str = "FS8";
pub const FS9: &str = "FS9";
pub const FLOAT_SAVED_REGISTERS: [&str; 10] = [FS0, FS1, FS2, FS3, FS4, FS5, FS6, FS7, FS8, FS9];

// Temporary float registers
pub const FT0: &str = "FT0";
pub const FT1: &str = "FT1";
pub const FT2: &str = "FT2";
pub const FT3: &str = "FT3";
pub const FT4: &str = "FT4";
pub const FT5: &str = "FT5";
pub const FT6: &str = "FT6";
pub const FT7: &str = "FT7";
pub const FT8: &str = "FT8";
pub const FT9: &str = "FT9";
pub const FLOAT_TEMPORARY_REGISTERS: [&str; 10] =
    [FT0, FT1, FT2, FT3, FT4, FT5, FT6, FT7, FT8, FT9];

pub const LO: &str = "LO"; // Special "LOW" register to store the lower part of a multiplication
pub const HI: &str = "HI"; // Special "HIGH" register to store the higher part of a multiplication
pub const SPECIAL_REGISTERS: [&str; 2] = [LO, HI];

pub const ALL_REGISTERS: [&str ; 54] = [ZERO, PID, FID, PC, SP, RA, A0, A1, A2, R0, R1, R2, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, FS0, FS1, FS2, FS3, FS4, FS5, FS6, FS7, FS8, FS9, FT0, FT1, FT2, FT3, FT4, FT5, FT6, FT7, FT8, FT9, LO, HI];



pub fn is_register(register: &str) -> bool {
    if register.len() < 1 {
        return false;
    }
    let mut temp = register;
    if register.starts_with("$") {
        temp = &register[1..];
    }
    let binding = temp.to_uppercase();
    temp = binding.as_str();

    let number: usize = match usize::from_str(temp) {
        Ok(x) => x,
        Err(_) => REGISTERS_COUNT + 1,
    };
    REGISTER_BY_STRING.contains_key(&temp.to_string()) || REGISTER_BY_NUMBER.contains_key(&number)
}
