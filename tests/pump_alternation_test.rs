use soft_plc::{
    signal::SignalValue,
    engine::ScanEngine,
    Result,
};

#[tokio::test]
async fn test_pump_alternation() -> Result<()> {
    // Load the pump alternation configuration
    let yaml_config = include_str!("../config/pump_alternation.yaml");
    let config = soft_plc::engine::PlcConfig::from_yaml(yaml_config)?;
    let mut engine = ScanEngine::new(config)?;
    
    // Initial state - pressure is OK (55.0)
    engine.execute_blocks()?;
    assert_eq!(engine.signal_bus().get_bool("pump1_run")?, false);
    assert_eq!(engine.signal_bus().get_int("pump_index")?, 0);
    
    println!("Initial state - all pumps off, index=0");
    
    // Simulate pressure drop below start setpoint
    engine.signal_bus().set("pressure", SignalValue::Float(45.0))?;
    engine.execute_blocks()?;
    engine.execute_blocks()?; // Need two scans for edge detection
    
    // Pump 1 should start (index 0)
    assert_eq!(engine.signal_bus().get_bool("pump1_run")?, true);
    assert_eq!(engine.signal_bus().get_bool("pump2_run")?, false);
    assert_eq!(engine.signal_bus().get_int("pump_index")?, 0);
    println!("Low pressure detected - Pump 1 started");
    
    // Simulate pressure recovery
    engine.signal_bus().set("pressure", SignalValue::Float(65.0))?;
    engine.execute_blocks()?;
    engine.execute_blocks()?;
    
    // All pumps should stop
    assert_eq!(engine.signal_bus().get_bool("pump1_run")?, false);
    println!("Pressure recovered - Pump 1 stopped");
    
    // Second pressure drop - should start pump 2
    engine.signal_bus().set("pressure", SignalValue::Float(45.0))?;
    engine.execute_blocks()?;
    engine.execute_blocks()?;
    
    assert_eq!(engine.signal_bus().get_bool("pump1_run")?, false);
    assert_eq!(engine.signal_bus().get_bool("pump2_run")?, true);
    assert_eq!(engine.signal_bus().get_int("pump_index")?, 1);
    println!("Second low pressure - Pump 2 started");
    
    // Recover pressure
    engine.signal_bus().set("pressure", SignalValue::Float(65.0))?;
    engine.execute_blocks()?;
    engine.execute_blocks()?;
    
    // Test cycling through all pumps
    for expected_index in 2..5 {
        engine.signal_bus().set("pressure", SignalValue::Float(45.0))?;
        engine.execute_blocks()?;
        engine.execute_blocks()?;
        
        assert_eq!(engine.signal_bus().get_int("pump_index")?, expected_index);
        println!("Pump {} started (index {})", expected_index + 1, expected_index);
        
        engine.signal_bus().set("pressure", SignalValue::Float(65.0))?;
        engine.execute_blocks()?;
        engine.execute_blocks()?;
    }
    
    // Next cycle should wrap back to pump 1
    engine.signal_bus().set("pressure", SignalValue::Float(45.0))?;
    engine.execute_blocks()?;
    engine.execute_blocks()?;
    
    assert_eq!(engine.signal_bus().get_bool("pump1_run")?, true);
    assert_eq!(engine.signal_bus().get_int("pump_index")?, 0);
    println!("Wrapped back to Pump 1");
    
    // Test reset functionality
    engine.signal_bus().set("system_reset", SignalValue::Bool(true))?;
    engine.execute_blocks()?;
    
    assert_eq!(engine.signal_bus().get_int("pump_index")?, 0);
    println!("System reset - index back to 0");
    
    // Test manual override
    engine.signal_bus().set("manual_override", SignalValue::Bool(true))?;
    engine.signal_bus().set("system_reset", SignalValue::Bool(false))?;
    engine.signal_bus().set("pressure", SignalValue::Float(45.0))?;
    engine.execute_blocks()?;
    engine.execute_blocks()?;
    
    // No pumps should run in manual mode
    assert_eq!(engine.signal_bus().get_bool("pump1_run")?, false);
    println!("Manual override active - no auto pump control");
    
    Ok(())
}
