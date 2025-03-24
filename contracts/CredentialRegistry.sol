// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract CredentialRegistry {
    // 证书哈希到发行机构地址的映射
    mapping(bytes32 => address) public credentialIssuer;
    
    // 证书发行事件
    event CredentialIssued(bytes32 indexed credentialHash, address indexed issuer);
    
    // 证书验证事件
    event CredentialVerified(bytes32 indexed credentialHash, bool isValid);

    // 发行新证书
    function issueCredential(bytes32 credentialHash) public {
        require(credentialIssuer[credentialHash] == address(0), "Credential already exists");
        credentialIssuer[credentialHash] = msg.sender;
        emit CredentialIssued(credentialHash, msg.sender);
    }

    // 验证证书
    function verifyCredential(bytes32 credentialHash) public returns (bool) {
        bool isValid = credentialIssuer[credentialHash] != address(0);
        emit CredentialVerified(credentialHash, isValid);
        return isValid;
    }

    // 获取证书发行者
    function getCredentialIssuer(bytes32 credentialHash) public view returns (address) {
        return credentialIssuer[credentialHash];
    }
}
