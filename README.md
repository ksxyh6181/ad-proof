# 学历证书验证系统

基于零知识证明的学历证书验证系统，实现了证书的安全颁发、验证和查询功能。通过零知识证明技术，确保学历信息的真实性和隐私保护。

## 功能特点

- 基于零知识证明的证书验证
- 支持证书的颁发、验证和查询
- 多级别的信息保护机制
- 基于角色的访问控制
- 完整的API接口支持
- 用户友好的Web界面
- 自动化的证书验证流程

## 系统架构

### 技术栈

- **前端**: Vue 3, Element Plus
- **后端**: Rust, Salvo
- **加密**: 零知识证明 (bellman, bls12_381)
- **数据存储**: 文件系统

### 项目结构

```
AdProof/
├── web/             # 前端代码
│   ├── src/         # 源代码
│   ├── public/      # 静态资源
│   └── components/  # Vue组件
└── server/          # 后端代码
    └── crates/
        ├── bin/     # 可执行文件
        │   └── gateway/  # Web服务
        └── lib/     # 库代码
            └── zkp/      # 零知识证明实现
```

## 快速开始

### 系统要求

- Node.js 16+
- Rust 1.70+
- Cargo
- pnpm

### 安装和运行

1. 克隆项目
```bash
git clone https://github.com/yourusername/AdProof.git
cd AdProof
```

2. 安装前端依赖
```bash
cd web
pnpm install
```

3. 启动前端开发服务器
```bash
pnpm dev
```

4. 安装后端依赖并运行
```bash
cd ../server
cargo run
```

## API文档

### 证书相关接口

- `POST /credential/issue` - 颁发新的学历证书
- `POST /credential/verify` - 验证学历证书
- `POST /credential/get` - 获取证书详情

### 请求示例

**颁发证书**:
```json
{
  "student_id": "string",
  "name": "string",
  "degree": "string",
  "graduation_date": "string"
}
```

**验证证书**:
```json
{
  "hash": "string",
  "proof": {
    "proof": number[],
    "public_inputs": number[]
  }
}
```

## 开发指南

### 前端开发

前端使用Vue 3和Element Plus构建，主要功能包括：
- 证书颁发页面
- 证书验证页面
- 证书查询功能

### 后端开发

后端使用Rust和Salvo框架开发，主要模块包括：
- Web服务（gateway）
- 零知识证明实现（zkp）
- 数据存储和访问

## 安全与隐私

- 使用零知识证明保护学历信息隐私
- 实现基于角色的访问控制
- 证书数据安全存储
- 支持可选的信息披露范围

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情
