import { Controller, Get, Param, Post } from '@nestjs/common';
import { WalletService } from './wallet.service';

@Controller('wallets')
export class WalletController {
  constructor(private service: WalletService) {}

  @Post()
  create() {
    return this.service.createWallet();
  }

  @Post(':pubkey/airdrop')
  airdrop(@Param('pubkey') pubkey: string) {
    return this.service.airdrop(pubkey);
  }

  @Get(':pubkey/balance')
  balance(@Param('pubkey') pubkey: string) {
    return this.service.balance(pubkey);
  }
}
