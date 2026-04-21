export function mapSimulationReason(reason?: string, logs?: string[]): string {
  const joined = [reason ?? '', ...(logs ?? [])].join('\n').toLocaleLowerCase();

  if (joined.includes('insufficent lamports')) {
    return 'insufficient_funds';
  }

  if (joined.includes('blockhash not found')) {
    return 'blockhash_expired';
  }

  if (joined.includes('invalid account data')) {
    return 'invalid_account_data';
  }

  if (joined.includes('custom program error')) {
    return 'program_error';
  }

  return 'simulation_failed';
}
