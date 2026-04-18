import { Injectable } from '@nestjs/common';
import { SignerClient } from 'src/clients/signer.client';

@Injectable()
export class WalletService {
  constructor(private signer: SignerClient) {}

  async createWallet() {
    return this.signer.createWallet();
  }

  async getAccount(walletID: string, accountIndex: number) {
    return this.signer.getAccount(walletID, accountIndex);
  }

  async listAccounts(walletID: string) {
    return this.signer.listAccounts(walletID);
  }

  async airdrop(pubkey: string) {
    return this.signer.airdrop(pubkey, 1_000_000_000);
  }

  async balance(pubkey: string) {
    return this.signer.getBalance(pubkey);
  }
}
