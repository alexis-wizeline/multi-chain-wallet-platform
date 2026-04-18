import { Controller, Get, Param, Post } from '@nestjs/common';
import { WalletService } from './wallet.service';

@Controller('wallets')
export class WalletController {
  constructor(private service: WalletService) {}

  @Post()
  create() {
    return this.service.createWallet();
  }

  @Get(':walletID/accounts/:accountIndex')
  getAccount(
    @Param('walletID') walletID: string,
    @Param('accountIndex') accountIndex: number,
  ) {
    return this.service.getAccount(walletID, accountIndex);
  }

  @Get(':walleteID/accounts')
  listAccounts(@Param('walletID') walletID: string) {
    return this.service.listAccounts(walletID);
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
