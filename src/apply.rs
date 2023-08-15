use crate::{resources::AppInstance, scripting::Script, Result};
use kube::ResourceExt;

/// Generates shell script that will apply the manifests and writes it to w
pub fn emit_script<W>(app_instance: &AppInstance, w: &mut W) -> Result<()>
where
    W: std::io::Write,
{
    let script = script(app_instance, "/tmp/manifests")?;
    write!(w, "{script}")?;
    Ok(())
}

/// Generates shell script that will apply the manifests
pub fn script(app_instance: &AppInstance, manifests_dir: &str) -> Result<Script> {
    let tokens = emit_commandline(app_instance, manifests_dir);
    Ok(Script::from_vec(tokens))
}

pub fn emit_commandline(app_instance: &AppInstance, manifests_dir: &str) -> Vec<String> {
    vec![
        "kubectl",
        "apply",
        "-f",
        manifests_dir,
        "-n",
        &app_instance.namespace().unwrap(),
        "--server-side",
        "--prune",
        "--applyset",
        &app_instance.name_any(),
        "--force-conflicts",
        "-v=2",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
