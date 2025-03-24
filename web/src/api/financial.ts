import request from "@/utils/request";
import type { 
  IncomeProofData, 
  CreditScoreData, 
  CrossBorderData, 
  FinancialVerifyData 
} from "@/types/financial";

export const financialApi = {
  // 颁发收入证明
  issueIncomeProof: (data: IncomeProofData) => {
    return request({
      url: "/financial/income",
      method: "post",
      data,
    });
  },

  // 颁发信用评分证明
  issueCreditScore: (data: CreditScoreData) => {
    return request({
      url: "/financial/credit",
      method: "post",
      data,
    });
  },

  // 颁发跨境信用证明
  issueCrossBorder: (data: CrossBorderData) => {
    return request({
      url: "/financial/cross_border",
      method: "post",
      data,
    });
  },

  // 验证金融凭证
  verifyFinancial: (data: FinancialVerifyData) => {
    console.log('准备验证金融凭证，数据:', data);
    // 确保hash是字符串类型，并且被正确包装
    const requestData = {
      hash: String(data.hash || '')
    };
    console.log('发送给后端的验证请求数据:', requestData);
    
    return request({
      url: "/financial/verify",
      method: "post",
      data: requestData,
    });
  },

  // 获取金融凭证
  getFinancial: (hash: string) => {
    return request({
      url: "/financial/get",
      method: "post",
      data: { hash },
    });
  },
  
  // 获取金融凭证列表
  listFinancial: (personal_id?: string) => {
    return request({
      url: "/financial/list",
      method: "post",
      data: { personal_id },
    });
  }
};
