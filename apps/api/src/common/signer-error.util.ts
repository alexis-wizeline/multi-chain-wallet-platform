import {
  BadGatewayException,
  BadRequestException,
  NotAcceptableException,
} from '@nestjs/common';
import { mapSimulationReason } from './simulation-reason.util';

type SignerErrorResponse = {
  error?: {
    code?: string;
    message?: string;
    details?: Record<string, unknown>;
  };
};

export function throwMappedSignerError(
  status: number,
  body: SignerErrorResponse | string,
): never {
  const parsed =
    typeof body === 'string'
      ? {
          error: {
            code: 'UNKNOWN_ERROR',
            message: body,
          },
        }
      : body;

  const code = parsed.error?.code ?? 'UNKNOWN_ERROR';
  const message = parsed.error?.message ?? 'An unknown signer error occurred';
  const details = parsed.error?.details ?? {};

  const normalizedDetails =
    code === 'SIMULATION_FAILED'
      ? {
          ...details,
          reason: mapSimulationReason(
            typeof details.reason === 'string' ? details.reason : undefined,
            Array.isArray(details.logs)
              ? (details.logs as string[])
              : undefined,
          ),
        }
      : details;

  const payload = {
    error: {
      code,
      message,
      details: normalizedDetails,
    },
  };

  if (status === 404) {
    throw new NotAcceptableException(payload);
  }

  if (status >= 400 && status < 500) {
    throw new BadRequestException(payload);
  }

  throw new BadGatewayException(payload);
}
