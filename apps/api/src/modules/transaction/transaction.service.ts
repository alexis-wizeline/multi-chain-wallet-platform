import { Injectable } from '@nestjs/common';
import { SignerClient } from 'src/clients/signer.client';
import { TransferDto } from 'src/dto/transfer.dto';
import { SolanaService } from 'src/solana/solana.service';

@Injectable()
export class TransactionService {
  constructor(
    private solana: SolanaService,
    private signer: SignerClient,
  ) {}

  async transfer(dto: TransferDto): Promise<string> {
    const account = await this.signer.getAccount(
      dto.wallet_id,
      dto.account_index,
    );

    const unsignedTx = await this.solana.buildTransferTx(
      account.pubkey,
      dto.to,
      dto.lamports,
    );

    const result = await this.signer.signAndSendTx({
      walletID: dto.wallet_id,
      accountIndex: dto.account_index,
      serializedTx: unsignedTx,
      intent: {
        kind: 'sol_transfer',
        from: account.pubkey,
        to: dto.to,
        lamports: dto.lamports,
      },
    });

    return result.signature;
  }
}
