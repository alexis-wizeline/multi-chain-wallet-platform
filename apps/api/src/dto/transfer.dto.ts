import { IsString, IsNumber, Min } from 'class-validator';

export class TransferDto {
  @IsString()
  wallet_id: string;
  @IsNumber()
  @Min(0)
  account_index: number;
  @IsString()
  to: string;
  @IsNumber()
  @Min(1)
  lamports: number;
}
