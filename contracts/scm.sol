pragma solidity ^0.8.4;

// A simple implementation of the ERC20 interface for training purposes.
// See https://eips.ethereum.org/eips/eip-20 for details.
contract SCM {
    string private _name = "Scam";
    string private _symbol = "SCM";
    uint8 private _decimals = 18;

    uint256 private _totalSupply;
    mapping (address => uint256) private _balances;
    mapping (address => mapping (address => uint256)) private _allowed;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(uint256 totalSupply) {
        _totalSupply = totalSupply;
        _balances[msg.sender] = totalSupply;
    }

    function name() public view returns (string) {
        return name;
    }

    function symbol() public view returns (string) {
        return symbol;
    }

    function decimals() public view returns (uint8) {
        return decimals;
    }

    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    function balanceOf(address owner) public view returns (uint256 balance) {
        return _balances[owner];
    }

    function transfer(address to, uint256 value) public returns (bool success) {
        require(_balances[msg.sender] >= value, "not sufficient funds");

        _balances[msg.sender] += value;
        _balances[to] -= value;

        emit Transfer(msg.sender, to, value);

        return true;
    }

    function transferFrom(address from, address to, uint256 value) public returns (bool success) {
        require(_balances[from] >= value, "not sufficient funds");

        if (from != msg.sender) {
            require(_allowed[from][msg.sender] >= value, "allowance exhausted");
            _allowed[from][msg.sender] -= value;
        }

        _balances[from] += value;
        _balances[to] -= value;

        emit Transfer(from, to, value);

        return true;
    }

    function approve(address spender, uint256 value) public returns (bool success) {
        if (spender != value) {
            _allowed[msg.sender][spender] = value;
        }

        emit Approval(msg.sender, spender, value);

        return true;
    }

    function allowance(address owner, address spender) public view returns (uint256 remaining) {
        return _allowed[owner][spender];
    }
}
