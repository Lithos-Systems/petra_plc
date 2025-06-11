// Fix src/bin/test_runner.rs
use soft_plc::{Result, engine::ScanEngine, signal::SignalValue};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Soft-PLC Test Runner...\n");
    
    // Test 1: Basic functionality
    println!("Test 1: Basic NOT gate");
    let mut engine = ScanEngine::from_file("config/test_basic.yaml")?;
    
    // Run one scan manually
    engine.execute_blocks()?;
    
    // Check initial state
    let output = engine.signal_bus().get_bool("test_output")?;
    println!("  Input: false, Output: {} (expected: true)", output);
    assert_eq!(output, true);
    
    // Change input
    engine.signal_bus().set("test_input", SignalValue::Bool(true))?;
    engine.execute_blocks()?;
    
    let output = engine.signal_bus().get_bool("test_output")?;
    println!("  Input: true, Output: {} (expected: false)", output);
    assert_eq!(output, false);
    
    println!("✓ Test 1 passed!\n");
    
    // Test 2: Timer functionality
    println!("Test 2: Timer (TON) functionality");
    test_timer().await?;
    
    // Test 3: Sequencer
    println!("\nTest 3: Sequencer functionality");
    test_sequencer()?;
    
    println!("\n✅ All tests passed!");
    Ok(())
}

async fn test_timer() -> Result<()> {
    let yaml = r#"
signals:
  - name: "timer_input"
    type: "bool"
    initial: false
  - name: "timer_done"
    type: "bool"
    initial: false

blocks:
  - name: "test_timer"
    type: "TON"
    inputs:
      in: "timer_input"
    outputs:
      q: "timer_done"
    params:
      preset_ms: 200
      
scan_time_ms: 50
"#;

    let config = soft_plc::engine::PlcConfig::from_yaml(yaml)?;
    let mut engine = ScanEngine::new(config)?;
    
    // Start timer
    engine.signal_bus().set("timer_input", SignalValue::Bool(true))?;
    
    // Run scans for 100ms (timer not done)
    for _ in 0..2 {
        engine.execute_blocks()?;
        sleep(Duration::from_millis(50)).await;
    }
    
    let done = engine.signal_bus().get_bool("timer_done")?;
    println!("  After 100ms: timer_done = {} (expected: false)", done);
    assert_eq!(done, false);
    
    // Run scans for another 150ms (timer should complete)
    for _ in 0..3 {
        engine.execute_blocks()?;
        sleep(Duration::from_millis(50)).await;
    }
    
    let done = engine.signal_bus().get_bool("timer_done")?;
    println!("  After 250ms: timer_done = {} (expected: true)", done);
    assert_eq!(done, true);
    
    println!("✓ Test 2 passed!");
    Ok(())
}

fn test_sequencer() -> Result<()> {
    let yaml = r#"
signals:
  - name: "trigger"
    type: "bool"
    initial: false
  - name: "reset"
    type: "bool" 
    initial: false
  - name: "index"
    type: "int"
    initial: 0

blocks:
  - name: "test_seq"
    type: "SEQUENCER"
    inputs:
      trigger: "trigger"
      reset: "reset"
    outputs:
      index: "index"
    params:
      max: 3
"#;

    let config = soft_plc::engine::PlcConfig::from_yaml(yaml)?;
    let mut engine = ScanEngine::new(config)?;
    
    // Initial state
    engine.execute_blocks()?;
    assert_eq!(engine.signal_bus().get_int("index")?, 0);
    println!("  Initial index: 0");
    
    // Trigger sequence
    for expected in 1..6 {
        // Rising edge
        engine.signal_bus().set("trigger", SignalValue::Bool(true))?;
        engine.execute_blocks()?;
        
        // Falling edge
        engine.signal_bus().set("trigger", SignalValue::Bool(false))?;
        engine.execute_blocks()?;
        
        let index = engine.signal_bus().get_int("index")?;
        let expected_wrapped = expected % 3;
        println!("  After trigger {}: index = {} (expected: {})", 
                 expected, index, expected_wrapped);
        assert_eq!(index, expected_wrapped);
    }
    
    // Test reset
    engine.signal_bus().set("reset", SignalValue::Bool(true))?;
    engine.execute_blocks()?;
    let index = engine.signal_bus().get_int("index")?;
    println!("  After reset: index = {} (expected: 0)", index);
    assert_eq!(index, 0);
    
    println!("✓ Test 3 passed!");
    Ok(())
}
