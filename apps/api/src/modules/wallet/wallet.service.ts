import { Injectable } from '@nestjs/common';
import { SignerClient } from 'src/clients/signer.client';

@Injectable()
export class WalletService {
  constructor(private signer: SignerClient) {}

  async createWallet() {
    return this.signer.createWallet();
  }

  async airdrop(pubkey: string) {
    return this.signer.airdrop(pubkey, 1_000_000_000);
  }

  async balance(pubkey: string) {
    return this.signer.getBalance(pubkey);
  }
}
