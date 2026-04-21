export type ApiErrorCode =
  | 'SIMULATION_ERROR'
  | 'VALIDATION_ERROR'
  | 'INTERNAL_ERROR'
  | 'INVALID_REQUEST'
  | 'WALLET_NOT_FOUND'
  | 'ACCOUNT_NOT_FOUND'
  | 'RPC_ERROR'
  | 'INTERNAL_ERROR'
  | 'UNKNOWN_ERROR';

export interface ApiErrorPayload {
  error: {
    code: ApiErrorCode;
    message: string;
    details?: Record<string, unknown>;
  };
}
