import { Body, Controller, Post } from '@nestjs/common';
import { TransactionService } from './transaction.service';
import { TransferDto } from 'src/dto/transfer.dto';

@Controller('transaction')
export class TransactionController {
  constructor(private service: TransactionService) {}

  @Post('transfer')
  async trnsfer(@Body() dto: TransferDto) {
    return this.service.transfer(dto);
  }
}
