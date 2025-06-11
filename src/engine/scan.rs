use crate::{Result, signal::SignalBus, blocks};
use crate::engine::config::PlcConfig;
use crate::signal::SignalValue;
use tokio::time::{interval, Duration};
use tracing::{info, error, debug};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ScanEngine {
    config: PlcConfig,
    signal_bus: SignalBus,
    blocks: Vec<Box<dyn blocks::BlockTrait>>,
    running: Arc<RwLock<bool>>,
    scan_count: u64,
}

impl ScanEngine {
    pub fn new(config: PlcConfig) -> Result<Self> {
        let signal_bus = SignalBus::new();
        
        // Initialize signals
        for signal_config in &config.signals {
            let initial_value = signal_config.to_signal_value()?;
            signal_bus.set(&signal_config.name, initial_value)?;
            debug!("Initialized signal '{}' with type '{}'", 
                signal_config.name, signal_config.signal_type);
        }
        
        // Create blocks
        let mut blocks = Vec::new();
        for block_config in &config.blocks {
            let block = blocks::create_block(block_config)?;
            info!("Created block '{}' of type '{}'", 
                block_config.name, block_config.block_type);
            blocks.push(block);
        }
        
        Ok(Self {
            config,
            signal_bus,
            blocks,
            running: Arc::new(RwLock::new(false)),
            scan_count: 0,
        })
    }
    
    pub fn from_file(config_path: &str) -> Result<Self> {
        let config = PlcConfig::from_file(config_path)?;
        Self::new(config)
    }
    
    pub fn signal_bus(&self) -> &SignalBus {
        &self.signal_bus
    }
    
    // Add method to execute blocks manually (for testing)
    pub fn execute_blocks(&mut self) -> Result<()> {
        for block in &mut self.blocks {
            block.execute(&self.signal_bus)?;
        }
        Ok(())
    }
    
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting scan engine with {}ms scan time", self.config.scan_time_ms);
        
        {
            let mut running = self.running.write().await;
            *running = true;
        }
        
        let mut scan_interval = interval(Duration::from_millis(self.config.scan_time_ms));
        
        while *self.running.read().await {
            scan_interval.tick().await;
            
            let scan_start = std::time::Instant::now();
            
            // Execute all blocks in order
            self.execute_blocks().ok();
            
            self.scan_count += 1;
            let scan_duration = scan_start.elapsed();
            
            if scan_duration.as_millis() > self.config.scan_time_ms as u128 {
                error!("Scan overrun: {}ms > {}ms", 
                    scan_duration.as_millis(), self.config.scan_time_ms);
            } else {
                debug!("Scan {} completed in {:?}", self.scan_count, scan_duration);
            }
        }
        
        info!("Scan engine stopped after {} scans", self.scan_count);
        Ok(())
    }
    
    pub async fn stop(&self) {
        info!("Stopping scan engine...");
        let mut running = self.running.write().await;
        *running = false;
    }
    
    pub fn is_running(&self) -> bool {
        // Try to read without blocking
        self.running.try_read().map(|r| *r).unwrap_or(false)
    }
    
    pub fn scan_count(&self) -> u64 {
        self.scan_count
    }
    
    pub fn dump_signals(&self) -> Vec<(String, SignalValue)> {
        self.signal_bus.iter()
    }
}
