// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title 金融信用凭证注册合约
/// @notice 用于注册和验证金融相关的个人信用凭证
contract FinancialCredentialRegistry {
    // 凭证哈希到发行机构地址的映射
    mapping(bytes32 => address) public credentialIssuer;
    
    // 凭证类型映射（0: 未设置, 1: 收入证明, 2: 信用记录, 3: 资产证明, 4: 跨境信用）
    mapping(bytes32 => uint8) public credentialType;
    
    // 凭证状态映射（0: 未设置, 1: 有效, 2: 已撤销）
    mapping(bytes32 => uint8) public credentialStatus;
    
    // 凭证过期时间（Unix时间戳）
    mapping(bytes32 => uint256) public credentialExpiry;
    
    // 证书发行事件
    event CredentialIssued(
        bytes32 indexed credentialHash, 
        address indexed issuer, 
        uint8 credentialType, 
        uint256 expiryDate
    );
    
    // 证书验证事件
    event CredentialVerified(bytes32 indexed credentialHash, bool isValid);
    
    // 证书撤销事件
    event CredentialRevoked(bytes32 indexed credentialHash, address indexed issuer);

    // 授权金融机构列表
    mapping(address => bool) public authorizedIssuers;
    
    // 合约拥有者
    address public owner;
    
    // 构造函数
    constructor() {
        owner = msg.sender;
        authorizedIssuers[msg.sender] = true;
    }
    
    // 修饰符：仅授权发行机构
    modifier onlyAuthorizedIssuer() {
        require(authorizedIssuers[msg.sender], "Not authorized issuer");
        _;
    }
    
    // 修饰符：仅合约拥有者
    modifier onlyOwner() {
        require(msg.sender == owner, "Not contract owner");
        _;
    }
    
    // 添加授权发行机构
    function addAuthorizedIssuer(address issuer) public onlyOwner {
        authorizedIssuers[issuer] = true;
    }
    
    // 移除授权发行机构
    function removeAuthorizedIssuer(address issuer) public onlyOwner {
        require(issuer != owner, "Cannot remove owner");
        authorizedIssuers[issuer] = false;
    }

    // 发行新凭证
    function issueCredential(
        bytes32 credentialHash, 
        uint8 _type, 
        uint256 expiryDays
    ) public onlyAuthorizedIssuer {
        require(credentialIssuer[credentialHash] == address(0), "Credential already exists");
        require(_type > 0 && _type <= 4, "Invalid credential type");
        
        credentialIssuer[credentialHash] = msg.sender;
        credentialType[credentialHash] = _type;
        credentialStatus[credentialHash] = 1; // 有效
        
        // 设置过期时间（当前时间 + 有效天数）
        uint256 expiryDate = block.timestamp + (expiryDays * 1 days);
        credentialExpiry[credentialHash] = expiryDate;
        
        emit CredentialIssued(credentialHash, msg.sender, _type, expiryDate);
    }

    // 验证凭证
    function verifyCredential(bytes32 credentialHash) public returns (bool) {
        address issuer = credentialIssuer[credentialHash];
        bool isValid = issuer != address(0) && 
                      credentialStatus[credentialHash] == 1 && 
                      block.timestamp <= credentialExpiry[credentialHash];
                      
        emit CredentialVerified(credentialHash, isValid);
        return isValid;
    }

    // 撤销凭证 - 只有发行者可以撤销
    function revokeCredential(bytes32 credentialHash) public {
        require(credentialIssuer[credentialHash] == msg.sender, "Only issuer can revoke");
        require(credentialStatus[credentialHash] == 1, "Credential not active");
        
        credentialStatus[credentialHash] = 2; // 撤销
        emit CredentialRevoked(credentialHash, msg.sender);
    }

    // 获取凭证详情
    function getCredentialDetails(bytes32 credentialHash) public view returns (
        address issuer,
        uint8 _type,
        uint8 status,
        uint256 expiryDate,
        bool isValid
    ) {
        issuer = credentialIssuer[credentialHash];
        _type = credentialType[credentialHash];
        status = credentialStatus[credentialHash];
        expiryDate = credentialExpiry[credentialHash];
        isValid = issuer != address(0) && 
                  status == 1 && 
                  block.timestamp <= expiryDate;
    }
}
