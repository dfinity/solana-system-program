//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct InitializeNonceAccount {
    pub nonce_account: solana_program::pubkey::Pubkey,

    pub recent_blockhashes_sysvar: solana_program::pubkey::Pubkey,

    pub rent_sysvar: solana_program::pubkey::Pubkey,
}

impl InitializeNonceAccount {
    pub fn instruction(
        &self,
        args: InitializeNonceAccountInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeNonceAccountInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.nonce_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.recent_blockhashes_sysvar,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent_sysvar,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeNonceAccountInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SYSTEM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeNonceAccountInstructionData {
    discriminator: u32,
}

impl InitializeNonceAccountInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 6 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeNonceAccountInstructionArgs {
    pub nonce_authority: Pubkey,
}

/// Instruction builder for `InitializeNonceAccount`.
///
/// ### Accounts:
///
///   0. `[writable]` nonce_account
///   1. `[optional]` recent_blockhashes_sysvar (default to `SysvarRecentB1ockHashes11111111111111111111`)
///   2. `[optional]` rent_sysvar (default to `SysvarRent111111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeNonceAccountBuilder {
    nonce_account: Option<solana_program::pubkey::Pubkey>,
    recent_blockhashes_sysvar: Option<solana_program::pubkey::Pubkey>,
    rent_sysvar: Option<solana_program::pubkey::Pubkey>,
    nonce_authority: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeNonceAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn nonce_account(&mut self, nonce_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.nonce_account = Some(nonce_account);
        self
    }
    /// `[optional account, default to 'SysvarRecentB1ockHashes11111111111111111111']`
    #[inline(always)]
    pub fn recent_blockhashes_sysvar(
        &mut self,
        recent_blockhashes_sysvar: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.recent_blockhashes_sysvar = Some(recent_blockhashes_sysvar);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent_sysvar(&mut self, rent_sysvar: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent_sysvar = Some(rent_sysvar);
        self
    }
    #[inline(always)]
    pub fn nonce_authority(&mut self, nonce_authority: Pubkey) -> &mut Self {
        self.nonce_authority = Some(nonce_authority);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitializeNonceAccount {
            nonce_account: self.nonce_account.expect("nonce_account is not set"),
            recent_blockhashes_sysvar: self.recent_blockhashes_sysvar.unwrap_or(
                solana_program::pubkey!("SysvarRecentB1ockHashes11111111111111111111"),
            ),
            rent_sysvar: self.rent_sysvar.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
        };
        let args = InitializeNonceAccountInstructionArgs {
            nonce_authority: self
                .nonce_authority
                .clone()
                .expect("nonce_authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_nonce_account` CPI accounts.
pub struct InitializeNonceAccountCpiAccounts<'a, 'b> {
    pub nonce_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub recent_blockhashes_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_nonce_account` CPI instruction.
pub struct InitializeNonceAccountCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub nonce_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub recent_blockhashes_sysvar: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeNonceAccountInstructionArgs,
}

impl<'a, 'b> InitializeNonceAccountCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeNonceAccountCpiAccounts<'a, 'b>,
        args: InitializeNonceAccountInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            nonce_account: accounts.nonce_account,
            recent_blockhashes_sysvar: accounts.recent_blockhashes_sysvar,
            rent_sysvar: accounts.rent_sysvar,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.nonce_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.recent_blockhashes_sysvar.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent_sysvar.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeNonceAccountInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SYSTEM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.nonce_account.clone());
        account_infos.push(self.recent_blockhashes_sysvar.clone());
        account_infos.push(self.rent_sysvar.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitializeNonceAccount` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` nonce_account
///   1. `[]` recent_blockhashes_sysvar
///   2. `[]` rent_sysvar
#[derive(Clone, Debug)]
pub struct InitializeNonceAccountCpiBuilder<'a, 'b> {
    instruction: Box<InitializeNonceAccountCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeNonceAccountCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeNonceAccountCpiBuilderInstruction {
            __program: program,
            nonce_account: None,
            recent_blockhashes_sysvar: None,
            rent_sysvar: None,
            nonce_authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn nonce_account(
        &mut self,
        nonce_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.nonce_account = Some(nonce_account);
        self
    }
    #[inline(always)]
    pub fn recent_blockhashes_sysvar(
        &mut self,
        recent_blockhashes_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.recent_blockhashes_sysvar = Some(recent_blockhashes_sysvar);
        self
    }
    #[inline(always)]
    pub fn rent_sysvar(
        &mut self,
        rent_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.rent_sysvar = Some(rent_sysvar);
        self
    }
    #[inline(always)]
    pub fn nonce_authority(&mut self, nonce_authority: Pubkey) -> &mut Self {
        self.instruction.nonce_authority = Some(nonce_authority);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = InitializeNonceAccountInstructionArgs {
            nonce_authority: self
                .instruction
                .nonce_authority
                .clone()
                .expect("nonce_authority is not set"),
        };
        let instruction = InitializeNonceAccountCpi {
            __program: self.instruction.__program,

            nonce_account: self
                .instruction
                .nonce_account
                .expect("nonce_account is not set"),

            recent_blockhashes_sysvar: self
                .instruction
                .recent_blockhashes_sysvar
                .expect("recent_blockhashes_sysvar is not set"),

            rent_sysvar: self
                .instruction
                .rent_sysvar
                .expect("rent_sysvar is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeNonceAccountCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    nonce_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recent_blockhashes_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    nonce_authority: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
