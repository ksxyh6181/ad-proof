import { PublicKey, Connection } from "@solana/web3.js";

// 静态导入IDL - 这样可以避免运行时导入错误
import CREDENTIAL_IDL from "./idl/credential.json";

// Solana网络配置
// export const SOLANA_NETWORK = process.env.VITE_SOLANA_NETWORK || "localnet";
export const SOLANA_RPC_URL = "http://127.0.0.1:8899";

// 凭证程序ID
export const CREDENTIAL_PROGRAM_ID = new PublicKey(
  "CREDNDPg65AB4U94arwPLJGm2Pzpk3PBwdGxCCJGhZT7"
);

// 注册表公钥 - 部署后需要更新此值
export const REGISTRY_PUBLIC_KEY = new PublicKey(
  "9XwXD5VXHGE8YZEMiLyDHgVzQJQ7FqNTCcCVBdjWvNef"
);

/**
 * 获取凭证程序IDL
 * @returns 凭证程序IDL
 */
export const getCredentialProgramIdl = () => {
  return CREDENTIAL_IDL;
};

/**
 * 创建Solana连接
 * @returns Solana连接对象
 */
export const createSolanaConnection = (): Connection => {
  return new Connection(SOLANA_RPC_URL, "confirmed");
};
