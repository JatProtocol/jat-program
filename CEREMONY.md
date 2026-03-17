# SEAL trusted-setup ceremony

The Groth16 proving keys currently shipped (`circuits/*_final.zkey`) are an
initial setup, fine for devnet. Before mainnet, run this multi-party phase-2
ceremony so the keys come from independent contributors. Groth16 phase-2 is
sound as long as at least one contributor is honest and destroys their entropy,
which is exactly what a multi-party run guarantees.

This is per circuit: `seal` (the gate) and `withdraw`.

## 1. Coordinator: initialize

```bash
# phase-1 (powers of tau) is circuit-independent; pot14 covers these circuits.
snarkjs groth16 setup circuits/seal.r1cs     circuits/pot14_final.ptau seal_0000.zkey
snarkjs groth16 setup circuits/withdraw.r1cs circuits/pot14_final.ptau withdraw_0000.zkey
```

## 2. Each contributor (independent machines, private entropy)

Pass the zkey from one contributor to the next. Every contributor publishes
their contribution hash so anyone can confirm they are in the chain.

```bash
node scripts/contribute.mjs seal_0000.zkey seal_alice.zkey "alice"
# alice -> bob -> carol -> ...
node scripts/contribute.mjs seal_alice.zkey seal_bob.zkey "bob"
```

## 3. Coordinator: finalize with a public beacon

A verifiable public randomness beacon (e.g. a future Bitcoin block hash, or a
drand round) removes any doubt about the last contributor.

```bash
snarkjs zkey beacon seal_bob.zkey seal_final.zkey <BEACON_HEX> 10 -n="final beacon"
snarkjs zkey export verificationkey seal_final.zkey circuits/seal_vkey.json
```

## 4. Anyone: verify the whole chain

```bash
snarkjs zkey verify circuits/seal.r1cs circuits/pot14_final.ptau seal_final.zkey
# prints each contributor's hash; confirms the key derives from this r1cs + ptau
```

## 5. Bake the verifying key into the program and redeploy

```bash
node scripts/vk_to_rust.mjs circuits/seal_vkey.json     <out> SEAL_VK
node scripts/vk_to_rust.mjs circuits/withdraw_vkey.json <out> WITHDRAW_VK
# rebuild + deploy (SBPFv3, see DESIGN_stealth.md), then publish the final zkeys
# and the contribution transcript so integrators can verify what they prove against.
```

Until steps 1-5 are done with independent contributors, treat every proof as
dev-grade and keep the deployment on devnet.
