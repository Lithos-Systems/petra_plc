use soft_plc::{Result, engine::ScanEngine};
use tracing::{info, error};
use tracing_subscriber;
use tokio::signal;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    info!("Soft-PLC starting...");
    
    // Get config file from command line or use default
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example_logic.yaml".to_string());
    
    info!("Loading configuration from: {}", config_path);
    
    // Create and start scan engine
    let mut engine = ScanEngine::from_file(&config_path)?;
    
    // Clone signal bus for monitoring
    let signal_bus = engine.signal_bus().clone();
    
    // Spawn monitoring task
    let monitor_handle = tokio::spawn(async move {
        let mut monitor_interval = tokio::time::interval(Duration::from_secs(1));
        
        loop {
            monitor_interval.tick().await;
            
            // Print key signals
            if let Ok(motor_run) = signal_bus.get("motor_run") {
                info!("Motor status: {:?}", motor_run);
            }
            
            if let Ok(timer_done) = signal_bus.get("timer_done") {
                info!("Timer status: {:?}", timer_done);
            }
        }
    });
    
    // Spawn engine task
    let engine_handle = tokio::spawn(async move {
        if let Err(e) = engine.run().await {
            error!("Engine error: {}", e);
        }
    });
    
    // Wait for Ctrl+C
    info!("PLC running. Press Ctrl+C to stop...");
    signal::ctrl_c().await?;
    
    info!("Shutdown signal received");
    
    // Stop tasks
    monitor_handle.abort();
    engine_handle.abort();
    
    info!("Soft-PLC stopped");
    Ok(())
}
