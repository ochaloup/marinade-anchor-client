# anchor-client call to Marinade state

With refactoring work on the Marinade liquid-staking-program, the `anchor-client` calls
are possible as the anchor version was updated to 0.27.0.
See work in progressa at branch: https://github.com/marinade-finance/liquid-staking-program/tree/anchor-0.27

## To build&run

```bash
# mainnet
cargo run
# devnet
cargo run -- devnet
```

## Troubleshooting

When the new Solana version (like `1.16.0`)
is downloaded as dependency instead of the predefined `1.14.18`
then the build of `anchor-lang` fails with the following error
(maybe connected to https://stackoverflow.com/q/76213582).

```
error[E0277]: the trait bound `Pubkey: BorshDeserialize` is not satisfied
  --> /home/chalda/.cargo/registry/src/github.com-1ecc6299db9ec823/anchor-lang-0.27.0/src/idl.rs:35:27
   |
35 | #[derive(AnchorSerialize, AnchorDeserialize)]
   |                           ^^^^^^^^^^^^^^^^^ the trait `BorshDeserialize` is not implemented for `Pubkey`
   |
   = help: the following other types implement trait `BorshDeserialize`:
             ()
             (T0, T1)
             (T0, T1, T2)
             (T0, T1, T2, T3)
             (T0, T1, T2, T3, T4)
             (T0, T1, T2, T3, T4, T5)
             (T0, T1, T2, T3, T4, T5, T6)
             (T0, T1, T2, T3, T4, T5, T6, T7)
           and 101 others
   = help: see issue #48214
   = note: this error originates in the derive macro `AnchorDeserialize` (in Nightly builds, run with -Z macro-backtrace for more info)
```
