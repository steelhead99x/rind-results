// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Parity of the `crit` commands with the Python scripts they replace.
//!
//! `fixtures/crit/expected/` is what the scripts printed (run at f88cc53 with
//! Python 3.12) on the inputs below. The criterion homes are rebuilt from
//! each published `rind-results/ab-v1` JSON, one `new/` directory per row,
//! alternating slope and mean-only estimates; `ab-collect` on those homes
//! must give back the published JSON byte for byte.

use serde_json::{Map, Value, json};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const AB_JSON: &[&str] = &[
    "hosts/m1pro/ab-narrow-mul/ab-mac-p521.json",
    "hosts/m1pro/ab-narrow-mul/ab-mac-pk1.json",
    "hosts/m1pro/ab-narrow-mul/ab-mac-series24.json",
    "hosts/m1pro/ab-narrow-mul/ab-mac-stack.json",
    "hosts/m1pro/ab-narrow-mul/ab-mac-w7.json",
    "hosts/pi5/ab-narrow-mul/ab-cpu-detect.json",
    "hosts/pi5/ab-narrow-mul/ab-narrow-mul.json",
    "hosts/pi5/ab-narrow-mul/ab-p384-w7.json",
    "hosts/pi5/ab-narrow-mul/ab-p521-w7.json",
    "hosts/pi5/ab-narrow-mul/ab-rsa-8n.json",
    "hosts/pi5/ab-narrow-mul/series2-layout/ab-s2-det2.json",
    "hosts/pi5/ab-narrow-mul/series2-layout/ab-s2-det3.json",
    "hosts/pi5/ab-narrow-mul/series2-layout/ab-s2-deta.json",
    "hosts/pi5/ab-narrow-mul/series2-layout/ab-s2-nista.json",
    "hosts/pi5/ab-narrow-mul/table-align/ab-p256-table-align.json",
    "tools/xtask/tests/fixtures/crit/synthetic.json",
];

const PMU_TSV: &[&str] = &[
    "results/argon2-interleave/raw/m1pro-neon-wall-r1.tsv",
    "results/argon2-interleave/raw/m1pro-neon-wall-r2.tsv",
    "results/argon2-interleave/raw/m1pro-sha3-wall-r1.tsv",
    "results/argon2-interleave/raw/m1pro-sha3-wall-r2.tsv",
    "results/argon2-interleave/raw/pi5-pmu-r1.tsv",
    "results/argon2-interleave/raw/pi5-pmu-r2-LOADED.tsv",
    "results/argon2-interleave/raw/pi5-pmu-r3-LOADED.tsv",
    "results/argon2-interleave/raw/pi5-pmu-r4.tsv",
    "results/argon2-interleave/raw/pi5-pmu-r5-umlal.tsv",
    "results/argon2-scalar-rider/raw/m1pro-neon-wall-h1.tsv",
    "results/argon2-scalar-rider/raw/m1pro-sha3-wall-h1-20alt.tsv",
    "results/argon2-scalar-rider/raw/m1pro-sha3-wall-h1.tsv",
    "results/argon2-scalar-rider/raw/pi5-pmu-h1-LOADED.tsv",
    "results/argon2-scalar-rider/raw/pi5-pmu-h2.tsv",
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn expected(name: &str) -> String {
    let p = root()
        .join("tools/xtask/tests/fixtures/crit/expected")
        .join(name);
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn xtask(args: &[&str], cwd: &Path) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run xtask");
    assert!(
        out.status.success(),
        "xtask {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 output")
}

fn stem(p: &str) -> String {
    Path::new(p)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/// Rebuilds `src`'s criterion homes, META.json and run log under `work`.
fn rebuild(src: &Path, work: &Path) -> Value {
    let d: Value = serde_json::from_str(&fs::read_to_string(src).unwrap()).unwrap();
    let _ = fs::remove_dir_all(work);
    let homes = work.join("homes");
    for (i, r) in d["results"].as_array().unwrap().iter().enumerate() {
        let nd = homes
            .join(format!("{}-r{}", r["mode"].as_str().unwrap(), r["round"]))
            .join(format!("b{i}"))
            .join("new");
        fs::create_dir_all(&nd).unwrap();
        let est = json!({
            "point_estimate": r["ns"],
            "confidence_interval": {"lower_bound": r["ns_low"], "upper_bound": r["ns_high"]},
        });
        let slope = if i % 2 == 0 { est.clone() } else { Value::Null };
        fs::write(
            nd.join("benchmark.json"),
            json!({"full_id": r["id"]}).to_string(),
        )
        .unwrap();
        fs::write(
            nd.join("estimates.json"),
            json!({"mean": est, "slope": slope}).to_string(),
        )
        .unwrap();
    }
    // Neither a <mode>-r<N> label nor a sample: both are skipped.
    fs::create_dir_all(homes.join("notes/x/new")).unwrap();
    fs::create_dir_all(homes.join("base-r9/empty/new")).unwrap();
    let meta: Map<String, Value> = d
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| !["schema", "criterion_args", "log", "results"].contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    fs::write(work.join("meta.json"), Value::Object(meta).to_string()).unwrap();
    let log: String = d["log"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| format!("{}\n", l.as_str().unwrap()))
        .collect();
    fs::write(work.join("run.log"), log).unwrap();
    d
}

#[test]
fn criterion_summaries_match_the_scripts() {
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("crit");
    for src in AB_JSON {
        let s = stem(src);
        let work = tmp.join(&s);
        let d = rebuild(&root().join(src), &work);
        let modes: Vec<String> = d["results"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["mode"].as_str().unwrap().to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let homes = work.join("homes");
        let h = homes.to_str().unwrap();
        let cwd = root();
        assert_eq!(
            xtask(&["crit-parse", h], &cwd),
            expected(&format!("{s}.crit-parse.md")),
            "crit-parse {src}"
        );
        let mut cmp = vec!["ab-cmp", h];
        cmp.extend(modes.iter().map(String::as_str));
        assert_eq!(
            xtask(&cmp, &cwd),
            expected(&format!("{s}.ab-cmp.md")),
            "ab-cmp {src}"
        );
        let (first, last) = (&modes[0], &modes[modes.len() - 1]);
        assert_eq!(
            xtask(&["ab-min", h, first, last], &cwd),
            expected(&format!("{s}.ab-min.md")),
            "ab-min {src}"
        );
        let collected = xtask(
            &[
                "ab-collect",
                h,
                work.join("run.log").to_str().unwrap(),
                work.join("meta.json").to_str().unwrap(),
            ],
            &cwd,
        );
        let want = if s == "synthetic" {
            expected("synthetic.ab-collect.json")
        } else {
            fs::read_to_string(root().join(src)).unwrap()
        };
        assert_eq!(collected, want, "ab-collect {src}");
    }
}

#[test]
fn argon2_pmu_sum_matches_the_script() {
    for src in PMU_TSV {
        let mut s = stem(src);
        if src.contains("argon2-scalar-rider") {
            s = format!("rider-{s}");
        }
        let text = fs::read_to_string(root().join(src)).unwrap();
        let base = text
            .lines()
            .find(|l| !l.starts_with('#') && !l.contains("DEBUG"))
            .and_then(|l| l.split('\t').nth(1))
            .unwrap()
            .to_string();
        let cwd = root();
        assert_eq!(
            xtask(&["argon2-pmu-sum", src], &cwd),
            expected(&format!("{s}.argon2-pmu-sum.md")),
            "argon2-pmu-sum {src}"
        );
        assert_eq!(
            xtask(&["argon2-pmu-sum", src, &base], &cwd),
            expected(&format!("{s}.argon2-pmu-sum.base.md")),
            "argon2-pmu-sum {src} {base}"
        );
    }
}

#[test]
fn ab_pmu_and_perf_sums_match_the_scripts() {
    let s2 = root().join("hosts/pi5/ab-narrow-mul/series2-layout");
    assert_eq!(
        xtask(&["ab-pmu-sum", "harness-pmu-raw.log"], &s2),
        expected("harness-pmu-raw.ab-pmu-sum.md")
    );
    let perf = xtask(&["ab-perf-sum"], &s2.join("perf-stat"));
    assert_eq!(perf, expected("perf-stat.ab-perf-sum.md"));
    // The published table is the same text.
    assert!(
        fs::read_to_string(s2.join("s2-perf-stat.md"))
            .unwrap()
            .starts_with(&perf)
    );
}

#[test]
fn bad_input_fails_like_the_scripts() {
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("crit-bad");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    fs::write(tmp.join("short.tsv"), "a1\tv\t1MiB\n").unwrap();
    fs::write(tmp.join("badkv.tsv"), "a1\tv\t1MiB\trind\twall_ms\n").unwrap();
    for f in ["short.tsv", "badkv.tsv"] {
        let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
            .args(["argon2-pmu-sum", f])
            .current_dir(&tmp)
            .output()
            .unwrap();
        assert!(!out.status.success(), "{f}");
    }
    // A rind id with no BASE rounds has no minimum.
    let homes = tmp.join("homes/new-r1/b/new");
    fs::create_dir_all(&homes).unwrap();
    fs::write(homes.join("benchmark.json"), r#"{"full_id": "x/rind"}"#).unwrap();
    fs::write(
        homes.join("estimates.json"),
        r#"{"mean": {"point_estimate": 1.0}, "slope": null}"#,
    )
    .unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["ab-min", "homes", "base", "new"])
        .current_dir(&tmp)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("empty sequence"));
}
