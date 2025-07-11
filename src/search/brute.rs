use rayon::prelude::*;

use crate::search::heuristics::{candidate_indexes, candidate_queue};

use crate::bip39::{checksum::entropy_to_mnemonic, seed::derive_seed};
use crate::shamir::reconstruct::attempt_reconstruction;
use crate::utils::hexify;

// Compile-time check: this code assumes a little-endian target
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("brute search assumes a little-endian target");
};

/// Try to brute-force the third share given two known shares.
///
/// `share_a` and `share_b` must be equal length and contain the share index in
/// their first byte. The remaining bytes are the share payload. `max_depth`
/// controls how many candidate payloads will be enumerated (little-endian base
/// 256 counter). The expected `zpub` is used to verify recovered mnemonics.
use crate::agents::codex_researcher::CodexResearcher;
use crate::search::diff::zpub_diff;
use crate::ui; // why: track global search metrics

pub fn brute_force_third_share(
    share_a: &[u8],
    share_b: &[u8],
    expected_zpub: &str,
    max_depth: usize,
    log: Option<&CodexResearcher>,
    prng: &crate::entropy::prng::PrngSettings,
    index_collision_prob: f64,
    show_progress: bool,
) -> Option<(Vec<u8>, String)> {
    assert_eq!(share_a.len(), share_b.len(), "share length mismatch");
    assert!(share_a.len() > 1, "shares must include payload");
    assert!(share_a[0] != share_b[0], "duplicate share indexes");

    assert!((0.0..=1.0).contains(&index_collision_prob), "probability out of range");

    let payload_len = share_a.len() - 1;

    let heuristics_first = candidate_queue(share_a, share_b, max_depth, 256, prng);
    let idx_candidates = candidate_indexes(share_a[0], share_b[0], index_collision_prob);

    let stats = ui::global(); // why: collect metrics during brute force

    let mut best_similarity = 0.0f64;
    if show_progress {
        for data in heuristics_first {
            for &idx in idx_candidates.iter() {
                if index_collision_prob == 0.0 && (idx == share_a[0] || idx == share_b[0]) {
                    continue;
                }

                let mut share_c = Vec::with_capacity(payload_len + 1);
                share_c.push(idx);
                share_c.extend_from_slice(&data);

                stats.inc_candidates(1); // why: count each tested candidate
                if let Ok(secret) = attempt_reconstruction(share_a, share_b, &share_c) {
                    if secret.len() != 16 {
                        continue;
                    }
                    if let Ok(words) = entropy_to_mnemonic(&secret) {
                        let mnemonic = words.join(" ");
                        let cand_z = crate::bip39::seed::derive_seed_zpub(&mnemonic).unwrap_or_default();
                        let diff = zpub_diff(&cand_z, expected_zpub);
                        stats.update_best(diff.prefix_len as u64, &cand_z[..diff.prefix_len.min(cand_z.len())]);
                        if diff.similarity > best_similarity {
                            best_similarity = diff.similarity;
                            println!("[Progress] best similarity {:.4}", best_similarity);
                        }
                        if let Ok(true) = derive_seed(&mnemonic, expected_zpub) {
                            stats.inc_matches(1);
                            println!(
                                "[+] Found candidate idx={} payload={} mnemonic={}",
                                idx,
                                hexify(&data),
                                mnemonic
                            );
                            return Some((share_c, mnemonic));
                        } else if let Some(r) = log {
                            let path_str = format!("{}/{}/{}", share_a[0], share_b[0], idx);
                            let _ = r.record_attempt(&mnemonic, &cand_z, expected_zpub, &secret, &path_str, diff);
                        }
                    }
                }
            }
        }
    } else if let Some(found) = heuristics_first.into_par_iter().find_map_any(|data| {
        for &idx in idx_candidates.iter() {
            if index_collision_prob == 0.0 && (idx == share_a[0] || idx == share_b[0]) {
                continue;
            }

            let mut share_c = Vec::with_capacity(payload_len + 1);
            share_c.push(idx);
            share_c.extend_from_slice(&data);

            stats.inc_candidates(1); // why: parallel candidate count
            if let Ok(secret) = attempt_reconstruction(share_a, share_b, &share_c) {
                if secret.len() != 16 {
                    continue;
                }
                if let Ok(words) = entropy_to_mnemonic(&secret) {
                    let mnemonic = words.join(" ");
                    let cand_z = crate::bip39::seed::derive_seed_zpub(&mnemonic).unwrap_or_default();
                    let diff = zpub_diff(&cand_z, expected_zpub);
                    stats.update_best(diff.prefix_len as u64, &cand_z[..diff.prefix_len.min(cand_z.len())]);
                    if let Ok(true) = derive_seed(&mnemonic, expected_zpub) {
                        stats.inc_matches(1);
                        println!(
                            "[+] Found candidate idx={} payload={} mnemonic={}",
                            idx,
                            hexify(&data),
                            mnemonic
                        );
                        return Some((share_c, mnemonic));
                    } else if let Some(r) = log {
                        let path_str = format!("{}/{}/{}", share_a[0], share_b[0], idx);
                        let _ = r.record_attempt(&mnemonic, &cand_z, expected_zpub, &secret, &path_str, diff);
                    }
                }
            }
        }
        None
    }) {
        return Some(found);
    }

    if show_progress {
        for candidate in 0..max_depth {
            // why: enumerate candidate payloads as a little-endian counter
            let mut data = vec![0u8; payload_len];
            let mut tmp = candidate;
            for b in data.iter_mut() {
                *b = (tmp & 0xff) as u8;
                tmp >>= 8;
            }

            for &idx in idx_candidates.iter() {
                if index_collision_prob == 0.0 && (idx == share_a[0] || idx == share_b[0]) {
                    continue;
                }

                let mut share_c = Vec::with_capacity(payload_len + 1);
                share_c.push(idx);
                share_c.extend_from_slice(&data);

                stats.inc_candidates(1);
                if let Ok(secret) = attempt_reconstruction(share_a, share_b, &share_c) {
                    if secret.len() != 16 {
                        continue;
                    }

                    if let Ok(words) = entropy_to_mnemonic(&secret) {
                        let mnemonic = words.join(" ");

                        let cand_z = crate::bip39::seed::derive_seed_zpub(&mnemonic).unwrap_or_default();
                        let diff = zpub_diff(&cand_z, expected_zpub);
                        stats.update_best(diff.prefix_len as u64, &cand_z[..diff.prefix_len.min(cand_z.len())]);
                        if diff.similarity > best_similarity {
                            best_similarity = diff.similarity;
                            println!("[Progress] best similarity {:.4}", best_similarity);
                        }
                        if let Ok(true) = derive_seed(&mnemonic, expected_zpub) {
                            stats.inc_matches(1);
                            println!(
                                "[+] Found candidate idx={} payload={} mnemonic={}",
                                idx,
                                hexify(&data),
                                mnemonic
                            );
                            return Some((share_c, mnemonic));
                        } else if let Some(r) = log {
                            let path_str = format!("{}/{}/{}", share_a[0], share_b[0], idx);
                            let _ = r.record_attempt(&mnemonic, &cand_z, expected_zpub, &secret, &path_str, diff);
                        }
                    }
                }
            }
        }
        None
    } else {
        (0..max_depth).into_par_iter().find_map_any(|candidate| {
            // why: enumerate candidate payloads as a little-endian counter
            let mut data = vec![0u8; payload_len];
            let mut tmp = candidate;
            for b in data.iter_mut() {
                *b = (tmp & 0xff) as u8;
                tmp >>= 8;
            }

            for &idx in idx_candidates.iter() {
                if index_collision_prob == 0.0 && (idx == share_a[0] || idx == share_b[0]) {
                    continue;
                }

                let mut share_c = Vec::with_capacity(payload_len + 1);
                share_c.push(idx);
                share_c.extend_from_slice(&data);

                stats.inc_candidates(1);
                if let Ok(secret) = attempt_reconstruction(share_a, share_b, &share_c) {
                    // why: only 16-byte secrets map to 12-word mnemonics
                    if secret.len() != 16 {
                        continue;
                    }

                    if let Ok(words) = entropy_to_mnemonic(&secret) {
                        let mnemonic = words.join(" ");

                        let cand_z = crate::bip39::seed::derive_seed_zpub(&mnemonic).unwrap_or_default();
                        let diff = zpub_diff(&cand_z, expected_zpub);
                        stats.update_best(diff.prefix_len as u64, &cand_z[..diff.prefix_len.min(cand_z.len())]);
                        if let Ok(true) = derive_seed(&mnemonic, expected_zpub) {
                            stats.inc_matches(1);
                            println!(
                                "[+] Found candidate idx={} payload={} mnemonic={}",
                                idx,
                                hexify(&data),
                                mnemonic
                            );
                            return Some((share_c, mnemonic));
                        } else if let Some(r) = log {
                            let path_str = format!("{}/{}/{}", share_a[0], share_b[0], idx);
                            let _ = r.record_attempt(&mnemonic, &cand_z, expected_zpub, &secret, &path_str, diff);
                        }
                    }
                }
            }
            None
        })
    }
}

/// Placeholder wrapper used by the CLI. Real parameters will be wired up later.
pub fn start_brute_force() {
    println!("[Search] Brute-force mode initiated...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bip39::seed::derive_seed_zpub;
    use crate::shamir::gf256::{gf_add, gf_mul};

    fn make_share(secret: &[u8], a: &[u8], b: &[u8], idx: u8) -> Vec<u8> {
        let mut out = Vec::with_capacity(secret.len() + 1);
        out.push(idx);
        for i in 0..secret.len() {
            let mut y = secret[i];
            y = gf_add(y, gf_mul(a[i], idx));
            let idx_sq = gf_mul(idx, idx);
            y = gf_add(y, gf_mul(b[i], idx_sq));
            out.push(y);
        }
        out
    }

    #[test]
    fn test_brute_force_finds_share() {
        let _ = crate::ui::init_global(); // why: required for stats
        let secret = [0u8; 16];
        let a = [0u8; 16];
        let b = [0u8; 16];

        let s1 = make_share(&secret, &a, &b, 1);
        let s2 = make_share(&secret, &a, &b, 2);
        let s3 = make_share(&secret, &a, &b, 3);

        let words = entropy_to_mnemonic(&secret).unwrap();
        let mnemonic = words.join(" ");
        let zpub = derive_seed_zpub(&mnemonic).unwrap();

        let candidate_num: usize = 0; // all zero payload

        let (found_share, found_mnemonic) = brute_force_third_share(
            &s1,
            &s2,
            &zpub,
            candidate_num + 1,
            None,
            &crate::entropy::prng::PrngSettings::default(),
            0.0,
            false,
        )
        .expect("should find share");

        assert_eq!(found_share, s3);
        assert_eq!(found_mnemonic, mnemonic);
    }
}
