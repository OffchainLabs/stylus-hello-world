//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```solidity
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{U256, Address}, call::{call, Call}, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// Adds the wei value from msg_value to the number in storage.
    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let number = self.number.get();
        self.set_number(number + self.vm().msg_value());
    }
    // External call example
    pub fn call_external_contract(&mut self, target: Address, data: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {        
        let return_data = call(
            Call::new_in(self), // Configuration for gas, value, etc.
            target,  // The target contract address
            &data, // Raw calldata to be sent
        )?;

        // Return the raw return data from the contract call
        Ok(return_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counter() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Counter::from(&vm);

        assert_eq!(U256::ZERO, contract.number());

        contract.increment();
        assert_eq!(U256::from(1), contract.number());

        contract.add_number(U256::from(3));
        assert_eq!(U256::from(4), contract.number());

        contract.mul_number(U256::from(2));
        assert_eq!(U256::from(8), contract.number());

        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.number());

        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));

        contract.add_from_msg_value();
        assert_eq!(U256::from(102), contract.number());
    }
    #[test]
    fn test_external_call() {
        use stylus_sdk::testing::*;
        // 1) Create the VM and the contract instance
        let vm = TestVM::new();
        let mut contract = Counter::from(&vm);
        
      // 2) Prepare inputs
      let target = Address::from([0x05; 20]);
      let call_data = vec![1, 2, 3, 4];
      let success_ret = vec![5, 6, 7, 8];
      let error_ret   = vec![9, 9, 9];
    
      // 3) Mock a successful external call
      vm.mock_call(target, call_data.clone(), Ok(success_ret.clone()));
      let got = contract
        .call_external_contract(target, call_data.clone())
        .unwrap();
    
      assert_eq!(got, success_ret);
    
      // 4) Mock a reverting external call
      vm.mock_call(target, call_data.clone(), Err(error_ret.clone()));
      let err = contract
        .call_external_contract(target, call_data)
        .unwrap_err();
      assert_eq!(err, error_ret);
    }
}
