use solana_client::rpc_response::transaction::CompiledInstruction;
use solana_sdk::{message::Instruction, pubkey::Pubkey, transaction::Transaction};
use solana_system_interface::{instruction as system_instruction, program as system_program};

use crate::error::AppError;

pub fn validate_transaction(tx: &Transaction, signer_pubkey: &Pubkey) -> Result<(), AppError> {
    validate_fee_payer(tx, signer_pubkey)?;
    validate_signer_present(tx, signer_pubkey)?;
    validate_allowed_programs(tx)?;
    validate_supported_instructions(tx, signer_pubkey)?;
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

fn validate_supported_instructions(
    tx: &Transaction,
    signer_pubkey: &Pubkey,
) -> Result<(), AppError> {
    for ix in &tx.message.instructions {
        let program_id = get_program_id(tx, ix)?;

        if program_id == system_program::id() {
            validate_system_instruction(tx, ix, signer_pubkey)?;
        } else {
            return Err(AppError::ValidationFailed(format!(
                "unsupported program {}",
                program_id
            )));
        }
    }

    Ok(())
}

fn validate_system_instruction(
    tx: &Transaction,
    ix: &CompiledInstruction,
    signer_pubkey: &Pubkey,
) -> Result<(), AppError> {
    let instruction = decode_system_instruction(tx, ix)?;

    match instruction {
        SystemInstructionShape::Transfer { from, .. } => {
            if &from != signer_pubkey {
                return Err(AppError::ValidationFailed(format!(
                    "transfer source {} does not match signer {}",
                    from, signer_pubkey
                )));
            }

            Ok(())
        }
    }
}

fn get_program_id(tx: &Transaction, ix: &CompiledInstruction) -> Result<Pubkey, AppError> {
    tx.message
        .account_keys
        .get(ix.program_id_index as usize)
        .copied()
        .ok_or_else(|| AppError::ValidationFailed("invalid program id index".into()))
}

enum SystemInstructionShape {
    Transfer { from: Pubkey },
}

fn decode_system_instruction(
    tx: &Transaction,
    ix: &CompiledInstruction,
) -> Result<SystemInstructionShape, AppError> {
    let program_id = get_program_id(tx, ix)?;
    if program_id != system_program::id() {
        return Err(AppError::ValidationFailed(
            "instruction is not a system program instruction".into(),
        ));
    }

    let account_metas = ix
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
        Ok(system_instruction::SystemInstruction::Transfer { lamports: _ }) => {
            if account_metas.len() < 2 {
                return Err(AppError::ValidationFailed(
                    "transfer requires at least 2 accounts".into(),
                ));
            }

            Ok(SystemInstructionShape::Transfer {
                from: account_metas[0],
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
