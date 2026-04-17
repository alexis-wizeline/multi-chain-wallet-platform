import { Module } from '@nestjs/common';
import { WalletModule } from './modules/wallet/wallet.module';
import { TransactionModule } from './modules/transaction/transaction.module';
import { SignerClient } from './clients/signer.client';
import { ConfigModule } from '@nestjs/config';
import { SolanaService } from './solana/solana.service';

@Module({
  imports: [
    WalletModule,
    TransactionModule,
    ConfigModule.forRoot({ isGlobal: true }),
  ],
  controllers: [],
  providers: [SignerClient, SolanaService],
  exports: [SignerClient, SolanaService],
})
export class AppModule {}
