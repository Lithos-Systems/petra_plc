use soft_plc::{engine::ScanEngine, signal::SignalValue};
use std::time::Duration;

#[tokio::main]
async fn main() -> soft_plc::Result<()> {
    let config_path = "config/pump_alternation.yaml";
    let mut engine = ScanEngine::from_file(config_path)?;
    
    // Spawn engine task
    let engine_handle = tokio::spawn(async move {
        if let Err(e) = engine.run().await {
            eprintln!("Engine error: {}", e);
        }
    });
    
    // Monitor in main task
    let bus = engine.signal_bus().clone();
    let mut monitor_interval = tokio::time::interval(Duration::from_secs(1));
    
    println!("=== Pump Alternation Monitor ===");
    println!("Commands:");
    println!("  Press 'p' to drop pressure");
    println!("  Press 'r' to recover pressure");
    println!("  Press 's' to reset system");
    println!("  Press 'q' to quit\n");
    
    loop {
        monitor_interval.tick().await;
        
        // Display status
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("=== Pump Status ===");
        
        if let Ok(pressure) = bus.get("pressure") {
            println!("Pressure: {:?}", pressure);
        }
        
        if let Ok(index) = bus.get("pump_index") {
            println!("Active Pump Index: {:?}", index);
        }
        
        println!("\nPump States:");
        for i in 1..=5 {
            if let Ok(state) = bus.get(&format!("pump{}_run", i)) {
                let status = match state {
                    SignalValue::Bool(true) => "RUNNING",
                    _ => "OFF"
                };
                println!("  Pump {}: {}", i, status);
            }
        }
        
        // Simple command handling (would need proper async stdin in production)
        // This is just for demonstration
    }
    
    engine_handle.abort();
    Ok(())
}
