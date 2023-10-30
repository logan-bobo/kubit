use assert_cmd::prelude::*;
use kubit::apply::{KUBECTL_APPLYSET_ENABLED, KUBECTL_IMAGE, KUBIT_APPLIER_FIELD_MANAGER};
use kubit::render::KUBECFG_IMAGE;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

const DEMO_PACKAGE: &str = "oci://ghcr.io/kubecfg/kubit/package-demo:v1";
const TEST_FILE: &str = "tests/fixtures/fake-package.yml";

#[tokio::test]
async fn local_apply_dry_run_script() {
    let mut cmd = Command::cargo_bin("kubit").unwrap();
    let output = cmd
        .args([
            "local",
            "apply",
            TEST_FILE,
            "--dry-run",
            "script",
            "--skip-auth",
        ])
        .unwrap()
        .stdout
        .to_vec();

    let output = from_utf8(&output).expect("unable to read output script");
    let overlay_file = PathBuf::from(
        std::fs::canonicalize(TEST_FILE)
            .expect("unable to find realpath for test")
            .file_name()
            .unwrap(),
    );

    // Assert some known required items in the output command.
    assert!(output.contains("docker"));
    assert!(output.contains(DEMO_PACKAGE));
    assert!(output.contains(KUBECTL_IMAGE));
    assert!(output.contains(KUBECFG_IMAGE));
    assert!(output.contains(KUBECTL_APPLYSET_ENABLED));
    assert!(output.contains(KUBIT_APPLIER_FIELD_MANAGER));
    assert!(output.contains("--server-side"));
    assert!(output.contains(&format!("appInstance_=/overlay/{}", overlay_file.display())));

    // When using --skip-auth we should not mount credentials
    assert!(!output.contains("DOCKER_CONFIG"));
}

#[tokio::test]
async fn local_apply_dry_run_render() {
    let mut cmd = Command::cargo_bin("kubit").unwrap();
    let output = cmd
        .args([
            "local",
            "apply",
            TEST_FILE,
            "--dry-run",
            "render",
            "--skip-auth",
        ])
        .unwrap()
        .stdout
        .to_vec();

    let output = from_utf8(&output).expect("unable to read output script");

    // Assert some known required items in the rendered output.
    assert!(output.contains("gar-docker-secret"));
    assert!(output.contains("StatefulSet"));
    assert!(output.contains("Service"));
    assert!(output.contains("AppInstance"));
    assert!(output.contains("kubecfg.dev/v1alpha1"));
    assert!(output.contains(DEMO_PACKAGE.strip_prefix("oci://").unwrap()));
}
