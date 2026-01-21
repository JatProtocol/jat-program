// SEAL Announcer: the on-chain index for stealth payments.
//
// Why this program exists: Solana does not retain transaction logs, so a payer
// cannot "emit and forget" the ephemeral public key R that a recipient needs to
// recover a stealth address. R must be written to durable account state. Each
// announcement is its OWN PDA seeded by R (which is unique per payment), so
// concurrent payers never contend on a shared writable account: no serialization,
// no timing side channel from a global ring buffer. A recipient (or an untrusted
// indexer they delegate the scan key to) enumerates announcements with
// getProgramAccounts filtered by data size, then runs the 1-byte view-tag
// prefilter locally. The announcer learns nothing: R and the view tag are public
// by construction and reveal no link to the recipient without the scan key.
//
// This program holds no funds and has no authority. announce() is permissionless.
use anchor_lang::prelude::*;

declare_id!("seaWHA64tVzN8yfa33bE6cvqKRSxVp3R6c7Ts5NXPM9");

#[program]
pub mod announcer {
    use super::*;

    /// Publish a stealth announcement. Called by the payer, normally in the SAME
    /// transaction as the lamport transfer that funds the derived stealth address,
    /// so funding and announcement commit atomically.
    ///
    /// `r`         the ephemeral public key R = r*B (32 bytes, compressed Edwards)
    /// `view_tag`  first byte of H(ECDH shared secret); lets a scanner reject
    ///             ~255/256 of foreign announcements with one byte, no point math
    /// `scheme`    versioning byte for the derivation scheme (0 = Ed25519 dual-key)
    pub fn announce(ctx: Context<Announce>, r: [u8; 32], view_tag: u8, scheme: u8) -> Result<()> {
        let a = &mut ctx.accounts.announcement;
        a.r = r;
        a.view_tag = view_tag;
        a.scheme = scheme;
        a.slot = Clock::get()?.slot;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(r: [u8; 32])]
pub struct Announce<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // PDA seeded by R. `init` (not init_if_needed) makes a duplicate R a hard
    // error: R is a fresh ephemeral key every payment, so a collision means a
    // buggy or malicious client, and we refuse rather than overwrite.
    #[account(
        init,
        payer = payer,
        space = Announcement::SPACE,
        seeds = [b"ann", r.as_ref()],
        bump,
    )]
    pub announcement: Account<'info, Announcement>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Announcement {
    pub r: [u8; 32],
    pub view_tag: u8,
    pub scheme: u8,
    pub slot: u64,
}

impl Announcement {
    // 8 discriminator + 32 R + 1 view_tag + 1 scheme + 8 slot
    pub const SPACE: usize = 8 + 32 + 1 + 1 + 8;
}
