// SPDX-License-Identifier: Unlicense

pragma solidity ^0.5.0;

import "./scm.sol";
import "canonical-weth/contracts/WETH9.sol";

contract ICO {
    // Current state of the ICO.
    enum State {
        // ICO is is progress, new contributions are accepted.
        Ongoing,

        // ICO is closed, new contributions are not accepted.
        // SCM tokens are pending release.
        Closed,

        // ICO is finished, SCM tokens are released
        // and available for withdrawal.
        Finished
    }

    // How much ether do we want to collect.
    uint256 public constant target = 10 ether;

    // Rate for converting ETH to SCM.
    uint256 public constant rate = 10;

    // Time between ICO is closed and tokens are released.
    uint256 public constant holdDuration = 2 minutes;

    // SCM coin we're issuing.
    SCM public scm;

    // WETH coin we're collecting.
    WETH9 public weth;

    // Emitted when the ICO is closed.
    //
    // Args:
    //
    // - `closedTime`: timestamp of when this ICO was closed.
    // - `finishedTime`: timestamp of when issued SCM tokens
    //   will be available for withdrawal.
    event IcoClosed(uint256 closedTime, uint256 finishedTime);

    // Emitted when someone funds this ICO.
    //
    // Args:
    //
    // - `buyer`: who bought the tokens.
    // - `ethersUsed`: number of ETH tokens added to the ICO.
    // - `scmPurchased`: number of SCM tokens added to the buyer's balance.
    event Fund(address indexed buyer, uint256 ethUsed, uint256 scmPurchased);

    uint256 private _left = target;
    uint256 private _closeTime = 0;
    mapping (address => uint256) private _contributions;

    constructor(WETH9 _weth) public {
        scm = new SCM(toScm(target));
        weth = _weth;
    }

    // Convert ETH to SCM.
    function toScm(uint256 eth) public pure returns (uint256) {
        return eth * rate;
    }

    // Get the current state of the ICO.
    function state() public view returns (State) {
        if (_left > 0) {
            return State.Ongoing;
        } else if (block.timestamp < _closeTime + holdDuration) {
            return State.Closed;
        } else {
            return State.Finished;
        }
    }

    // Time when this ICO was closed. It is an error to call this function
    // when the ICO is still ongoing.
    function closeTime() public view returns (uint256) {
        require(state() != State.Ongoing, "ICO is still ongoing");

        return _closeTime;
    }

    // Time when this ICO was or will be finished. It is an error to call
    // this function when the ICO is still ongoing.
    function finishTime() public view returns (uint256) {
        require(state() != State.Ongoing, "ICO is still ongoing");

        return _closeTime + holdDuration;
    }

    // Get number of WETH tokens that's left to gather to close this ICO.
    function leftEth() public view returns (uint256) {
        return _left;
    }

    // Get number of SCM tokens available for purchase.
    function leftScm() public view returns (uint256) {
        return toScm(_left);
    }

    // Get number of ETH tokens contributed by the given user.
    // The balance will become zero once the user claims their tokens.
    function balanceEth(address owner) public view returns (uint256) {
        return _contributions[owner];
    }

    // Get number of SCM tokens available for the given user.
    // The balance will become zero once the user claims their tokens.
    function balanceScm(address owner) public view returns (uint256) {
        return toScm(_contributions[owner]);
    }

    // Fund this ICO with the given amount of WETH. If there's not enough
    // SCM left for the given amount of WETH, this function fails.
    // Returns number of purchased SCM tokens.
    function fund(uint256 funds) public returns (uint256) {
        require(state() == State.Ongoing, "ICO is closed");

        require(funds <= _left, "not enough tokens left");

        require(weth.balanceOf(msg.sender) >= funds, "not enough WETH");
        require(weth.allowance(msg.sender, address(this)) >= funds, "not allowed to spend WETH");

        require(weth.transferFrom(msg.sender, address(this), funds));

        _contributions[msg.sender] += funds;
        _left -= funds;

        emit Fund(msg.sender, funds, toScm(funds));

        if (_left == 0) {
            _closeTime = block.timestamp;
            emit IcoClosed(closeTime(), finishTime());
        }

        return toScm(funds);
    }

    // Fund this ICO with the given amount of WETH. If there's not enough
    // SCM left for the given amount of WETH, purchase all available SCM.
    // Returns number of spent WETH and number of purchased SCM tokens.
    function fundAny(uint256 funds) public returns (uint256 spent, uint256 claimed) {
        if (funds > _left) {
            return (_left, fund(_left));
        } else {
            return (funds, fund(funds));
        }
    }

    // Claim purchased SCM. Returns number of transferred tokens.
    // It is an error to call this function twice, or to call it if user's
    // balance is zero.
    function claim() public returns (uint256) {
        require(state() == State.Finished, "ICO is not finished yet");

        uint256 balance = toScm(_contributions[msg.sender]);

        require(balance > 0, "no SCM tokens to claim");
        require(scm.transfer(msg.sender, balance));

        _contributions[msg.sender] = 0;

        return balance;
    }
}
