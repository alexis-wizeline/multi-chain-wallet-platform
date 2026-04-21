import {
  BadGatewayException,
  Injectable,
  NotFoundException,
} from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { throwMappedSignerError } from 'src/common/signer-error.util';

type SolTransferIntent = {
  kind: 'sol_transfer';
  from: string;
  to: string;
  lamports: number;
};

type SignAndSendTxPayload = {
  walletID: string;
  accountIndex: number;
  serializedTx: string;
  intent: SolTransferIntent;
};

type AccountResponse = {
  index: number;
  pubkey: string;
};

@Injectable()
export class SignerClient {
  private baseUrl: string;

  constructor(private configService: ConfigService) {
    this.baseUrl =
      this.configService.get<string>('SIGNER_SERVICE_URL') ||
      'http://localhost:3000';
  }

  private async parseResponse<T>(res: Response): Promise<T> {
    const contentTye = res.headers.get('Content-Type') ?? '';
    const isJson = contentTye.includes('application/json');

    const body = isJson ? await res.json() : await res.text();

    if (!res.ok) {
      throwMappedSignerError(res.status, body);
    }

    return body as T;
  }

  async createWallet(): Promise<{ walletID: string }> {
    const res = await fetch(`${this.baseUrl}/wallets`, {
      method: 'POST',
    });

    return this.parseResponse<{ walletID: string }>(res);
  }

  async getAccount(
    walletID: string,
    accountIndex: number,
  ): Promise<AccountResponse> {
    const res = await fetch(
      `${this.baseUrl}/wallets/${walletID}/${accountIndex}`,
      {
        method: 'GET',
      },
    );

    return this.parseResponse<AccountResponse>(res);
  }

  async listAccounts(walletID: string): Promise<AccountResponse[]> {
    const res = await fetch(`${this.baseUrl}/wallets/${walletID}`, {
      method: 'GET',
    });

    return this.parseResponse<AccountResponse[]>(res);
  }

  async signAndSendTx(
    payload: SignAndSendTxPayload,
  ): Promise<{ signature: string }> {
    const res = await fetch(`${this.baseUrl}/sign/transaction`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        wallet_id: payload.walletID,
        account_index: payload.accountIndex,
        serialized_tx: payload.serializedTx,
        intent: payload.intent,
      }),
    });

    return this.parseResponse<{ signature: string }>(res);
  }

  async airdrop(
    pubkey: string,
    lamports: number,
  ): Promise<{ signature: string }> {
    const res = await fetch(`${this.baseUrl}/devnet/airdrop`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ pubkey, lamports }),
    });

    return this.parseResponse<{ signature: string }>(res);
  }

  async getBalance(pubkey: string): Promise<{ balance: number }> {
    const res = await fetch(`${this.baseUrl}/devnet/balance/${pubkey}`, {
      method: 'GET',
    });

    return this.parseResponse<{ balance: number }>(res);
  }
}
