// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6;
pragma experimental ABIEncoderV2;

abstract contract InboundChannel {
    uint64 public nonce;

    struct Message {
        address target;
        uint64 nonce;
        bytes payload;
    }

    event MessageDelivered(uint64 nonce, bool result);

    function submit(Message[] memory commitment, bytes32 _commitment)
        public
        virtual;
}
