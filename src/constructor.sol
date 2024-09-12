// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

contract Erc20Example {
    // ERC20-Data
    mapping(address account => uint256) private _balances;
    mapping(address account => mapping(address spender => uint256))
        private _allowances;
    uint256 private _totalSupply;
    string private _name;
    string private _symbol;

    // Pausable-Data
    bool private _paused;

    mapping(address account => uint256) _nonces;

    // AccessControl-Data
    struct RoleData {
        mapping(address account => bool) hasRole;
        bytes32 adminRole;
    }

    mapping(bytes32 role => RoleData) private _roles;

    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;

    // Constructor
    constructor(string memory name_, string memory symbol_, address admin_) {
        _name = name_;
        _symbol = symbol_;
        _paused = false;
        _grantRole(DEFAULT_ADMIN_ROLE, admin_);
    }

    event RoleGranted(
        bytes32 indexed role,
        address indexed account,
        address indexed sender
    );

    function hasRole(
        bytes32 role,
        address account
    ) public view virtual returns (bool) {
        return _roles[role].hasRole[account];
    }

    function _grantRole(
        bytes32 role,
        address account
    ) internal virtual returns (bool) {
        if (!hasRole(role, account)) {
            _roles[role].hasRole[account] = true;
            emit RoleGranted(role, account, msg.sender);
            return true;
        } else {
            return false;
        }
    }
}