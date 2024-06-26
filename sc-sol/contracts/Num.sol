//SPDX-License-Identifier: MIT
pragma solidity 0.8.6;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "hardhat/console.sol";

contract Num is Initializable {
    uint256 public num;

    // removed so as to make the contract upgradeable
    // constructor(uint256 _num) {
    //     num = _num;
    // }

    // replaced with this, so as to make the contract upgradeable
    function initialize(uint256 _num) external initializer {
        num = _num;
    }

    function update(uint256 _num) external {
        num = _num;
    }
}
