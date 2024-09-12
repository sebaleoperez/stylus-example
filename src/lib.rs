#![cfg_attr(not(test), no_main, no_std)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::{Address, U256};
use openzeppelin_stylus::token::erc20::Erc20;
use openzeppelin_stylus::token::erc20::IErc20;
use openzeppelin_stylus::token::erc20::extensions::Erc20Metadata;
use openzeppelin_stylus::utils::Pausable;
use openzeppelin_stylus::access::control::AccessControl;
use stylus_sdk::prelude::{entrypoint, external, sol_storage};
use stylus_sdk::msg;

sol_storage! {
    #[entrypoint]
    struct Erc20Example {
        #[borrow]
        Erc20 erc20;
        #[borrow]
        Erc20Metadata metadata;
        #[borrow]
        Pausable pausable;
        #[borrow]
        AccessControl access;
        // Blacklist
        mapping(address => bool) _blacklist;
    }
}

// `keccak256("PAUSER_ROLE")`
pub const PAUSER_ROLE: [u8; 32] = [
    101,215,162,142,50,101,179,122,100,116,146,159,51,101,33,
    179,50,193,104,27,147,63,108,185,243,55,102,115,68,13,134,42
];

// `keccak256("MINTER_ROLE")`
pub const MINTER_ROLE: [u8; 32] = [
    159,45,240,254,210,199,118,72,222,88,96,164,204,80,140,208,
    129,140,133,184,184,161,171,76,238,239,141,152,28,137,86,166
];

// `keccak256("BLACKLISTER_ROLE")`
pub const BLACKLISTER_ROLE: [u8; 32] = [
    152,219,138,34,12,208,240,155,173,206,159,34,208,186,126,147,
    237,179,212,4,68,140,195,86,13,57,26,176,150,173,22,233
];

#[external]
#[inherit(Erc20, Erc20Metadata, Pausable, AccessControl)]
impl Erc20Example {
    pub const PAUSER_ROLE: [u8; 32] = PAUSER_ROLE;
    pub const MINTER_ROLE: [u8; 32] = MINTER_ROLE;
    pub const BLACKLISTER_ROLE: [u8; 32] = BLACKLISTER_ROLE;

    pub fn mint(
        &mut self,
        account: Address,
        value: U256,
    ) -> Result<(), Vec<u8>> {
        self.access.only_role(Erc20Example::MINTER_ROLE.into())?;
        self.pausable.when_not_paused()?;
        self.erc20._mint(account, value)?;
        Ok(())
    }

    fn is_blacklisted(&self, account: Address) -> Result<bool, Vec<u8>> {
        Ok(self._blacklist.get(account))
    }

    pub fn add_to_blacklist(
        &mut self,
        account: Address,
    ) -> Result<(), Vec<u8>> {
        self.access.only_role(Erc20Example::BLACKLISTER_ROLE.into())?;
        if self.is_blacklisted(account).unwrap() {
            return Err(Vec::<u8>::from("Error::AddressAlreadyBlacklisted"));
        }
        self._blacklist.insert(account, true);
        Ok(())
    }

    pub fn remove_from_blacklist(
        &mut self,
        account: Address,
    ) -> Result<(), Vec<u8>> {
        self.access.only_role(Erc20Example::BLACKLISTER_ROLE.into())?;
        if !self.is_blacklisted(account).unwrap() {
            return Err(Vec::<u8>::from("Error::AddressNotBlacklisted"));
        }
        self._blacklist.delete(account);
        Ok(())
    }
    

    pub fn pause(&mut self) -> Result<(), Vec<u8>> {
        self.access.only_role(Erc20Example::PAUSER_ROLE.into())?;
        self.pausable.pause()?;
        Ok(())
    }

    pub fn transfer(
        &mut self,
        to: Address,
        value: U256,
    ) -> Result<bool, Vec<u8>> {
        
        if self.is_blacklisted(msg::sender()).unwrap() {
            return Err(Vec::<u8>::from("Error::AddressBlacklisted"));
        }
        
        self.pausable.when_not_paused()?;
        self.erc20.transfer(to, value).map_err(|e| e.into())
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Vec<u8>> {
        if self.is_blacklisted(msg::sender()).unwrap() {
            return Err(Vec::<u8>::from("Error::AddressBlacklisted"));
        }
        
        self.pausable.when_not_paused()?;
        self.erc20.transfer_from(from, to, value).map_err(|e| e.into())
    }
}