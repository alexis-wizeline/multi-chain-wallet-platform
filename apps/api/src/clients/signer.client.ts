import {
  BadGatewayException,
  Injectable,
  NotFoundException,
} from '@nestjs/common';
import { ConfigService } from '@nestjs/config';

type SignAndSendTxPayload = {
  walletID: string;
  accountIndex: number;
  serializedTx: string;
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

  async createWallet(): Promise<{ walletID: string }> {
    const res = await fetch(`${this.baseUrl}/wallets`, {
      method: 'POST',
    });

    if (!res.ok) {
      throw new BadGatewayException(
        'Failed to create wallet in signer service',
      );
    }

    return res.json() as Promise<{ walletID: string }>;
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

    if (res.status === 404) {
      throw new NotFoundException('Account not found in signer service');
    }

    if (!res.ok) {
      throw new BadGatewayException(
        'failed to get account from signer service',
      );
    }

    return res.json() as Promise<AccountResponse>;
  }

  async listAccounts(walletID: string): Promise<AccountResponse[]> {
    const res = await fetch(`${this.baseUrl}/wallets/${walletID}`, {
      method: 'GET',
    });

    if (res.status === 404) {
      throw new NotFoundException('Wallet not found in signer service');
    }

    if (!res.ok) {
      throw new BadGatewayException(
        'Failed to list accounst in signer service',
      );
    }

    return res.json() as Promise<AccountResponse[]>;
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
      }),
    });

    if (!res.ok) {
      throw new BadGatewayException(
        `Signer service failed to sing/send transaction ${res.status} ${res.statusText}`,
      );
    }

    return res.json() as Promise<{ signature: string }>;
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

    if (!res.ok) {
      throw new BadGatewayException(
        `Signer service failed to airdrop ${res.status} ${res.statusText}`,
      );
    }

    return res.json() as Promise<{ signature: string }>;
  }

  async getBalance(pubkey: string): Promise<{ balance: number }> {
    const res = await fetch(`${this.baseUrl}/devnet/balance/${pubkey}`, {
      method: 'GET',
    });

    if (!res.ok) {
      throw new BadGatewayException(
        `Signer service failed to get balance ${res.status} ${res.statusText}`,
      );
    }

    return res.json() as Promise<{ balance: number }>;
  }
}
