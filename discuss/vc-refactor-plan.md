# ad-proof Refactor Plan

## Goal

Shrink `ad-proof` from a broad ZK credential playground into a small, explainable selective disclosure demo with real verifier questions.

This refactor keeps two scenarios:

1. Income threshold proof
2. KYC level proof

The project becomes a learning-oriented MVP demo, not a general-purpose credential platform.

## Product Positioning

`ad-proof` is a selective disclosure VC demo where a holder proves eligibility to a verifier without revealing the full underlying credential.

The demo focuses on:

- `income >= threshold_tier`
- `kyc_level >= required_level`

It does not try to be:

- a generic academic credential platform
- a personhood or agent binding system
- a chain-first registry product

## Roles

### Issuer

- Issues a credential
- Signs the credential header
- Defines expiry and status

### Holder

- Holds the credential
- Generates a presentation for a specific verifier request
- Reveals only the minimum public metadata plus a predicate proof

### Verifier

- Checks issuer trust
- Checks expiry and status
- Verifies that the requested predicate is true
- Does not learn the exact income or raw KYC payload

## Retained Scenarios

### Scenario A: Income Threshold Proof

Verifier wants to know:

- Is this applicant above the required income tier?
- Was the credential issued by a trusted financial institution?
- Is the credential still valid?

Verifier must not learn:

- Exact salary
- Full account or identity payload

### Scenario B: KYC Level Proof

Verifier wants to know:

- Has this user reached the required KYC assurance level?
- Was the credential issued by a trusted KYC issuer?
- Is the credential still valid?

Verifier must not learn:

- Full identity record
- Raw KYC evidence

## Explicitly Removed or Downgraded

- Education credential system as a primary narrative
- Credit score and cross-border financial flows
- Personhood and agent binding modules
- Hash-equality-as-proof logic as the core ZK story
- Solana registry as a product pillar

Solana, if kept at all, is optional future work for status anchoring only.

## System Design

## Credential Header

Public, signed metadata:

- `credential_id`
- `kind`
- `issuer_id`
- `subject_ref`
- `issued_at`
- `expires_at`
- `status`

## Private Claim Payload

### Income

- `income_tier`

### KYC

- `kyc_level`

For the MVP demo, tiered claims are acceptable because they let the verifier ask a real business question without learning the exact value.

## Proof Semantics

ZK is used for predicate proofs, not for simple field-hash consistency.

The proof must represent one of:

- `income_tier >= required_tier`
- `kyc_level >= required_level`

The proof is bound to a concrete `credential_id` so a verifier can tie the presentation back to a signed credential header.

## Verification Checks

Verifier checks:

1. Credential header signature is valid
2. Issuer is trusted for this credential kind
3. Credential is not expired
4. Credential status is active
5. Predicate proof is valid for the requested threshold and the credential id

## Public vs Private

Public:

- credential id
- issuer id
- credential kind
- expiry
- status
- requested threshold
- proof bytes

Private:

- exact income payload
- raw KYC data
- holder-side subject details beyond `subject_ref`

## Code Refactor Plan

### Keep

- `server/crates/lib/zkp`
- `server/crates/bin/gateway`
- shared response and auth helpers where still useful

### Replace

- Replace `credential.rs`, `financial_zkp.rs`, `identity.rs` with a single `vc.rs`
- Replace the old generic `zkp.rs` with shared proof helpers only
- Replace separate credential and identity controllers with a single `vc_controller.rs`

### Remove from Main Flow

- education routes
- identity routes
- solana initialization
- solana client dependency from the main demo path

## API Shape After Refactor

- `POST /api/vc/income/issue`
- `POST /api/vc/income/present`
- `POST /api/vc/income/verify`
- `POST /api/vc/kyc/issue`
- `POST /api/vc/kyc/present`
- `POST /api/vc/kyc/verify`

## README Rewrite

README should explain:

1. the two retained verifier questions
2. the three roles
3. why selective disclosure is needed
4. why the demo uses threshold predicates instead of revealing raw fields
5. what is intentionally out of scope

## Implementation Notes

- This refactor prioritizes business clarity over protocol completeness
- The issuer signature can stay outside the ZK circuit in this MVP
- Revocation remains a simple status flag for now
- Supported income thresholds are tier-based in the MVP to keep the circuit small and explainable
