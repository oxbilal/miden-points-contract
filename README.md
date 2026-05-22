# Miden Points Smart Contract

Custom smart contract built and tested on Miden testnet.

## What I built

- `points-account`: stores points in account storage.
- `points-note`: calls the contract and increases points by 10.
- `points_flow.rs`: publishes and consumes the note.

## Testnet Proof

Points Account ID:
`0x922b676e96912b00499fce5fe98596`

Note publish transaction:
`0xccbd80227a6e57e610d14dc335d0283d8d9c8144d617040cc159a79040145777`

Consume transaction:
`0xfbfb7966a94aab6589313dcc1f56fa6063f86d649ac1c3f7497af0706f7139b0`

Result:
`Points were increased by 10 ✅`

---

## Badge Contract Proof

I also built and tested a second custom flow using a `badge-account` contract.

- `badge-account`: stores badge count in account storage.
- `badge-note`: calls the contract and increases badge count by 1.
- `badge_flow.rs`: publishes and consumes the note.

Badge Account ID:
`0xc54291249b095b00527055d8604c1b`

Badge note publish transaction:
`0x834934f19aa85031975a94610d578245334d641736dd2e540eef59dd8756583c`

Badge consume transaction:
`0x09f5a192d1139c1f68a78b5a806f3659769ef84e583a75f84547d6b89740c417`

Result:
`Badge count increased by 1 ✅`

---

## Level Contract Proof

I also built and tested a third custom flow using a `level-account` contract.

- `level-account`: stores XP in account storage.
- `level-note`: calls the contract and increases XP by 25.
- `level_flow.rs`: publishes and consumes the note.

Level Account ID:
`0xdade9a850995de0023145c84a1e0db`

Level note publish transaction:
`0x4d95fbb42377145de5a621524a316d107c771635b92760cb903685a36cf7216d`

Level consume transaction:
`0x9c7042379c9f6381e34214b5fa5afdf3d116828c78e7f706f39a43d5ee49ccf9`

Result:
`XP increased by 25 ✅`
