use anyhow::Result;
use tracing::{debug, info};

/// # Errors
pub fn hello() -> Result<()> {
    info!("Kitty :3");
    println!("Hello from lib!");
    debug!("hello printed");
    Ok(())
}
