"""calibrate.py — Gold test set generation and calibration CLI for quality gates.

Phase 13 Plan 02:

- ``generate_gold_fixtures()`` is a one-shot helper that creates the labelled
  gold fixture tree under ``tools/authoring/test-fixtures/gold/``. Fixtures are
  committed to git; this helper is not called on production paths.
- ``run_calibrate()`` iterates the committed gold manifest and measures
  mechanical + judgment gate accuracy (TPR/TNR). Consumed by the
  ``python -m authoring calibrate`` CLI subcommand.

Security: YAML parsing uses ``yaml.safe_load()`` exclusively (T-13-04 mitigation
in the Phase 13 threat register). Never ``yaml.load()``.
"""
from __future__ import annotations

import shutil
import textwrap
from dataclasses import dataclass
from pathlib import Path

import yaml  # pyyaml

from .quality_gate import GateReport, run_gate
from .subprocess_tools import build_binaries, resolve_project_root


_THIS_FILE = Path(__file__).resolve()
GOLD_DIR = _THIS_FILE.parent / "test-fixtures" / "gold"
KINEMATICS_SRC = (
    _THIS_FILE.parent.parent.parent
    / "content"
    / "classical-mechanics"
    / "kinematics"
)


# ---------------------------------------------------------------------------
# Fixture generation helpers
# ---------------------------------------------------------------------------
def _copy_node(src: Path, dst: Path) -> None:
    """Copy a node directory (all .md + node.yaml) to ``dst``."""
    dst.mkdir(parents=True, exist_ok=True)
    for f in src.iterdir():
        if f.suffix in (".md", ".yaml"):
            shutil.copy2(f, dst / f.name)


def _patch_yaml_field(node_yaml: Path, key: str, value) -> None:
    """Rewrite ``node_yaml`` with a single field replaced.

    Uses ``yaml.safe_load()`` — never ``yaml.load()`` (T-13-04 mitigation).
    """
    data = yaml.safe_load(node_yaml.read_text())
    if not isinstance(data, dict):
        data = {}
    data[key] = value
    node_yaml.write_text(yaml.dump(data, allow_unicode=True, sort_keys=False))


def _inject_orphan_dollar(phase_file: Path) -> None:
    """Prepend a lone ``$`` to the body of a phase file (after frontmatter)."""
    text = phase_file.read_text()
    parts = text.split("---", 2)
    if len(parts) >= 3:
        body = "\n$orphan_dollar\n" + parts[2]
        phase_file.write_text(parts[0] + "---" + parts[1] + "---" + body)
    else:
        phase_file.write_text(text + "\n$orphan_dollar\n")


def _truncate_phase_body(phase_file: Path, replacement: str = "Too short.\n") -> None:
    """Preserve frontmatter, replace body with a very short text."""
    text = phase_file.read_text()
    parts = text.split("---", 2)
    if len(parts) >= 3:
        phase_file.write_text(parts[0] + "---" + parts[1] + "---\n" + replacement)
    else:
        phase_file.write_text("---\n---\n" + replacement)


# ---------------------------------------------------------------------------
# Gold fixture generation
# ---------------------------------------------------------------------------
def generate_gold_fixtures() -> None:
    """Generate all gold test fixtures from the kinematics pilot node.

    Produces 20 nodes total:

    - 1 good copy (``kinematics-good``) — expected PASS
    - 5 hand-crafted bad-judgment nodes with pre-written ``review-report.md``
      — expected FAIL on specific judgment dimensions
    - 14 programmatic mutations, each targeting exactly one mechanical failure
      mode — expected FAIL

    Run once; commit results. Safe to re-run (overwrites existing fixtures).
    """
    GOLD_DIR.mkdir(parents=True, exist_ok=True)

    # --- Good node (ground truth PASS) ---
    #
    # The pilot kinematics node declares prerequisites (vectors, calculus)
    # that don't yet exist as content directories. That makes the unmodified
    # pilot fail the gate's prerequisite_existence mechanical check, which
    # would make every gold copy of it fail even when no intentional mutation
    # has been applied. To preserve the "good node = PASS" invariant, we
    # clear `prerequisites` on the baseline good node and on every mutation
    # *except* the `missing-prerequisites` mutation (which explicitly sets a
    # bad prereq slug).
    good_dir = GOLD_DIR / "kinematics-good"
    _copy_node(KINEMATICS_SRC, good_dir)
    _patch_yaml_field(good_dir / "node.yaml", "prerequisites", [])
    # Good node has no review-report.md — judgment checks yield WARNING, not
    # FAIL. Overall verdict remains PASS (WARNINGs do not block per Plan 01).

    # --- Bad-judgment nodes (hand-crafted review-report.md with FAIL dims) ---
    def _make_bad_judgment(slug: str, review_text: str) -> None:
        d = GOLD_DIR / slug
        _copy_node(KINEMATICS_SRC, d)
        # Clear prereqs so judgment failure is the only reason to FAIL.
        _patch_yaml_field(d / "node.yaml", "prerequisites", [])
        (d / "review-report.md").write_text(review_text)

    _make_bad_judgment(
        "kinematics-bad-judgment-wrong-formula",
        textwrap.dedent("""\
            # Review Report

            ## Physics Reviewer

            ### Formula Correctness
            Status: FAIL
            The central formula F = ma is stated without derivation in Phase 2.
            The dimensional analysis is missing and the formula is presented
            as axiomatic rather than derived from Newton's second law.

            ### Derivation Rigor
            Status: FAIL
            No derivation is provided. Phase 2 jumps directly to symbolic
            manipulation without connecting to Phase 1 struggle observations.

            ### Unit Consistency
            Status: PASS

            ## Pedagogy Reviewer

            ### Productive Failure Design
            Status: PASS

            ### Concreteness Fading Sequence
            Status: PASS

            ### Worked Example Fading
            Status: PASS

            ### Self Explanation Quality
            Status: PASS

            ### Cognitive Load
            Status: PASS
        """),
    )

    _make_bad_judgment(
        "kinematics-bad-judgment-rubber-stamp",
        textwrap.dedent("""\
            # Review Report

            ## Physics Reviewer

            ### Formula Correctness
            Status: PASS

            ### Derivation Rigor
            Status: PASS

            ### Unit Consistency
            Status: PASS

            ## Pedagogy Reviewer

            ### Productive Failure Design
            Status: FAIL
            Phase 1 struggle problem is solvable with prior knowledge of basic
            algebra. A student at EQF 3 can apply substitution to reach the
            answer without encountering productive failure. The problem does not
            create a genuine impasse requiring new conceptual tools.

            ### Concreteness Fading Sequence
            Status: PASS

            ### Worked Example Fading
            Status: PASS

            ### Self Explanation Quality
            Status: PASS

            ### Cognitive Load
            Status: PASS
        """),
    )

    _make_bad_judgment(
        "kinematics-bad-judgment-no-fading",
        textwrap.dedent("""\
            # Review Report

            ## Physics Reviewer

            ### Formula Correctness
            Status: PASS

            ### Derivation Rigor
            Status: PASS

            ### Unit Consistency
            Status: PASS

            ## Pedagogy Reviewer

            ### Productive Failure Design
            Status: PASS

            ### Concreteness Fading Sequence
            Status: FAIL
            Phase 2 (Concreteness Fading) goes directly to symbolic notation
            without grounding in concrete examples. The iconic intermediate
            representation (diagram/graph) is absent. The fading sequence is
            concrete -> symbolic (missing iconic stage).

            ### Worked Example Fading
            Status: FAIL
            Phase 3 presents fully worked examples with no gradual withdrawal
            of scaffolding. Phase 4 provides another complete worked example
            rather than a partially-worked or completion problem.

            ### Self Explanation Quality
            Status: PASS

            ### Cognitive Load
            Status: PASS
        """),
    )

    _make_bad_judgment(
        "kinematics-bad-judgment-poor-self-explanation",
        textwrap.dedent("""\
            # Review Report

            ## Physics Reviewer

            ### Formula Correctness
            Status: PASS

            ### Derivation Rigor
            Status: PASS

            ### Unit Consistency
            Status: PASS

            ## Pedagogy Reviewer

            ### Productive Failure Design
            Status: PASS

            ### Concreteness Fading Sequence
            Status: PASS

            ### Worked Example Fading
            Status: PASS

            ### Self Explanation Quality
            Status: FAIL
            Phase 5 self-explanation prompts ask students to restate what they
            read rather than explain the reasoning behind each step. Prompts
            are surface-level ("What is the formula?") rather than process-level
            ("Why does the acceleration term appear on the left?"). No prompts
            target misconceptions listed in the node specification.

            ### Cognitive Load
            Status: PASS
        """),
    )

    _make_bad_judgment(
        "kinematics-bad-judgment-high-cognitive-load",
        textwrap.dedent("""\
            # Review Report

            ## Physics Reviewer

            ### Formula Correctness
            Status: PASS

            ### Derivation Rigor
            Status: PASS

            ### Unit Consistency
            Status: PASS

            ## Pedagogy Reviewer

            ### Productive Failure Design
            Status: PASS

            ### Concreteness Fading Sequence
            Status: PASS

            ### Worked Example Fading
            Status: PASS

            ### Self Explanation Quality
            Status: PASS

            ### Cognitive Load
            Status: FAIL
            Phase 2 introduces three new representations (position-time graph,
            velocity-time graph, and symbolic kinematics equations) simultaneously.
            Each representation should be introduced and consolidated separately
            before combining. The current structure exceeds working memory limits
            for a learner at EQF 3.
        """),
    )

    # --- Programmatic mutations (mechanical failure modes) ---
    # Each mutation copies the good node and applies exactly one corruption.
    # Per D-07 + plan: 14 total mutations (includes unconditional
    # broken-latex-phase-4 and missing-phase-1).
    MUTATIONS: list[tuple[str, str, list[str]]] = [
        ("missing-phase-1", "phase-1.md deleted", ["rust_validator"]),
        ("missing-phase-3", "phase-3.md deleted", ["rust_validator"]),
        ("missing-phase-5", "phase-5.md deleted", ["rust_validator"]),
        ("bad-yaml-syntax", "node.yaml syntax error (unclosed bracket)", ["rust_validator"]),
        ("wrong-eqf-level", "eqf_level set to 0 (invalid)", ["rust_validator"]),
        ("empty-misconceptions", "misconceptions list is empty", ["rust_validator"]),
        ("broken-latex-inline", "unbalanced $ in phase-2.md", ["latex_balance_phase_2"]),
        ("broken-latex-display", "unbalanced \\[ in phase-2.md", ["latex_balance_phase_2"]),
        ("broken-latex-phase-0", "unbalanced $ in phase-0.md", ["latex_balance_phase_0"]),
        ("broken-latex-phase-4", "unbalanced $ in phase-4.md", ["latex_balance_phase_4"]),
        ("short-phase-1", "phase-1.md reduced to 3 words", ["word_count_phase_1"]),
        ("short-phase-2", "phase-2.md reduced to 3 words", ["word_count_phase_2"]),
        ("missing-prerequisites", "prerequisites list references nonexistent slug", ["prerequisite_existence"]),
        ("empty-node-yaml", "node.yaml is an empty file", ["rust_validator"]),
    ]

    for slug_suffix, _desc, _failing in MUTATIONS:
        slug = f"kinematics-{slug_suffix}"
        dst = GOLD_DIR / slug
        _copy_node(KINEMATICS_SRC, dst)

        # Clear prereqs on every mutation EXCEPT the prereq-targeting one and
        # the yaml-corruption ones (where rewriting node.yaml would mask the
        # intended failure mode).
        if slug_suffix not in (
            "missing-prerequisites",
            "bad-yaml-syntax",
            "empty-node-yaml",
        ):
            _patch_yaml_field(dst / "node.yaml", "prerequisites", [])

        if slug_suffix == "missing-phase-1":
            (dst / "phase-1.md").unlink(missing_ok=True)

        elif slug_suffix == "missing-phase-3":
            (dst / "phase-3.md").unlink(missing_ok=True)

        elif slug_suffix == "missing-phase-5":
            (dst / "phase-5.md").unlink(missing_ok=True)

        elif slug_suffix == "bad-yaml-syntax":
            (dst / "node.yaml").write_text("concept_id: [invalid_yaml\n")

        elif slug_suffix == "wrong-eqf-level":
            _patch_yaml_field(dst / "node.yaml", "eqf_level", 0)

        elif slug_suffix == "empty-misconceptions":
            _patch_yaml_field(dst / "node.yaml", "misconceptions", [])

        elif slug_suffix == "broken-latex-inline":
            _inject_orphan_dollar(dst / "phase-2.md")

        elif slug_suffix == "broken-latex-display":
            text = (dst / "phase-2.md").read_text()
            (dst / "phase-2.md").write_text(text + "\n\\[ unclosed display math\n")

        elif slug_suffix == "broken-latex-phase-0":
            _inject_orphan_dollar(dst / "phase-0.md")

        elif slug_suffix == "broken-latex-phase-4":
            _inject_orphan_dollar(dst / "phase-4.md")

        elif slug_suffix == "short-phase-1":
            _truncate_phase_body(dst / "phase-1.md")

        elif slug_suffix == "short-phase-2":
            _truncate_phase_body(dst / "phase-2.md")

        elif slug_suffix == "missing-prerequisites":
            _patch_yaml_field(
                dst / "node.yaml",
                "prerequisites",
                ["nonexistent-node-abc123"],
            )

        elif slug_suffix == "empty-node-yaml":
            (dst / "node.yaml").write_text("")

    # --- Build gold-manifest.yaml ---
    manifest_entries: list[dict] = []

    manifest_entries.append(
        {
            "slug": "kinematics-good",
            "path": "kinematics-good",
            "expected_verdict": "PASS",
            "expected_failing_checks": [],
            "notes": "Copy of pilot kinematics node — ground truth PASS",
        }
    )

    judgment_nodes = [
        (
            "kinematics-bad-judgment-wrong-formula",
            ["formula_correctness", "derivation_rigor"],
            "Wrong formula and missing derivation",
        ),
        (
            "kinematics-bad-judgment-rubber-stamp",
            ["productive_failure_design"],
            "Struggle problem solvable with prior knowledge",
        ),
        (
            "kinematics-bad-judgment-no-fading",
            ["concreteness_fading_sequence", "worked_example_fading"],
            "Missing iconic stage and worked example fading",
        ),
        (
            "kinematics-bad-judgment-poor-self-explanation",
            ["self_explanation_quality"],
            "Surface-level self-explanation prompts",
        ),
        (
            "kinematics-bad-judgment-high-cognitive-load",
            ["cognitive_load"],
            "Three representations introduced simultaneously",
        ),
    ]
    for slug, failing, notes in judgment_nodes:
        manifest_entries.append(
            {
                "slug": slug,
                "path": slug,
                "expected_verdict": "FAIL",
                "expected_failing_checks": failing,
                "notes": notes,
            }
        )

    for slug_suffix, desc, failing in MUTATIONS:
        slug = f"kinematics-{slug_suffix}"
        manifest_entries.append(
            {
                "slug": slug,
                "path": slug,
                "expected_verdict": "FAIL",
                "expected_failing_checks": failing,
                "notes": desc,
            }
        )

    manifest = {"nodes": manifest_entries}
    manifest_path = GOLD_DIR / "gold-manifest.yaml"
    manifest_path.write_text(
        yaml.dump(manifest, allow_unicode=True, sort_keys=False)
    )
    print(f"[calibrate] Generated {len(manifest_entries)} gold nodes in {GOLD_DIR}")
    print(f"[calibrate] Manifest written to {manifest_path}")


# ---------------------------------------------------------------------------
# Calibration (Task 2)
# ---------------------------------------------------------------------------
@dataclass
class CalibrationResult:
    tp: int
    tn: int
    fp: int
    fn: int

    @property
    def tpr(self) -> float:
        """True Positive Rate (sensitivity): TP / (TP + FN)."""
        denom = self.tp + self.fn
        return self.tp / denom if denom > 0 else float("nan")

    @property
    def tnr(self) -> float:
        """True Negative Rate (specificity): TN / (TN + FP)."""
        denom = self.tn + self.fp
        return self.tn / denom if denom > 0 else float("nan")

    @property
    def total(self) -> int:
        return self.tp + self.tn + self.fp + self.fn


def run_calibrate(
    manifest_path: Path | None = None,
    project_root: Path | None = None,
    verbose: bool = True,
) -> CalibrationResult:
    """Run the quality gate against the gold test set and report TPR/TNR.

    Loads ``gold-manifest.yaml`` (via ``yaml.safe_load`` — T-13-04), resolves
    each node path relative to the manifest's parent directory, and invokes
    ``run_gate`` once per node. Predictions are compared against the labelled
    ``expected_verdict`` to produce a confusion matrix.

    Per Research Pitfall 4, the Rust binaries are built once before the loop
    via ``build_binaries()`` to avoid per-node rebuild overhead.

    Args:
        manifest_path: Path to ``gold-manifest.yaml``. Defaults to
            ``GOLD_DIR / "gold-manifest.yaml"``.
        project_root: Rust project root. Resolved automatically if ``None``.
        verbose: Print per-node result lines + summary to stdout.

    Returns:
        CalibrationResult with tp, tn, fp, fn, and derived tpr/tnr properties.
    """
    if manifest_path is None:
        manifest_path = GOLD_DIR / "gold-manifest.yaml"

    manifest = yaml.safe_load(manifest_path.read_text())
    nodes = manifest["nodes"]

    resolved_root = resolve_project_root() if project_root is None else project_root

    # Build Rust binaries once before iterating (Research Pitfall 4).
    build_binaries(resolved_root)

    tp = tn = fp = fn = 0

    for entry in nodes:
        slug = entry["slug"]
        # Resolve to absolute path — the Rust validator is invoked with
        # cwd=project_root and treats relative paths relative to that cwd,
        # which causes false "file not found" errors on gold fixtures.
        node_path = (manifest_path.parent / entry["path"]).resolve()
        expected_pass = entry["expected_verdict"] == "PASS"

        try:
            report: GateReport = run_gate(node_path, resolved_root)
            predicted_pass = report.overall_pass
        except Exception as exc:  # noqa: BLE001 — gate failure = predicted FAIL
            predicted_pass = False
            if verbose:
                print(f"  [ERROR] {slug}: gate raised exception: {exc}")

        # Classification convention: "positive" = the gate correctly
        # catches a bad node. So:
        #   expected FAIL, predicted FAIL → TP (caught the defect)
        #   expected PASS, predicted PASS → TN (passed a good node)
        #   expected PASS, predicted FAIL → FP (false alarm)
        #   expected FAIL, predicted PASS → FN (missed defect — most dangerous)
        if not expected_pass and not predicted_pass:
            outcome = "TP"
            tp += 1
        elif expected_pass and predicted_pass:
            outcome = "TN"
            tn += 1
        elif expected_pass and not predicted_pass:
            outcome = "FP"
            fp += 1
        else:  # not expected_pass and predicted_pass
            outcome = "FN"
            fn += 1

        if verbose:
            verdict = "PASS" if predicted_pass else "FAIL"
            expected = "PASS" if expected_pass else "FAIL"
            marker = "OK" if outcome in ("TP", "TN") else "!!"
            print(
                f"  [{marker}] {slug}: expected={expected} got={verdict} ({outcome})"
            )

    result = CalibrationResult(tp=tp, tn=tn, fp=fp, fn=fn)

    if verbose:
        print()
        print(
            f"Gold set: {result.total} nodes | "
            f"TPR={result.tpr:.2f} | TNR={result.tnr:.2f} | "
            f"TP={tp} TN={tn} FP={fp} FN={fn}"
        )
        if result.tpr < 0.8:
            print("  WARNING: TPR below 0.80 — gate misses too many real failures")
        if result.tnr < 0.8:
            print("  WARNING: TNR below 0.80 — gate produces too many false alarms")

    return result
