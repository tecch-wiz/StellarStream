#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // init function
    pub fn init(_env: Env, name: String) -> String {
        name
    }

    pub fn hello(_env: Env, name: String) -> String {
        name
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_hello() {
        let env = Env::default();
        let name = String::from_str(&env, "Alice");
        let result = HelloContract::hello(env.clone(), name.clone());
        assert_eq!(result, name);
    }

    #[test]
    fn test_init() {
        let env = Env::default();
        let name = String::from_str(&env, "Bob");
        let result = HelloContract::init(env.clone(), name.clone());
        assert_eq!(result, name);
    }
}
