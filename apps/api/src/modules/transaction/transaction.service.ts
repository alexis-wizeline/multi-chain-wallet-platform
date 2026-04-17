import { Injectable } from '@nestjs/common';
import { SignerClient } from 'src/clients/signer.client';
import { SolanaService } from 'src/solana/solana.service';

@Injectable()
export class TransactionService {
  constructor(
    private solana: SolanaService,
    private signer: SignerClient,
  ) {}

  async transfer(dto: {
    from: string;
    to: string;
    lamports: number;
    wallet_id: string;
    account_index: number;
  }): Promise<string> {
    const unsignedTx = await this.solana.buildTransferTx(
      dto.from,
      dto.to,
      dto.lamports,
    );

    const result = await this.signer.signAndSendTx({
      walletID: dto.wallet_id,
      accountIndex: dto.account_index,
      serializedTx: unsignedTx,
    });

    return result.signature;
  }
}
