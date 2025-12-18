# Solana Kit Client Generator

This project generates type-safe TypeScript client code from your Solana Anchor IDL (Interface Definition Language) file, making it easy to interact with your Solana programs from the frontend using `@solana/kit`.

## Why Generate TypeScript from IDL?

### The Problem

When building a frontend application that interacts with Solana programs, you need to:

1. **Manually construct instructions** - You must manually encode instruction data, account metadata, and discriminators
2. **Handle account serialization/deserialization** - Manually decode account data from on-chain state
3. **Manage type safety** - No TypeScript types for your program's accounts, instructions, or errors
4. **Maintain synchronization** - When your program changes, you must manually update all frontend code
5. **Handle PDAs** - Manually derive Program Derived Addresses with correct seeds

This leads to:

- ❌ Error-prone code (wrong account order, incorrect data encoding)
- ❌ Runtime errors that could be caught at compile time
- ❌ Time-consuming manual updates when programs change
- ❌ Difficult debugging and maintenance

### The Solution

By generating TypeScript code from your IDL file, you get:

- ✅ **Type Safety** - Full TypeScript types for all accounts, instructions, and errors
- ✅ **Auto-completion** - IDE autocomplete for all program methods and accounts
- ✅ **Compile-time Validation** - Catch errors before runtime
- ✅ **Automatic Synchronization** - Regenerate code when your program changes
- ✅ **Simplified API** - Clean, easy-to-use functions for each instruction
- ✅ **PDA Derivation** - Built-in helpers for deriving Program Derived Addresses
- ✅ **Error Handling** - Type-safe error handling with proper error types

## What Gets Generated?

The generator creates a complete TypeScript client with:

- **Instructions** (`src/generated/instructions/`) - Type-safe functions for each program instruction (e.g., `deposit`, `withdraw`, `initialize`)
- **Accounts** (`src/generated/accounts/`) - Type-safe account types and deserialization helpers
- **Errors** (`src/generated/errors/`) - Type-safe error types matching your program's error codes
- **Programs** (`src/generated/programs/`) - Program address and metadata

## Prerequisites

- Node.js (v18 or higher)
- pnpm (v10.22.0 or higher) - This project uses pnpm as the package manager
- An Anchor IDL file (`idl.json`) in the project root

## Installation

1. **Install dependencies:**

```bash
pnpm install
```

## Usage

### Step 1: Place Your IDL File

Ensure your Anchor IDL file is named `idl.json` and placed in the project root directory:

```
solana-kit-client/
├── idl.json          ← Your Anchor IDL file goes here
├── package.json
└── ...
```

### Step 2: Generate TypeScript Client

Run the generation script:

```bash
pnpm generate
```

Or using npm:

```bash
npm run generate
```

This will:

1. Read your `idl.json` file
2. Generate type-safe TypeScript code using Codama
3. Output all generated files to `src/generated/`

### Step 3: Use Generated Code in Your Frontend

Import and use the generated code in your frontend application:

```typescript
import {
  getDepositInstruction,
  getUserVaultAccount,
  VAULT_PROGRAM_ADDRESS,
} from "./generated";

// Example: Build a deposit instruction
const depositInstruction = getDepositInstruction({
  amount: 1000000n, // lamports
  user: userPublicKey,
  // ... other accounts
});

// Example: Deserialize account data
const userVault = await getUserVaultAccount(connection, userVaultAddress);
```

## Example: Complete Integration

Here's a complete example of using the generated client:

```typescript
import { Connection, Keypair, Transaction } from "@solana/web3.js";
import {
  getDepositInstruction,
  getUserVaultAccount,
  getDepositInstructionAsync,
  VAULT_PROGRAM_ADDRESS,
} from "./generated";

async function depositToVault(
  connection: Connection,
  user: Keypair,
  amount: bigint
) {
  // Derive the user vault PDA (handled automatically by generated code)
  const [userVault] = await getDepositInstructionAsync({
    amount,
    user: user.publicKey,
  });

  // Get the instruction
  const instruction = await getDepositInstructionAsync({
    amount,
    user: user.publicKey,
  });

  // Build and send transaction
  const transaction = new Transaction().add(instruction);
  const signature = await connection.sendTransaction(transaction, [user]);

  return signature;
}

async function fetchUserVault(
  connection: Connection,
  userPublicKey: PublicKey
) {
  // Deserialize account data with full type safety
  const userVault = await getUserVaultAccount(connection, userVaultAddress);
  return userVault;
}
```

## Regenerating After Program Changes

Whenever you update your Solana program and generate a new IDL:

1. **Replace the IDL file:**

   ```bash
   # Copy your new IDL file to the project root
   cp /path/to/new/idl.json ./idl.json
   ```

2. **Regenerate the client:**

   ```bash
   pnpm generate
   ```

3. **Update your code** - TypeScript will show you any breaking changes at compile time!

## Project Structure

```
solana-kit-client/
├── idl.json                    # Your Anchor IDL file
├── scripts/
│   └── generate.ts            # Generation script
├── src/
│   └── generated/              # Generated TypeScript code (DO NOT EDIT)
│       ├── accounts/          # Account types and deserializers
│       ├── errors/            # Error types
│       ├── instructions/      # Instruction builders
│       ├── programs/          # Program metadata
│       └── shared/            # Shared utilities
└── package.json
```

## Important Notes

⚠️ **Never edit files in `src/generated/`** - They are auto-generated and will be overwritten when you run `pnpm generate`.

✅ **Always regenerate** after updating your program's IDL to keep your frontend in sync.

## Troubleshooting

### "Cannot find module 'idl.json'"

- Ensure `idl.json` exists in the project root
- Check that the file is valid JSON

### Type errors after regeneration

- This is expected! The generated types have changed to match your updated program
- Review the TypeScript errors and update your code accordingly
- This is the power of type safety - catching breaking changes at compile time!

### Generation fails

- Verify your IDL file is valid Anchor IDL format
- Check that all dependencies are installed: `pnpm install`

## Technologies Used

- **[Codama](https://github.com/codama-idl/codama)** - IDL code generation framework
- **[@solana/kit](https://github.com/solana-labs/solana-kit)** - Type-safe Solana SDK
- **[@codama/nodes-from-anchor](https://github.com/codama-idl/codama)** - Anchor IDL parser
- **[@codama/renderers-js](https://github.com/codama-idl/codama)** - JavaScript/TypeScript renderer

## License

ISC
