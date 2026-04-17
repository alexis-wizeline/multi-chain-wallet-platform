import { Module } from '@nestjs/common';
import { TransactionService } from './transaction.service';
import { SolanaService } from 'src/solana/solana.service';
import { SignerClient } from 'src/clients/signer.client';

@Module({
  providers: [TransactionService, SolanaService, SignerClient],
})
export class TransactionModule {}
