import { Module } from '@nestjs/common';
import { WalletController } from './wallet.controller';
import { WalletService } from './wallet.service';
import { SignerClient } from 'src/clients/signer.client';

@Module({
  controllers: [WalletController],
  providers: [WalletService, SignerClient],
})
export class WalletModule {}
