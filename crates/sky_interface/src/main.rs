#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let instruction = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let instruction = if instruction.trim().is_empty() {
        None
    } else {
        Some(instruction)
    };

    sky_interface::run(instruction).await
}
