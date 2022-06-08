# Explanation of the wrapped EGLD hack (2022/06/06)

On 2022/06/06, a hacker stole 1.650M EGLD from the wrapped EGLD smart contracts ([450,000 EGLD on shard 0](https://explorer.elrond.com/transactions/39998ab5c929fa67e95d0c64081697fc4207235dbfeaaff10fb2704a6c7716b6), [800,000 EGLD on shard 1](https://explorer.elrond.com/transactions/41effd8536376f3a2edba7074c02776edae94bb5b464485ac414847202eebbe2), [400,000 EGLD on shard 2](https://explorer.elrond.com/transactions/8b8c332577e5b8bdd4e13450ea92b7c6b0ca15399f1f0bb38fc215cfc3ddb490)). This hack has been fixed by the Elrond team.

If you want to understand how we discovered how the exploit was possible and reproduce it exactly, go check [the reproduction repository](https://github.com/arda-org/wrapped-egld-hack-reproduction).

With this repository, you will be able to understand how the exploit works, thanks to a simpler version of the hacker's smart contract.

## Repository overview

This repository contains the following files:

- `wrapper.wasm`: the compiled code of the wrapped EGLD smart contract ([available here](https://explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqhe8t5jewej70zupmh44jurgn29psua5l2jps3ntjj3/code))
- `src/lib.rs`: the source code of a smart contract exploiting the wrapped EGLD smart contract
- `mandos/hack.scen.json`: a scenario that exploits the wrapped EGLD smart contract. You can see that at the end of the scenario, the hacker has successfully withdrawn 800,000 EGLD from the wrapped EGLD smart contract.

## Explanation

The simpler version of the hacker's smart contract in `src/lib.rs` contains 2 methods:

- `wrapEgld`: it sends some EGLD to the wrapped EGLD smart contract and asks it to send back the wrapped EGLD to the hacker smart contract by calling its `wrap_egld_callback` method,
- `wrap_egld_callback`: this method is called by the wrapped EGLD smart contract. It uses `execute_on_dest_context_by_caller_raw` to make the caller (i.e. the wrapped EGLD smart contract) sends a large amount of the EGLD it owns to the hacker smart contract.

Here is how to test the smart contract in `src/lib.rs`:

1. Install `erdpy` (cf. [tutorial](https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/))
2. Clone this repository
3. Open a terminal in the repository directory
4. Build the smart contract: `erdpy contract build`
5. Test the smart contract: `erdpy contract test`

The scenario in `mandos/hack.scen.json` should succeed. You can see that at the end of the scenario, the hacker has successfully withdrawn 800,000 EGLD from the wrapped EGLD smart contract.
