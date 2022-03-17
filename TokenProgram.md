# Token program analysis

> NOTE: I am going to inspect the 2022 version.
> [https://github.com/solana-labs/solana-program-library/tree/master/token/program-2022]

The token program is in charge of defining custom tokens that can represent fungible coins or NFTs. This gives a common
interface across the Solana environment to easy the interoperability between dApps.

A token can belong to a single owner or have many through a multisig account.

- **Address**: `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`. Not yet published.

## On-chain data

The program has two different states depending on the type of the account:

- Mint:
    - Mint authority
    - Total token supply
    - Token decimals
    - Whether it is initialized or not.
    - (Optional) The freeze authority.
- Token Account:
    - Mint
    - Owner
    - Amount of tokens
    - The state of the account (Uninitialized, Initialized, Frozen)
    - Whether it is a native account or not. A native account contains an equivalent amount of SOL and wrapped SOL
      tokens.
    - (Optional) A close authority.
    - (Optional) A delegate authority.
    - (Optional) A delegate token amount.
- Multisig:
    - The total number of signers. (Max 11)
    - The required number of signers to sign a transaction.
    - Whether it is initialized or not.
    - The list of signers.

## Features

The program allows a user to:

1. Create a mint, i.e. a token definition.
2. Create an account of a defined mint.
3. Create a multisig account.
4. Approve a delegate account to operate over part or the total of the tokens in the account.
5. Revoke the delegate.
6. Change the authority of an account.
7. Mint tokens into an account, i.e. increasing the total supply of the token.
8. Transfer tokens between accounts.
9. Burn tokens, i.e. reducing the total supply of the token.
10. Freeze an account preventing any further modification.
11. Thaw an account to allow new modifications again.
12. Close an account.

## Improvements

The new token program defines extensions over `Mint` and `Token Account` accounts through extensions:

- **Confidential transactions**: to send transactions between two accounts without publicly showing the transferred
  amount.
- **Immutable owner**: to prevent changing the owner of the account.
- **Transfer fee**: to charge a fee whenever the token is exchanged.

## Business

Almost all the projects in Solana use their own tokens, either fungible (Mango, Audius, Radium, etc.) or NFTs (ThugDAO,
MonkeDAO, ect.).

> Note: these uses are for the previous token version. But the 2022 cover the same requirements and more.

## Potential issues / Attacking vectors

The most important attack vector I can see without inspect the code in deep is Scamming. An example of this can be,
trusting a website that says that do something but instead asks you to approve a transaction in which they have the
right to steals your tokens and SOL, in short empty your wallet.

# Stats

There are some pages where you can see some stats of the program:

- [https://dashboard.chaincrunch.cc/public/dashboard/4c539ba3-50e2-4085-9441-8de0884077e4?token_names=KIN&token_names=Serum&token_names=Raydium&token_names=Orca&token_names=USD%20Coin&token_names=Mango&token_names=Saber%20Protocol%20Token#theme=night]
- [https://solscan.io/tokens]