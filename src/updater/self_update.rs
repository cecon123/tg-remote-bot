use anyhow::Result;

pub async fn self_update(_url: &str, _sha256: Option<&str>) -> Result<()> {
    // TODO: download → sha256 verify → rename exe → move tmp → sc stop → sc start → clean .old
    anyhow::bail!("self-update not yet implemented");
}
