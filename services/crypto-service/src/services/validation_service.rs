use solana_client::rpc_response::transaction::CompiledInstruction;
use solana_sdk::{message::Instruction, pubkey::Pubkey, transaction::Transaction};
use solana_system_interface::{instruction as system_instruction, program as system_program};

use crate::{error::AppError, models::signing::SignIntent};

pub fn validate_transaction(
    tx: &Transaction,
    signer_pubkey: &Pubkey,
    intent: &SignIntent,
) -> Result<(), AppError> {
    validate_fee_payer(tx, signer_pubkey)?;
    validate_signer_present(tx, signer_pubkey)?;
    validate_allowed_programs(tx)?;

    match intent {
        SignIntent::SolTransfer { from, to, lamports } => {
            validate_sol_transfer_intent(tx, signer_pubkey, from, to, *lamports)?;
        }
    }

    Ok(())
}

fn validate_fee_payer(tx: &Transaction, signer_pubkey: &Pubkey) -> Result<(), AppError> {
    let fee_payer = tx
        .message
        .account_keys
        .first()
        .ok_or_else(|| AppError::ValidationFailed("transaction has no account keys".into()))?;

    if fee_payer != signer_pubkey {
        return Err(AppError::ValidationFailed(format!(
            "fee payer {} does not mathc signer {}",
            fee_payer, signer_pubkey
        )));
    }

    Ok(())
}

fn validate_signer_present(tx: &Transaction, signer_pubkey: &Pubkey) -> Result<(), AppError> {
    let signer_count = tx.message.header.num_required_signatures as usize;

    let signer_keys = &tx.message.account_keys[..signer_count];

    if !signer_keys.iter().any(|k| k == signer_pubkey) {
        return Err(AppError::ValidationFailed(format!(
            "signer {} is not in required signer set",
            signer_pubkey
        )));
    }

    Ok(())
}

fn validate_allowed_programs(tx: &Transaction) -> Result<(), AppError> {
    for ix in &tx.message.instructions {
        let program_id = get_program_id(tx, ix)?;

        if program_id != system_program::id() {
            return Err(AppError::ValidationFailed(format!(
                ":program {} is not allowed",
                program_id
            )));
        }
    }
    Ok(())
}

fn validate_sol_transfer_intent(
    tx: &Transaction,
    signer_pubkey: &Pubkey,
    expected_from: &str,
    expected_to: &str,
    expected_lamports: u64,
) -> Result<(), AppError> {
    if tx.message.instructions.len() != 1 {
        return Err(AppError::ValidationFailed(
            "Sol transfer require exactly one instruction".into(),
        ));
    }

    let expected_from_pubkey = expected_from
        .parse::<Pubkey>()
        .map_err(|e| AppError::InvalidRequest(format!("invalid intent.from pubkey: {}", e)))?;

    let expected_to_pubkey = expected_to
        .parse::<Pubkey>()
        .map_err(|e| AppError::InvalidRequest(format!("invalid intent.to pubkey: {}", e)))?;

    if &expected_from_pubkey != signer_pubkey {
        return Err(AppError::ValidationFailed(format!(
            "intent.from {} does not match signer {}",
            expected_to_pubkey, signer_pubkey
        )));
    }

    let parsed = parse_system_transfer(tx, &tx.message.instructions[0])?;

    if parsed.from != expected_from_pubkey {
        return Err(AppError::ValidationFailed(format!(
            "transaction from {} does not match intention.from {}",
            parsed.from, expected_from_pubkey
        )));
    }

    if parsed.to != expected_to_pubkey {
        return Err(AppError::ValidationFailed(format!(
            "transaction to {}, does not match intention.to {}",
            parsed.to, expected_to_pubkey
        )));
    }

    if parsed.lamports != expected_lamports {
        return Err(AppError::ValidationFailed(format!(
            "transaction lamports {} does not match intent.lamports {}",
            parsed.lamports, expected_lamports
        )));
    }

    Ok(())
}

fn get_program_id(tx: &Transaction, ix: &CompiledInstruction) -> Result<Pubkey, AppError> {
    tx.message
        .account_keys
        .get(ix.program_id_index as usize)
        .copied()
        .ok_or_else(|| AppError::ValidationFailed("invalid program id index".into()))
}

#[derive(Debug)]
struct ParseSystemTransfer {
    from: Pubkey,
    to: Pubkey,
    lamports: u64,
}

fn parse_system_transfer(
    tx: &Transaction,
    ix: &CompiledInstruction,
) -> Result<ParseSystemTransfer, AppError> {
    let program_id = get_program_id(tx, ix)?;
    if program_id != system_program::id() {
        return Err(AppError::ValidationFailed(
            "instruction is not a system program instruction".into(),
        ));
    }

    let accounts = ix
        .accounts
        .iter()
        .map(|idx| {
            tx.message
                .account_keys
                .get(*idx as usize)
                .copied()
                .ok_or_else(|| AppError::ValidationFailed("invalid account index".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let instruction = Instruction {
        program_id: program_id,
        accounts: vec![],
        data: ix.data.clone(),
    };

    match bincode::deserialize::<system_instruction::SystemInstruction>(&instruction.data) {
        Ok(system_instruction::SystemInstruction::Transfer { lamports }) => {
            if accounts.len() < 2 {
                return Err(AppError::ValidationFailed(
                    "transfer requires at least 2 accounts".into(),
                ));
            }

            Ok(ParseSystemTransfer {
                from: accounts[0],
                to: accounts[1],
                lamports,
            })
        }
        Ok(_) => Err(AppError::ValidationFailed(
            "only system transfer is currently supported".into(),
        )),
        Err(err) => Err(AppError::ValidationFailed(format!(
            "failed to decode system instruction: {}",
            err
        ))),
    }
}
