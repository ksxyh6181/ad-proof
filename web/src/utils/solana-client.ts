import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import {
  getCredentialProgramIdl,
  CREDENTIAL_PROGRAM_ID,
  REGISTRY_PUBLIC_KEY
} from "@/config/solana";
import { debug } from "@/api/credential";

// 简单钱包接口，用于和Anchor交互
interface Wallet {
  publicKey: PublicKey;
  signTransaction(tx: any): Promise<any>;
  signAllTransactions(txs: any[]): Promise<any[]>;
}

// 类型导入 - 在项目中创建这些类型定义


/**
 * Solana凭证客户端
 * 处理与Solana区块链上凭证程序的交互
 */
export class SolanaCredentialClient {
  private program: Program<any>;
  private provider: anchor.AnchorProvider;
  private registryPubkey: PublicKey;
  private wallet: Wallet;

  /**
   * 初始化Solana凭证客户端
   * @param connection Solana连接实例
   * @param wallet 用户钱包
   */
  constructor(connection: anchor.web3.Connection, wallet: Wallet) {
    debug.log('SOLANA', '初始化Solana凭证客户端', {
      endpoint: connection.rpcEndpoint,
      programId: CREDENTIAL_PROGRAM_ID.toString(),
      registryId: REGISTRY_PUBLIC_KEY.toString()
    });

    // 创建Anchor提供者
    this.provider = new anchor.AnchorProvider(
      connection,
      wallet,
      { commitment: "confirmed" }
    );

    // 加载程序IDL并创建程序
    const idl = getCredentialProgramIdl();
    debug.log('SOLANA', '加载凭证程序IDL成功', {
      instructions: (idl as any).instructions?.map((i: any) => i.name)
    });

    this.program = new Program<any>(
      idl as any,
      CREDENTIAL_PROGRAM_ID,
      this.provider
    );

    // 设置注册表公钥
    this.registryPubkey = new PublicKey(REGISTRY_PUBLIC_KEY);
    this.wallet = wallet;
  }

  /**
   * 注册新凭证
   * @param hash 凭证哈希
   * @param credentialType 凭证类型
   * @param issuer 颁发者
   * @param issueTimestamp 颁发时间戳
   * @param metadataUri 元数据URI
   * @returns 交易签名
   */
  async registerCredential(
    hash: string,
    credentialType: string,
    issuer: string,
    issueTimestamp: number,
    metadataUri: string
  ): Promise<string> {
    // 格式化哈希值 - 确保长度合适并移除任何前缀（如0x）
    const formattedHash = this.formatHash(hash);
    debug.log('SOLANA', '开始注册凭证', {
      hash,
      formattedHash,
      credentialType,
      issuer,
      issueTimestamp,
      metadataUri
    });

    // 计算注册表的当前计数
    const registry: any = await this.program.account.registry.fetch(this.registryPubkey);
    const currentCount = registry.credentialCount;

    // 创建计数Buffer作为种子
    const countBuffer = new Uint8Array(8);
    new anchor.BN(currentCount).toBuffer().copy(countBuffer);

    // 找到凭证PDA
    const [credentialPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        this.registryPubkey.toBuffer(),
        countBuffer,
      ],
      this.program.programId
    );

    debug.log('SOLANA', '计算凭证PDA地址', {
      pda: credentialPda.toString(),
      seeds: ["credential", this.registryPubkey.toString(), formattedHash]
    });

    // 发送注册凭证交易
    try {
      const tx = await this.program.methods
        .registerCredential(
          formattedHash,
          credentialType,
          issuer,
          new anchor.BN(issueTimestamp),
          metadataUri
        )
        .accounts({
          registry: this.registryPubkey,
          credential: credentialPda,
          authority: this.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      debug.log('SOLANA', '凭证注册交易成功', {
        signature: tx,
        credentialPda: credentialPda.toString()
      });

      return tx;
    } catch (e: any) {
      const error = e as any;
      debug.log('ERROR', '凭证注册交易失败', {
        error: error.message,
        logs: error.logs,
        code: error.code
      });
      throw error;
    }
  }

  /**
   * 验证凭证
   * @param hash 凭证哈希
   * @returns 验证是否成功
   */
  async verifyCredential(hash: string): Promise<boolean> {
    // 格式化哈希值
    const formattedHash = this.formatHash(hash);
    debug.log('SOLANA', '开始验证凭证', { hash, formattedHash });

    try {
      // 计算凭证PDA
      const [credentialPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("credential"),
          this.registryPubkey.toBuffer(),
          Buffer.from(formattedHash)
        ],
        this.program.programId
      );

      debug.log('SOLANA', '计算凭证PDA地址', {
        pda: credentialPda.toString()
      });

      // 从链上获取凭证信息
      const credentialInfo: any = await this.program.account.credential.fetch(credentialPda);
      debug.log('SOLANA', '成功获取凭证信息', {
        hash: credentialInfo.hash,
        revoked: credentialInfo.revoked
      });

      // 检查凭证是否已被撤销
      return !credentialInfo.revoked;
    } catch (error) {
      // 如果账户不存在，捕获错误
      if ((error as any).message.includes("Account does not exist") ||
        (error as any).message.includes("Not found")) {
        debug.log('SOLANA', '凭证不存在', { hash });
        return false;
      }

      debug.log('ERROR', '验证凭证失败', {
        error: (error as any).message,
        code: (error as any).code
      });
      throw error;
    }
  }

  /**
   * 撤销凭证
   * @param hash 凭证哈希
   * @returns 交易签名
   */
  async revokeCredential(hash: string): Promise<string> {
    // 格式化哈希值
    const formattedHash = this.formatHash(hash);
    debug.log('SOLANA', '开始撤销凭证', { hash, formattedHash });

    // 计算凭证PDA
    const [credentialPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        this.registryPubkey.toBuffer(),
        Buffer.from(formattedHash)
      ],
      this.program.programId
    );

    // 发送撤销凭证交易
    try {
      const tx = await this.program.methods
        .revokeCredential()
        .accounts({
          registry: this.registryPubkey,
          credential: credentialPda,
          authority: this.wallet.publicKey,
        })
        .rpc();

      debug.log('SOLANA', '凭证撤销成功', { signature: tx });
      return tx;
    } catch (error) {
      debug.log('ERROR', '撤销凭证失败', {
        error: (error as any).message,
        code: (error as any).code
      });
      throw error;
    }
  }

  /**
   * 更新凭证元数据
   * @param hash 凭证哈希
   * @param newMetadataUri 新的元数据URI
   * @returns 交易签名
   */
  async updateMetadata(hash: string, newMetadataUri: string): Promise<string> {
    // 格式化哈希值
    const formattedHash = this.formatHash(hash);
    debug.log('SOLANA', '开始更新凭证元数据', {
      hash,
      formattedHash,
      newMetadataUri
    });

    // 计算凭证PDA
    const [credentialPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("credential"),
        this.registryPubkey.toBuffer(),
        Buffer.from(formattedHash)
      ],
      this.program.programId
    );

    // 发送更新元数据交易
    try {
      const tx = await this.program.methods
        .updateMetadata(newMetadataUri)
        .accounts({
          registry: this.registryPubkey,
          credential: credentialPda,
          authority: this.wallet.publicKey,
        })
        .rpc();

      debug.log('SOLANA', '凭证元数据更新成功', { signature: tx });
      return tx;
    } catch (e: any) {
      const error = e as any;
      debug.log('ERROR', '更新凭证元数据失败', {
        error: error.message,
        code: error.code
      });
      throw error;
    }
  }

  /**
   * 获取凭证信息
   * @param hash 凭证哈希
   * @returns 凭证信息对象，如果不存在则返回null
   */
  async getCredential(hash: string): Promise<any | null> {
    // 格式化哈希值
    const formattedHash = this.formatHash(hash);
    debug.log('SOLANA', '开始获取凭证信息', { hash, formattedHash });

    try {
      // 计算凭证PDA
      const [credentialPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("credential"),
          this.registryPubkey.toBuffer(),
          Buffer.from(formattedHash)
        ],
        this.program.programId
      );

      debug.log('SOLANA', '计算凭证PDA地址', {
        pda: credentialPda.toString()
      });

      // 从链上获取凭证信息
      const credentialInfo: any = await this.program.account.credential.fetch(credentialPda);

      // 转换BN为数字
      const issueDate = credentialInfo.issueDate.toNumber();

      const credential = {
        hash: credentialInfo.hash,
        credentialType: credentialInfo.credentialType,
        issuer: credentialInfo.issuer,
        issueDate: issueDate,
        metadataUri: credentialInfo.metadataUri,
        revoked: credentialInfo.revoked,
        owner: credentialInfo.authority,
      };

      debug.log('SOLANA', '成功获取凭证信息', credential);
      return credential;
    } catch (e: any) {
      const error = e as any;
      // 如果账户不存在，返回null
      if ((error as any).message?.includes("Account does not exist") ||
        (error as any).message?.includes("Not found")) {
        debug.log('SOLANA', '凭证不存在', { hash });
        return null;
      }

      debug.log('ERROR', '获取凭证信息失败', {
        error: error.message,
        code: error.code
      });
      throw error;
    }
  }

  /**
   * 格式化哈希值，确保格式适合Solana程序
   * @param hash 原始哈希字符串
   * @returns 格式化后的哈希
   */
  private formatHash(hash: string): string {
    // 移除可能存在的0x前缀
    let formattedHash = hash.toLowerCase().startsWith('0x')
      ? hash.slice(2)
      : hash;

    // 如果哈希太长，截断它
    if (formattedHash.length > 64) {
      debug.log('SOLANA', '哈希长度超过64个字符，将被截断', {
        original: formattedHash,
        length: formattedHash.length
      });
      formattedHash = formattedHash.substring(0, 64);
    }

    return formattedHash;
  }
}
