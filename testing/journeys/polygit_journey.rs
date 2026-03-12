use estream_test::{
    Journey, JourneyParty, JourneyStep, StepAction, JourneyMetrics,
    assert_metric_emitted, assert_blinded, assert_povc_witness,
};
use estream_test::convoy::{ConvoyContext, ConvoyResult};
use estream_test::stratum::{StratumVerifier, CsrTier, SeriesMerkleChain};
use estream_test::cortex::{CortexVisibility, RedactPolicy, ObfuscatePolicy};
use estream_test::es_git::{RepoGraph, CommitNode, MergeStrategy};

pub struct PolygitJourney;

impl Journey for PolygitJourney {
    fn name(&self) -> &str {
        "polygit_e2e"
    }

    fn description(&self) -> &str {
        "End-to-end journey for Polygit: repo creation, commits, code review, merge, graph verification"
    }

    fn parties(&self) -> Vec<JourneyParty> {
        vec![
            JourneyParty::new("alice")
                .with_spark_context("poly-git-v1")
                .with_role("maintainer"),
            JourneyParty::new("bob")
                .with_spark_context("poly-git-v1")
                .with_role("contributor"),
            JourneyParty::new("charlie")
                .with_spark_context("poly-git-v1")
                .with_role("reviewer"),
        ]
    }

    fn steps(&self) -> Vec<JourneyStep> {
        vec![
            // Step 1: Alice creates a new repository
            JourneyStep::new("alice_creates_repo")
                .party("alice")
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let repo = ctx.polygit().create_repo(
                        "quantum-lib",
                        &["bob", "charlie"],
                        "ml-dsa-87",
                    )?;

                    ctx.set("repo_id", &repo.id);
                    ctx.set("repo_root_hash", &repo.root_commit_hash);

                    assert!(repo.pq_signed);
                    assert_eq!(repo.signature_algo, "ml-dsa-87");
                    assert_eq!(repo.collaborators.len(), 2);

                    assert_metric_emitted!(ctx, "polygit.repo.created", {
                        "sig_algo" => "ml-dsa-87",
                        "collaborator_count" => "2",
                    });

                    assert_povc_witness!(ctx, "polygit.repo.create", {
                        witness_type: "repo_genesis",
                        repo_id: &repo.id,
                    });

                    Ok(())
                }))
                .timeout_ms(8_000),

            // Step 2: Bob pushes a PQ-signed commit
            JourneyStep::new("bob_pushes_commit")
                .party("bob")
                .depends_on(&["alice_creates_repo"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let repo_id = ctx.get::<String>("repo_id");

                    let tree = ctx.polygit().stage_files(&repo_id, &[
                        ("src/lib.rs", b"pub fn lattice_mul() {}"),
                        ("Cargo.toml", b"[package]\nname = \"quantum-lib\""),
                    ])?;

                    let commit = ctx.polygit().commit(
                        &repo_id,
                        &tree.tree_hash,
                        "feat: add lattice multiplication stub",
                    )?;

                    ctx.set("bob_commit_hash", &commit.hash);

                    assert!(commit.signature_valid);
                    assert_eq!(commit.signature_algo, "ml-dsa-87");
                    assert!(commit.parent_hash.is_some());

                    let push = ctx.polygit().push(&repo_id, "main", &commit.hash)?;
                    assert!(push.accepted);

                    assert_metric_emitted!(ctx, "polygit.commit.pushed", {
                        "branch" => "main",
                        "signed" => "true",
                    });

                    assert_blinded!(ctx, "polygit.commit.pushed", {
                        field: "author_id",
                        blinding: "hmac_sha3",
                    });

                    Ok(())
                }))
                .timeout_ms(10_000),

            // Step 3: Charlie submits a code review
            JourneyStep::new("charlie_submits_review")
                .party("charlie")
                .depends_on(&["bob_pushes_commit"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let repo_id = ctx.get::<String>("repo_id");
                    let commit_hash = ctx.get::<String>("bob_commit_hash");

                    let review = ctx.polygit().submit_review(
                        &repo_id,
                        &commit_hash,
                        "approve",
                        "LGTM — lattice stub looks correct",
                    )?;

                    ctx.set("review_id", &review.id);

                    assert!(review.pq_signed);
                    assert_eq!(review.verdict, "approve");

                    assert_povc_witness!(ctx, "polygit.review", {
                        witness_type: "code_review",
                        repo_id: &repo_id,
                        commit_hash: &commit_hash,
                    });

                    assert_metric_emitted!(ctx, "polygit.review.submitted", {
                        "verdict" => "approve",
                    });

                    assert_blinded!(ctx, "polygit.review.submitted", {
                        field: "reviewer_id",
                        blinding: "hmac_sha3",
                    });

                    Ok(())
                }))
                .timeout_ms(8_000),

            // Step 4: Alice merges the commit into main
            JourneyStep::new("alice_merges")
                .party("alice")
                .depends_on(&["charlie_submits_review"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let repo_id = ctx.get::<String>("repo_id");
                    let commit_hash = ctx.get::<String>("bob_commit_hash");

                    let merge = ctx.polygit().merge(
                        &repo_id,
                        "main",
                        &commit_hash,
                        MergeStrategy::FastForward,
                    )?;

                    ctx.set("merge_commit_hash", &merge.commit_hash);

                    assert!(merge.success);
                    assert!(merge.signature_valid);
                    assert_eq!(merge.strategy, MergeStrategy::FastForward);

                    assert_metric_emitted!(ctx, "polygit.merge.complete", {
                        "branch" => "main",
                        "strategy" => "fast_forward",
                    });

                    assert_povc_witness!(ctx, "polygit.merge", {
                        witness_type: "branch_merge",
                        repo_id: &repo_id,
                    });

                    Ok(())
                }))
                .timeout_ms(10_000),

            // Step 5: Verify repo graph state and Stratum storage
            JourneyStep::new("verify_repo_graph_state")
                .party("alice")
                .depends_on(&["alice_merges"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let repo_id = ctx.get::<String>("repo_id");
                    let root_hash = ctx.get::<String>("repo_root_hash");
                    let merge_hash = ctx.get::<String>("merge_commit_hash");

                    let graph = ctx.polygit().repo_graph(&repo_id)?;
                    assert!(graph.is_dag());
                    assert!(graph.contains_path(&root_hash, &merge_hash));
                    assert!(graph.all_signatures_valid());
                    assert_eq!(graph.head("main"), Some(&merge_hash));

                    let stratum = StratumVerifier::new(ctx);
                    let csr = stratum.verify_csr_tiers(&repo_id)?;
                    assert!(csr.tier_matches(CsrTier::Hot));
                    assert!(csr.shard_distribution_valid);

                    let merkle = stratum.verify_series_merkle_chain(&repo_id)?;
                    assert!(merkle.chain_intact);
                    assert!(merkle.root_hash_valid);

                    assert_metric_emitted!(ctx, "polygit.graph.verified", {
                        "dag_valid" => "true",
                        "all_sigs_valid" => "true",
                    });

                    Ok(())
                }))
                .timeout_ms(12_000),

            // Step 6: Verify blind telemetry and Cortex visibility
            JourneyStep::new("verify_blind_telemetry")
                .party("alice")
                .depends_on(&["verify_repo_graph_state"])
                .action(StepAction::Execute(|ctx: &mut ConvoyContext| {
                    let telemetry = ctx.streamsight().drain_telemetry("poly-git-v1");

                    for event in &telemetry {
                        assert_blinded!(ctx, &event.event_type, {
                            field: "user_id",
                            blinding: "hmac_sha3",
                        });

                        assert_blinded!(ctx, &event.event_type, {
                            field: "commit_content",
                            blinding: "absent",
                        });

                        assert_blinded!(ctx, &event.event_type, {
                            field: "review_body",
                            blinding: "absent",
                        });
                    }

                    let cortex = CortexVisibility::new(ctx);
                    cortex.assert_redacted("polygit", RedactPolicy::ContentFields)?;
                    cortex.assert_obfuscated("polygit", ObfuscatePolicy::PartyIdentifiers)?;

                    assert!(telemetry.len() >= 5, "Expected at least 5 telemetry events");

                    for event in &telemetry {
                        assert!(
                            event.namespace.starts_with("poly-git-v1"),
                            "Telemetry leaked outside poly-git-v1 namespace: {}",
                            event.namespace
                        );
                    }

                    Ok(())
                }))
                .timeout_ms(5_000),
        ]
    }

    fn metrics(&self) -> JourneyMetrics {
        JourneyMetrics {
            expected_events: vec![
                "polygit.repo.created",
                "polygit.commit.pushed",
                "polygit.review.submitted",
                "polygit.merge.complete",
                "polygit.graph.verified",
            ],
            max_duration_ms: 60_000,
            required_povc_witnesses: 4,
            lex_namespace: "poly-git-v1",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use estream_test::convoy::ConvoyRunner;

    #[tokio::test]
    async fn run_polygit_journey() {
        let runner = ConvoyRunner::new()
            .with_es_git()
            .with_streamsight("poly-git-v1")
            .with_stratum()
            .with_cortex();

        runner.run(PolygitJourney).await.expect("Polygit journey failed");
    }
}
