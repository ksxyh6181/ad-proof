import requests
import json
import logging
import os
from dotenv import load_dotenv

# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# 加载环境变量
load_dotenv()

# 配置
API_BASE_URL = os.getenv('API_BASE_URL', 'http://127.0.0.1:8090')

# 测试数据
TEST_CREDENTIAL = {
    'student_id': '2024001',
    'name': 'Test Student',
    'degree': 'Bachelor of Computer Science',
    'graduation_date': '2024-06-30'
}

HEADERS = {
    'education_institution': {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        'X-Role': 'education_institution'
    },
    'student': {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        'X-Role': 'student'
    },
    'employer': {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        'X-Role': 'employer'
    }
}

def check_response(response, expected_code=200):
    """检查响应状态和结构"""
    logger.info(f"Response status: {response.status_code}")
    
    assert response.status_code == expected_code, \
        f"Expected status code {expected_code}, got {response.status_code}"
    
    try:
        data = response.json()
    except json.JSONDecodeError as e:
        logger.error(f"Failed to decode JSON response: {e}")
        logger.error(f"Response text: {response.text}")
        raise AssertionError(f"Invalid JSON response: {response.text}")
    
    #logger.info(f"Response JSON: {json.dumps(data, indent=2, ensure_ascii=False)}")
    
    if expected_code == 200:
        assert 'code' in data, "Response missing 'code' field"
        assert 'msg' in data, "Response missing 'msg' field"
        assert 'data' in data, "Response missing 'data' field"
        assert data['code'] == 200, f"Response code not 200: {data}"
        assert data['data'] is not None, "Response data is None"
        return data['data']
    else:
        assert 'code' in data, "Error missing 'code' field"
        assert data['code'] == expected_code, f"Error code {data['code']} does not match expected {expected_code}"
        return None

def test_credential_flow():
    """测试完整的证书流程，包括颁发、查看和验证"""
    
    # 1. 测试教育机构颁发证书
    def test_institution_issues_credential():
        logger.info("Testing credential issuance")
        url = f"{API_BASE_URL}/api/credential/issue"
        
        # 测试成功颁发
        response = requests.post(url, json=TEST_CREDENTIAL, headers=HEADERS['education_institution'])
        data = check_response(response)
        assert 'credential' in data, "Missing credential in response"
        assert 'hash' in data, "Missing hash in response"
        credential_hash = data['hash']
        
        # 测试学生无法颁发
        response = requests.post(url, json=TEST_CREDENTIAL, headers=HEADERS['student'])
        check_response(response, 403)
        
        return credential_hash
    
    # 2. 测试查看证书
    def test_view_credential(credential_hash):
        logger.info("Testing credential view")
        url = f"{API_BASE_URL}/api/credential/get"
        payload = {"hash": credential_hash}
        
        # 测试各角色查看
        for role, headers in HEADERS.items():
            logger.info(f"Testing {role} view")
            response = requests.post(url, json=payload, headers=headers)
            data = check_response(response)
            
            # 验证基本字段
            assert data['hash'] == credential_hash
            assert data['degree'] == TEST_CREDENTIAL['degree']
            assert data['graduation_date'] == TEST_CREDENTIAL['graduation_date']
            assert data['name'] == TEST_CREDENTIAL['name']
            assert data['student_id'] == TEST_CREDENTIAL['student_id']
            
            # 验证 proof 字段
            # if role == 'education_institution':
            #     assert 'proof' in data, "Institution should see proof"
            # else:
            #     assert 'proof' not in data or data['proof'] is None, f"{role} should not see proof"
    
    # 3. 测试证书验证权限
    def test_verify_credential_permissions(credential_hash):
        logger.info("Testing credential verification permissions")
        
        # 首先获取完整的证书信息
        url = f"{API_BASE_URL}/api/credential/get"
        payload = {"hash": credential_hash}
        response = requests.post(url, json=payload, headers=HEADERS['education_institution'])
        data = check_response(response)
        credential = data
        
        # 然后验证证书
        url = f"{API_BASE_URL}/api/credential/verify"
        payload = {
            "hash": credential['hash'],
            "proof": credential['proof']
        }
        
        # 测试各角色验证
        for role, headers in HEADERS.items():
            logger.info(f"Testing {role} verification，content: {json.dumps(payload, indent=2)}")
            response = requests.post(url, json=payload, headers=headers)
            data = check_response(response)
            assert data['message'] == "证书验证通过", f"Verification failed for {role}"
    
    # 4. 测试错误情况
    def test_error_cases():
        logger.info("Testing error cases")
        
        # 测试无效证书哈希
        url = f"{API_BASE_URL}/api/credential/verify"
        payload = {
            "credential": {
                "hash": "invalid_hash"
            }
        }
        
        for role, headers in HEADERS.items():
            response = requests.post(url, json=payload, headers=headers)
            data = check_response(response,400)
            assert data['code'] != 200, f"Invalid hash should fail for {role}"
        
        # 测试缺少必填字段
        url = f"{API_BASE_URL}/api/credential/issue"
        invalid_credential = {
            "name": "Test Student",
            "degree": "Bachelor",
            "graduation_date": "2024-06-30"
        }
        response = requests.post(url, json=invalid_credential, headers=HEADERS['education_institution'])
        data = response.json()
        assert data['code'] != 200, "Missing required fields should fail"
    
    # 5. 测试学位验证
    def test_degree_validation():
        logger.info("Testing degree validation")
        
        # 测试有效学位
        valid_data = {
            "student_id": "2024001",
            "name": "张三",
            "degree": "计算机科学学士",  # 包含有效关键词
            "graduation_date": "2024-06-30"
        }
        url = f"{API_BASE_URL}/api/credential/issue"
        response = requests.post(url, json=valid_data, headers=HEADERS['education_institution'])
        data = check_response(response)
        assert 'credential' in data, "Missing credential in response"
        assert 'hash' in data, "Missing hash in response"
        
        # 测试无效学位
        invalid_data = {
            "student_id": "2024002",
            "name": "李四",
            "degree": "硕士",  # 无效学历
            "graduation_date": "2024-06-30"
        }
        response = requests.post(url, json=invalid_data, headers=HEADERS['education_institution'])
        data = check_response(response, 400)
        assert "degree requirement not met" in data['msg'].lower()
        
        # 测试边界情况
        test_cases = [
            ("BACHELOR of Engineering", 200),  # 全大写
            ("Undergraduate Degree", 200),     # 包含undergraduate
            ("硕士", 400),                     # 无效中文
            ("博士", 400),                     # 无效中文
            ("本科", 400)                      # 中文但不含关键词
        ]
        
        for degree, expected_status in test_cases:
            data = {
                "student_id": "202400X",
                "name": "测试用户",
                "degree": degree,
                "graduation_date": "2024-06-30"
            }
            response = requests.post(url, json=data, headers=HEADERS['education_institution'])
            if expected_status == 200:
                check_response(response)
            else:
                check_response(response, expected_status)
    
    # 执行所有测试
    try:
        credential_hash = test_institution_issues_credential()
        test_view_credential(credential_hash)
        test_verify_credential_permissions(credential_hash)
        test_error_cases()
        test_degree_validation()
        logger.info("All tests passed successfully!")
    except AssertionError as e:
        logger.error(f"Test failed: {str(e)}")
        raise
    except Exception as e:
        logger.error(f"Unexpected error: {str(e)}")
        raise

if __name__ == '__main__':
    test_credential_flow()


