import { Injectable } from '@nestjs/common';

@Injectable()
export class SignerClient {
  private baseUrl: string;

  constructor() {
    this.baseUrl = process.env.SIGNER_SERVICE_URL || 'http://localhost:3000';
  }

  async createWallet(): Promise<{ walletID: string }> {
    const res = await fetch(`${this.baseUrl}/wallets`, {
      method: 'POST',
    });

    return res.json() as Promise<{ walletID: string }>;
  }

  async signAndSendTx(payload: {
    walletID: string;
    accountIndex: number;
    serializedTx: string;
  }): Promise<{ signature: string }> {
    const res = await fetch(`${this.baseUrl}/sign/transaction`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        wallet_id: payload.walletID,
        account_index: payload.accountIndex,
        serialized_tx: payload.serializedTx,
      }),
    });

    return res.json() as Promise<{ signature: string }>;
  }

  async airdrop(
    pubkey: string,
    lamports: number,
  ): Promise<{ signature: string }> {
    const res = await fetch(`${this.baseUrl}/devnet/airdrop`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ pubkey, lamports }),
    });

    return res.json() as Promise<{ signature: string }>;
  }

  async getBalance(pubkey: string): Promise<{ balance: number }> {
    const res = await fetch(`${this.baseUrl}/devnet/balance/${pubkey}`, {
      method: 'GET',
    });

    return res.json() as Promise<{ balance: number }>;
  }
}
