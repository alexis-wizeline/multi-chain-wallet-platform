import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { getTransferSolInstruction } from '@solana-program/system';
import {
  address,
  appendTransactionMessageInstruction,
  compileTransaction,
  createNoopSigner,
  createSolanaRpc,
  createTransactionMessage,
  getBase64EncodedWireTransaction,
  pipe,
  Rpc,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  SolanaRpcApi,
} from '@solana/kit';

@Injectable()
export class SolanaService {
  rpc: Rpc<SolanaRpcApi>;

  constructor(private configService: ConfigService) {
    this.rpc = createSolanaRpc(
      this.configService.get<string>('RPC_SOLANA_URL') || '',
    );
  }

  async buildTransferTx(
    from: string,
    to: string,
    lamports: number,
  ): Promise<string> {
    const fromPubkey = address(from);
    const toPubkey = address(to);

    const { value: latestBlockhash } = await this.rpc
      .getLatestBlockhash()
      .send();

    const transferInstruction = getTransferSolInstruction({
      source: createNoopSigner(fromPubkey),
      destination: toPubkey,
      amount: BigInt(lamports),
    });

    const txMessage = pipe(
      createTransactionMessage({ version: 0 }),
      (tx) => setTransactionMessageFeePayer(fromPubkey, tx),
      (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
      (tx) => appendTransactionMessageInstruction(transferInstruction, tx),
    );

    const transaction = compileTransaction(txMessage);

    return getBase64EncodedWireTransaction(transaction);
  }
}
