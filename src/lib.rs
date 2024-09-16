#![cfg_attr(not(test), no_main, no_std)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use openzeppelin_stylus::token::erc20::Erc20;
use openzeppelin_stylus::token::erc20::IErc20;
use openzeppelin_stylus::token::erc20::extensions::Erc20Metadata;
use openzeppelin_stylus::utils::Pausable;
use openzeppelin_stylus::access::control::AccessControl;
use stylus_sdk::prelude::{entrypoint, external, sol_storage};
use stylus_sdk::msg;
use stylus_proc::SolidityError;

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

sol! {
    /// The sender `account` is blacklisted.
    ///
    /// * `account` - Account that was found to be blacklisted.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExampleSenderBlacklisted(address account);
    /// The recipient `account` is blacklisted.
    ///
    /// * `account` - Account that was found to be blacklisted.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExampleRecipientBlacklisted(address account);
    /// The recipient `account` is already blacklisted.
    ///
    /// * `account` - Account that was found to be blacklisted.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExampleAlreadyBlacklisted(address account);
    /// The `account` is not blacklisted.
    /// 
    /// * `account` - Account that was found to be not blacklisted.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExampleNotBlacklistedAccount(address account);
    /// The contract is paused.
    /// 
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExamplePausedContract();
    /// The account is not authorized.
    /// 
    /// * `account` - Account that was found to be not authorized.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExampleNotAuthorized(address account);
    /// An internal error occurred.
    /// 
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ERC20ExampleInternalError();
}

/// An error that occurred in this contract.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// The caller account is blacklisted.
    SenderBlacklisted(ERC20ExampleSenderBlacklisted),
    /// The caller account is blacklisted.
    RecipientBlacklisted(ERC20ExampleRecipientBlacklisted),
    /// The address is already blacklisted.
    AlreadyBlacklisted(ERC20ExampleAlreadyBlacklisted),
    /// The address is not blacklisted.
    RecipientNotBlacklisted(ERC20ExampleNotBlacklistedAccount),
    /// The contracts is paused
    ContractPaused(ERC20ExamplePausedContract),
    /// The account is not authorized
    NotAuthorized(ERC20ExampleNotAuthorized),
    /// Internal Error
    InternalError(ERC20ExampleInternalError),
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
    ) -> Result<(), Error> {
        self.access.only_role(Erc20Example::BLACKLISTER_ROLE.into()).map_err(|_| Error::NotAuthorized(ERC20ExampleNotAuthorized{ account: msg::sender() }))?;
        if self.is_blacklisted(account).unwrap() {
            return Err(Error::AlreadyBlacklisted(ERC20ExampleAlreadyBlacklisted{ account }));
        }
        self._blacklist.insert(account, true);
        Ok(())
    }

    pub fn remove_from_blacklist(
        &mut self,
        account: Address,
    ) -> Result<(), Error> {
        self.access.only_role(Erc20Example::BLACKLISTER_ROLE.into()).map_err(|_| Error::NotAuthorized(ERC20ExampleNotAuthorized{ account: msg::sender() }))?;
        if !self.is_blacklisted(account).unwrap() {
            return Err(Error::RecipientNotBlacklisted(ERC20ExampleNotBlacklistedAccount{ account }));
        }
        self._blacklist.delete(account);
        Ok(())
    }
    

    pub fn pause(&mut self) -> Result<(), Error> {
        self.access.only_role(Erc20Example::PAUSER_ROLE.into()).map_err(|_| Error::NotAuthorized(ERC20ExampleNotAuthorized{ account: msg::sender() }))?;
        self.pausable.pause().map_err(|_| Error::InternalError(ERC20ExampleInternalError{}))?;
        Ok(())
    }

    pub fn transfer(
        &mut self,
        to: Address,
        value: U256,
    ) -> Result<bool, Error> {
        
        if self.is_blacklisted(msg::sender()).unwrap() {
            return Err(Error::SenderBlacklisted(ERC20ExampleSenderBlacklisted{ account: msg::sender() }));
        }
        
        self.pausable.when_not_paused().map_err(|_| Error::ContractPaused(ERC20ExamplePausedContract{}))?;
        let result = self.erc20.transfer(to, value).map_err(|_| Error::InternalError(ERC20ExampleInternalError{}))?;
        Ok(result) 
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Error> {
        if self.is_blacklisted(msg::sender()).unwrap() {
            return Err(Error::SenderBlacklisted(ERC20ExampleSenderBlacklisted{ account: msg::sender() }));
        }
        
        self.pausable.when_not_paused().map_err(|_| Error::ContractPaused(ERC20ExamplePausedContract{}))?;
        let result = self.erc20.transfer_from(from, to, value).map_err(|_| Error::InternalError(ERC20ExampleInternalError{}))?;
        Ok(result)
    }
}