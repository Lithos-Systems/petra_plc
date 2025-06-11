pub mod traits;
pub mod basic;
pub mod timers;
pub mod triggers;
pub mod counters;

use crate::{Result, PlcError};
use traits::Block;

// Re-export commonly used items
pub use traits::Block as BlockTrait;
pub use traits::BlockConfig;

/// Factory function to create blocks from configuration
pub fn create_block(config: &BlockConfig) -> Result<Box<dyn Block>> {
    match config.block_type.as_str() {
        // Logic blocks
        "AND" => Ok(Box::new(basic::AndBlock::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        "OR" => Ok(Box::new(basic::OrBlock::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        "NOT" => Ok(Box::new(basic::NotBlock::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        // Comparison blocks
        "EQ" => Ok(Box::new(basic::EqBlock::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        "GT" => Ok(Box::new(basic::GtBlock::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        "LT" => Ok(Box::new(basic::LtBlock::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        // Trigger blocks
        "R_TRIG" => Ok(Box::new(triggers::RTrig::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        "F_TRIG" => Ok(Box::new(triggers::FTrig::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        "SR_LATCH" => Ok(Box::new(triggers::SRLatch::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
        )?)),
        
        // Timer blocks
        "TON" => Ok(Box::new(timers::TON::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
            &config.params,
        )?)),
        
        "TOF" => Ok(Box::new(timers::TOF::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
            &config.params,
        )?)),
        
        "TP" => Ok(Box::new(timers::TP::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
            &config.params,
        )?)),
        
        // Counter blocks
        "COUNTER" => Ok(Box::new(counters::Counter::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
            &config.params,
        )?)),
        
        "SEQUENCER" => Ok(Box::new(counters::Sequencer::new(
            config.name.clone(),
            &config.inputs,
            &config.outputs,
            &config.params,
        )?)),
        
        // Utility blocks
        "CONST" => Ok(Box::new(basic::ConstBlock::new(
            config.name.clone(),
            &config.outputs,
            &config.params,
        )?)),
        
        _ => Err(PlcError::ConfigError(format!(
            "Unknown block type: {}",
            config.block_type
        ))),
    }
}
