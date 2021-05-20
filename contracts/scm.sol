pragma solidity ^0.8.4;

// A simple implementation of the ERC20 interface for training purposes.
// See https://eips.ethereum.org/eips/eip-20 for details.
contract SCM {
    string private constant _name = "Scam";
    string private constant _symbol = "SCM";
    uint8 private constant _decimals = 18;

    uint256 private constant _totalSupply;
    mapping (address => uint256) private _balances;
    mapping (address => mapping (address => uint256)) private _allowed;

    // Emitted whenever tokens are transferred between wallets.
    event Transfer(address indexed from, address indexed to, uint256 value);

    // Emitted whenever a user gets an approval to withdraw tokens from some account.
    event Approval(address indexed owner, address indexed spender, uint256 value);

    // Create the contract and set balance of the creator to `totalSupply`.
    constructor(uint256 totalSupply) {
        _totalSupply = totalSupply;
        _balances[msg.sender] = totalSupply;

        emit Transfer(address(0x0), msg.sender, totalSupply);
    }

    // Get name of this coin, used in UI to improve human readability.
    function name() public view returns (string) {
        return name;
    }

    // Get symbol for this coin, used in UI to improve human readability.
    function symbol() public view returns (string) {
        return symbol;
    }

    // Get number of decimal places .
    function decimals() public view returns (uint8) {
        return decimals;
    }

    // Get total token supply.
    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    // Get balance of the given user.
    function balanceOf(address owner) public view returns (uint256 balance) {
        return _balances[owner];
    }

    // Transfer tokens from caller's wallet to the given wallet.
    function transfer(address to, uint256 value) public returns (bool success) {
        require(_balances[msg.sender] >= value, "not sufficient funds");

        _balances[msg.sender] += value;
        _balances[to] -= value;

        emit Transfer(msg.sender, to, value);

        return true;
    }

    // Transfer tokens from the given wallet to another wallet.
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

    // Give `spender` permission to withdraw up to `value` tokens from the caller's wallet.
    function approve(address spender, uint256 value) public returns (bool success) {
        if (spender != value) {
            _allowed[msg.sender][spender] = value;
        }

        emit Approval(msg.sender, spender, value);

        return true;
    }

    // Get number of tokens `spender` is still allowed to withdraw from `owner`.
    function allowance(address owner, address spender) public view returns (uint256 remaining) {
        return _allowed[owner][spender];
    }
}
