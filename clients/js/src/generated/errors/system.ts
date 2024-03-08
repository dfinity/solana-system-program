/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

export const enum SystemProgramErrorCode {
  /** AccountAlreadyInUse: an account with the same address already exists */
  ACCOUNT_ALREADY_IN_USE = 0x0, // 0
  /** ResultWithNegativeLamports: account does not have enough SOL to perform the operation */
  RESULT_WITH_NEGATIVE_LAMPORTS = 0x1, // 1
  /** InvalidProgramId: cannot assign account to this program id */
  INVALID_PROGRAM_ID = 0x2, // 2
  /** InvalidAccountDataLength: cannot allocate account data of this length */
  INVALID_ACCOUNT_DATA_LENGTH = 0x3, // 3
  /** MaxSeedLengthExceeded: length of requested seed is too long */
  MAX_SEED_LENGTH_EXCEEDED = 0x4, // 4
  /** AddressWithSeedMismatch: provided address does not match addressed derived from seed */
  ADDRESS_WITH_SEED_MISMATCH = 0x5, // 5
  /** NonceNoRecentBlockhashes: advancing stored nonce requires a populated RecentBlockhashes sysvar */
  NONCE_NO_RECENT_BLOCKHASHES = 0x6, // 6
  /** NonceBlockhashNotExpired: stored nonce is still in recent_blockhashes */
  NONCE_BLOCKHASH_NOT_EXPIRED = 0x7, // 7
  /** NonceUnexpectedBlockhashValue: specified nonce does not match stored nonce */
  NONCE_UNEXPECTED_BLOCKHASH_VALUE = 0x8, // 8
}

export class SystemProgramError extends Error {
  override readonly name = 'SystemProgramError';

  readonly code: SystemProgramErrorCode;

  readonly cause: Error | undefined;

  constructor(
    code: SystemProgramErrorCode,
    name: string,
    message: string,
    cause?: Error
  ) {
    super(`${name} (${code}): ${message}`);
    this.code = code;
    this.cause = cause;
  }
}

let systemProgramErrorCodeMap:
  | Record<SystemProgramErrorCode, [string, string]>
  | undefined;
if (__DEV__) {
  systemProgramErrorCodeMap = {
    [SystemProgramErrorCode.ACCOUNT_ALREADY_IN_USE]: [
      'AccountAlreadyInUse',
      `an account with the same address already exists`,
    ],
    [SystemProgramErrorCode.RESULT_WITH_NEGATIVE_LAMPORTS]: [
      'ResultWithNegativeLamports',
      `account does not have enough SOL to perform the operation`,
    ],
    [SystemProgramErrorCode.INVALID_PROGRAM_ID]: [
      'InvalidProgramId',
      `cannot assign account to this program id`,
    ],
    [SystemProgramErrorCode.INVALID_ACCOUNT_DATA_LENGTH]: [
      'InvalidAccountDataLength',
      `cannot allocate account data of this length`,
    ],
    [SystemProgramErrorCode.MAX_SEED_LENGTH_EXCEEDED]: [
      'MaxSeedLengthExceeded',
      `length of requested seed is too long`,
    ],
    [SystemProgramErrorCode.ADDRESS_WITH_SEED_MISMATCH]: [
      'AddressWithSeedMismatch',
      `provided address does not match addressed derived from seed`,
    ],
    [SystemProgramErrorCode.NONCE_NO_RECENT_BLOCKHASHES]: [
      'NonceNoRecentBlockhashes',
      `advancing stored nonce requires a populated RecentBlockhashes sysvar`,
    ],
    [SystemProgramErrorCode.NONCE_BLOCKHASH_NOT_EXPIRED]: [
      'NonceBlockhashNotExpired',
      `stored nonce is still in recent_blockhashes`,
    ],
    [SystemProgramErrorCode.NONCE_UNEXPECTED_BLOCKHASH_VALUE]: [
      'NonceUnexpectedBlockhashValue',
      `specified nonce does not match stored nonce`,
    ],
  };
}

export function getSystemProgramErrorFromCode(
  code: SystemProgramErrorCode,
  cause?: Error
): SystemProgramError {
  if (__DEV__) {
    return new SystemProgramError(
      code,
      ...(
        systemProgramErrorCodeMap as Record<
          SystemProgramErrorCode,
          [string, string]
        >
      )[code],
      cause
    );
  }

  return new SystemProgramError(
    code,
    'Unknown',
    'Error message not available in production bundles. Compile with __DEV__ set to true to see more information.',
    cause
  );
}
