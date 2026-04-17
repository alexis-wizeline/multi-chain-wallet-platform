import { IsString, IsNumber } from 'class-validator';

export class TransferDto {
  @IsString()
  wallet_id: string;
  @IsNumber()
  account_index: number;
  @IsString()
  to: string;
  @IsNumber()
  lamports: number;
}
