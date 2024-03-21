/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU32Decoder,
  getU32Encoder,
  getU64Decoder,
  getU64Encoder,
  mapEncoder,
} from '@solana/codecs';
import {
  IAccountMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
  ReadonlySignerAccount,
  WritableAccount,
} from '@solana/instructions';
import { IAccountSignerMeta, TransactionSigner } from '@solana/signers';
import { SYSTEM_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type WithdrawNonceAccountInstruction<
  TProgram extends string = typeof SYSTEM_PROGRAM_ADDRESS,
  TAccountNonceAccount extends string | IAccountMeta<string> = string,
  TAccountRecipientAccount extends string | IAccountMeta<string> = string,
  TAccountRecentBlockhashesSysvar extends
    | string
    | IAccountMeta<string> = 'SysvarRecentB1ockHashes11111111111111111111',
  TAccountRentSysvar extends
    | string
    | IAccountMeta<string> = 'SysvarRent111111111111111111111111111111111',
  TAccountNonceAuthority extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountNonceAccount extends string
        ? WritableAccount<TAccountNonceAccount>
        : TAccountNonceAccount,
      TAccountRecipientAccount extends string
        ? WritableAccount<TAccountRecipientAccount>
        : TAccountRecipientAccount,
      TAccountRecentBlockhashesSysvar extends string
        ? ReadonlyAccount<TAccountRecentBlockhashesSysvar>
        : TAccountRecentBlockhashesSysvar,
      TAccountRentSysvar extends string
        ? ReadonlyAccount<TAccountRentSysvar>
        : TAccountRentSysvar,
      TAccountNonceAuthority extends string
        ? ReadonlySignerAccount<TAccountNonceAuthority> &
            IAccountSignerMeta<TAccountNonceAuthority>
        : TAccountNonceAuthority,
      ...TRemainingAccounts,
    ]
  >;

export type WithdrawNonceAccountInstructionData = {
  discriminator: number;
  withdrawAmount: bigint;
};

export type WithdrawNonceAccountInstructionDataArgs = {
  withdrawAmount: number | bigint;
};

export function getWithdrawNonceAccountInstructionDataEncoder(): Encoder<WithdrawNonceAccountInstructionDataArgs> {
  return mapEncoder(
    getStructEncoder([
      ['discriminator', getU32Encoder()],
      ['withdrawAmount', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 5 })
  );
}

export function getWithdrawNonceAccountInstructionDataDecoder(): Decoder<WithdrawNonceAccountInstructionData> {
  return getStructDecoder([
    ['discriminator', getU32Decoder()],
    ['withdrawAmount', getU64Decoder()],
  ]);
}

export function getWithdrawNonceAccountInstructionDataCodec(): Codec<
  WithdrawNonceAccountInstructionDataArgs,
  WithdrawNonceAccountInstructionData
> {
  return combineCodec(
    getWithdrawNonceAccountInstructionDataEncoder(),
    getWithdrawNonceAccountInstructionDataDecoder()
  );
}

export type WithdrawNonceAccountInput<
  TAccountNonceAccount extends string = string,
  TAccountRecipientAccount extends string = string,
  TAccountRecentBlockhashesSysvar extends string = string,
  TAccountRentSysvar extends string = string,
  TAccountNonceAuthority extends string = string,
> = {
  nonceAccount: Address<TAccountNonceAccount>;
  recipientAccount: Address<TAccountRecipientAccount>;
  recentBlockhashesSysvar?: Address<TAccountRecentBlockhashesSysvar>;
  rentSysvar?: Address<TAccountRentSysvar>;
  nonceAuthority: TransactionSigner<TAccountNonceAuthority>;
  withdrawAmount: WithdrawNonceAccountInstructionDataArgs['withdrawAmount'];
};

export function getWithdrawNonceAccountInstruction<
  TAccountNonceAccount extends string,
  TAccountRecipientAccount extends string,
  TAccountRecentBlockhashesSysvar extends string,
  TAccountRentSysvar extends string,
  TAccountNonceAuthority extends string,
>(
  input: WithdrawNonceAccountInput<
    TAccountNonceAccount,
    TAccountRecipientAccount,
    TAccountRecentBlockhashesSysvar,
    TAccountRentSysvar,
    TAccountNonceAuthority
  >
): WithdrawNonceAccountInstruction<
  typeof SYSTEM_PROGRAM_ADDRESS,
  TAccountNonceAccount,
  TAccountRecipientAccount,
  TAccountRecentBlockhashesSysvar,
  TAccountRentSysvar,
  TAccountNonceAuthority
> {
  // Program address.
  const programAddress = SYSTEM_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    nonceAccount: { value: input.nonceAccount ?? null, isWritable: true },
    recipientAccount: {
      value: input.recipientAccount ?? null,
      isWritable: true,
    },
    recentBlockhashesSysvar: {
      value: input.recentBlockhashesSysvar ?? null,
      isWritable: false,
    },
    rentSysvar: { value: input.rentSysvar ?? null, isWritable: false },
    nonceAuthority: { value: input.nonceAuthority ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.recentBlockhashesSysvar.value) {
    accounts.recentBlockhashesSysvar.value =
      'SysvarRecentB1ockHashes11111111111111111111' as Address<'SysvarRecentB1ockHashes11111111111111111111'>;
  }
  if (!accounts.rentSysvar.value) {
    accounts.rentSysvar.value =
      'SysvarRent111111111111111111111111111111111' as Address<'SysvarRent111111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.nonceAccount),
      getAccountMeta(accounts.recipientAccount),
      getAccountMeta(accounts.recentBlockhashesSysvar),
      getAccountMeta(accounts.rentSysvar),
      getAccountMeta(accounts.nonceAuthority),
    ],
    programAddress,
    data: getWithdrawNonceAccountInstructionDataEncoder().encode(
      args as WithdrawNonceAccountInstructionDataArgs
    ),
  } as WithdrawNonceAccountInstruction<
    typeof SYSTEM_PROGRAM_ADDRESS,
    TAccountNonceAccount,
    TAccountRecipientAccount,
    TAccountRecentBlockhashesSysvar,
    TAccountRentSysvar,
    TAccountNonceAuthority
  >;

  return instruction;
}

export type ParsedWithdrawNonceAccountInstruction<
  TProgram extends string = typeof SYSTEM_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    nonceAccount: TAccountMetas[0];
    recipientAccount: TAccountMetas[1];
    recentBlockhashesSysvar: TAccountMetas[2];
    rentSysvar: TAccountMetas[3];
    nonceAuthority: TAccountMetas[4];
  };
  data: WithdrawNonceAccountInstructionData;
};

export function parseWithdrawNonceAccountInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedWithdrawNonceAccountInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 5) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      nonceAccount: getNextAccount(),
      recipientAccount: getNextAccount(),
      recentBlockhashesSysvar: getNextAccount(),
      rentSysvar: getNextAccount(),
      nonceAuthority: getNextAccount(),
    },
    data: getWithdrawNonceAccountInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
