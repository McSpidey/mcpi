// mcpi-common/src/plugin.rs
use serde_json::Value;
use std::error::Error;

// Plugin type to distinguish between core and extension plugins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    Core,    // Built-in core functionality
    Extension, // Add-on functionality
}

/// Trait that defines the interface for MCPI plugins
pub trait McpPlugin: Send + Sync {
    /// Get the unique name of this plugin
    fn name(&self) -> &str;
    
    /// Get plugin description
    fn description(&self) -> &str;
    
    /// Get the category this plugin belongs to
    fn category(&self) -> &str;
    
    /// Get the type of this plugin
    fn plugin_type(&self) -> PluginType {
        PluginType::Extension // Default to extension
    }
    
    /// Get list of operations this plugin supports
    fn supported_operations(&self) -> Vec<String>;
    
    /// Get the input schema for this plugin
    fn input_schema(&self) -> Value;
    
    /// Execute an operation on this plugin
    fn execute(&self, operation: &str, params: &Value) -> Result<Value, Box<dyn Error + Send + Sync>>;
    
    /// Get capabilities this plugin provides
    fn get_capabilities(&self) -> Vec<String> {
        vec![self.name().to_string()]
    }
    
    /// Get resources this plugin provides
    fn get_resources(&self) -> Vec<(String, String, Option<String>)> {
        vec![(
            self.name().to_string(),
            format!("mcpi://provider/resources/{}", self.name()),
            Some(self.description().to_string()),
        )]
    }
}

/// Simplified result type for plugin operations
pub type PluginResult = Result<Value, Box<dyn Error + Send + Sync>>;