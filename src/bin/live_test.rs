use soft_plc::{Result, engine::ScanEngine, signal::SignalValue};
use std::io::{self, Write};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Soft-PLC Live Test ===");
    println!("Loading pump alternation example...\n");
    
    let mut engine = ScanEngine::from_file("config/pump_alternation.yaml")?;
    let bus = engine.signal_bus().clone();
    
    // Start engine in background
    let engine_handle = tokio::spawn(async move {
        engine.run().await
    });
    
    // Interactive control loop
    let stdin = io::stdin();
    let mut input = String::new();
    
    loop {
        // Display status
        println!("\n--- Current Status ---");
        if let Ok(pressure) = bus.get("pressure") {
            println!("Pressure: {:?}", pressure);
        }
        if let Ok(index) = bus.get("pump_index") {
            println!("Pump Index: {:?}", index);
        }
        
        // Show pump states
        print!("Pumps: ");
        for i in 1..=5 {
            if let Ok(SignalValue::Bool(running)) = bus.get(&format!("pump{}_run", i)) {
                if running {
                    print!("[P{}:ON] ", i);
                } else {
                    print!("[P{}:--] ", i);
                }
            }
        }
        println!("\n");
        
        // Menu
        println!("Commands:");
        println!("  1 - Set pressure to 45 (trigger pump)");
        println!("  2 - Set pressure to 65 (stop pump)");
        println!("  3 - Toggle manual override");
        println!("  4 - System reset");
        println!("  5 - Show all signals");
        println!("  q - Quit");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                bus.set("pressure", SignalValue::Float(45.0))?;
                println!("→ Pressure set to 45.0");
            }
            "2" => {
                bus.set("pressure", SignalValue::Float(65.0))?;
                println!("→ Pressure set to 65.0");
            }
            "3" => {
                let current = bus.get_bool("manual_override").unwrap_or(false);
                bus.set("manual_override", SignalValue::Bool(!current))?;
                println!("→ Manual override: {}", !current);
            }
            "4" => {
                bus.set("system_reset", SignalValue::Bool(true))?;
                tokio::time::sleep(Duration::from_millis(200)).await;
                bus.set("system_reset", SignalValue::Bool(false))?;
                println!("→ System reset");
            }
            "5" => {
                println!("\n=== All Signals ===");
                let signals = bus.iter();  // Already returns Vec
                let mut signals = signals;
                signals.sort_by(|a, b| a.0.cmp(&b.0));
                for (name, value) in signals {
                    println!("  {:<25} = {:?}", name, value);
                }
            }
            "q" => break,
            _ => println!("Invalid choice"),
        }
        
        // Give engine time to process
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    engine_handle.abort();
    println!("\nShutting down...");
    Ok(())
}
