# API 测试说明

本目录包含了对学历证书验证系统 API 的自动化测试。

## 环境要求

- Python 3.8+
- pip

## 安装依赖

```bash
pip install -r requirements.txt
```

## 配置

在运行测试之前，请确保：

1. 后端服务已启动（默认地址：http://localhost:8000）
2. 创建 `.env` 文件并设置以下环境变量：
   ```
   API_BASE_URL=http://localhost:8000
   ```

## 运行测试

```bash
# 运行所有测试
pytest

# 生成HTML测试报告
pytest --html=report.html

# 运行特定测试文件
pytest test_credential_api.py
```

## 测试用例说明

### test_credential_api.py

1. **test_issue_credential**
   - 测试证书颁发功能
   - 验证响应状态码和返回的证书哈希

2. **test_verify_credential**
   - 测试证书验证功能
   - 先颁发证书，然后验证其有效性

3. **test_get_credential**
   - 测试获取证书信息功能
   - 验证返回的证书信息是否正确

4. **test_error_handling**
   - 测试错误处理
   - 包括无效哈希和缺少必填字段的情况
