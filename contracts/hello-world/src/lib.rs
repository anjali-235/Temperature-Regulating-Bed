#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct TempData {
    pub current_temp: i32,
    pub preferred_temp: i32,
    pub is_active: bool,
}

const TEMP_KEY: Symbol = symbol_short!("TEMP");

#[contract]
pub struct TempControlContract;

#[contractimpl]
impl TempControlContract {
    // Set user preferred temperature
    pub fn set_preferred_temp(env: Env, temp: i32) {
        let data = TempData {
            current_temp: temp, // Initially equal
            preferred_temp: temp,
            is_active: true,
        };
        env.storage().instance().set(&TEMP_KEY, &data);
    }

    // Get current system temperature
    pub fn get_current_temp(env: Env) -> i32 {
        let data: TempData = env.storage().instance().get(&TEMP_KEY).unwrap();
        data.current_temp
    }

    // Update current temperature by the system
    pub fn update_current_temp(env: Env, new_temp: i32) {
        let mut data: TempData = env.storage().instance().get(&TEMP_KEY).unwrap();
        data.current_temp = new_temp;
        env.storage().instance().set(&TEMP_KEY, &data);
    }

    // View complete temperature data
    pub fn get_status(env: Env) -> TempData {
        env.storage().instance().get(&TEMP_KEY).unwrap()
    }
}
