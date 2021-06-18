# fake-ico

Spend your precious `ETH` to get some `💩`!

---

This is an onboarding task for Gnosis that implements an ICO for a Scam currency.

The contracts are currently deployed on rinkeby.
Use CLI to interact with `WETH`, `SCM` and `ICO`:

```shell
export ETH_TRANSPORT="..."
export ETH_PK="..."

# Interact with WETH:
cargo run -- weth balance  # your WETH balance
# Other subcommands are balance, transfer, 
# allowance, approve, deposit, withdraw

# Interact with SCM:
cargo run -- scm balance  # your WETH balance
# Other subcommands are balance, transfer, 
# allowance, approve, deposit, withdraw

# Interact with ICO:
cargo run -- ico info  # basic info about ICO
cargo run -- ico balance  # how many SCM tokens you can claim
cargo run -- ico fund 1eth --wrap-weth  # purchase SCM
cargo run -- ico claim --wait  # claim SCM after ICO is finished
```

Whenever CLI expects amount of money (i.e. `weth transfer` or `ico fund`),
you can supply a positive integer with an optional suffix such as `eth`, `wei`, `gwei`
for ether and `scm`, `asc` (atta-scam), `nsc` (nano-scam) for scam token.
CLI does not support floats at the moment.
