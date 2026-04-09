use std::env;
use std::sync::Once;

// Initialization flag to ensure we only initialize once
static INIT: Once = Once::new();

// Environment variable to control enhanced output
const ENV_ENHANCED_OUTPUT: &str = "REST_ENHANCED_OUTPUT";
const DEFAULT_ENHANCED_OUTPUT: bool = true;

/// Configuration for Rest's output and behavior
pub struct Config {
    pub(crate) use_colors: bool,
    pub(crate) use_unicode_symbols: bool,
    pub(crate) show_success_details: bool,
    /// Enable enhanced test output (fluent assertions instead of standard output)
    pub(crate) enhanced_output: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Clone for Config
impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            use_colors: self.use_colors,
            use_unicode_symbols: self.use_unicode_symbols,
            show_success_details: self.show_success_details,
            enhanced_output: self.enhanced_output,
        }
    }
}

impl Config {
    /// Creates a new configuration with default settings
    pub fn new() -> Self {
        Self::from_env(|key| env::var(key).ok())
    }

    /// Creates a new configuration by reading env vars through the provided closure.
    /// This allows tests to inject mock env values without mutating process-global state.
    fn from_env(get_var: impl Fn(&str) -> Option<String>) -> Self {
        let enhanced_output = match get_var(ENV_ENHANCED_OUTPUT) {
            Some(val) => bool_from_str(&val, DEFAULT_ENHANCED_OUTPUT),
            None => DEFAULT_ENHANCED_OUTPUT,
        };

        Self { use_colors: true, use_unicode_symbols: true, show_success_details: true, enhanced_output }
    }

    /// Enable or disable colored output
    pub fn use_colors(mut self, enable: bool) -> Self {
        self.use_colors = enable;
        self
    }

    /// Enable or disable Unicode symbols
    pub fn use_unicode_symbols(mut self, enable: bool) -> Self {
        self.use_unicode_symbols = enable;
        self
    }

    /// Control whether to show details for successful tests
    pub fn show_success_details(mut self, enable: bool) -> Self {
        self.show_success_details = enable;
        self
    }

    /// Enable or disable enhanced output (fluent assertions)
    pub fn enhanced_output(mut self, enable: bool) -> Self {
        self.enhanced_output = enable;
        self
    }

    /// Apply the configuration
    pub fn apply(self) {
        use crate::reporter::GLOBAL_CONFIG;

        // Clone self before moving it into the global config
        let config = self.clone();
        *GLOBAL_CONFIG.write().unwrap() = self;

        // Initialize the event system if enhanced output is enabled
        if config.enhanced_output {
            crate::initialize();
        }
    }
}

/// Initialize the Rest system
/// This is called automatically when needed but can also be called explicitly
pub fn initialize() {
    INIT.call_once(|| {
        // Check if enhanced output is enabled in the config
        let config = crate::reporter::GLOBAL_CONFIG.read().unwrap();

        if config.enhanced_output {
            // Initialize event system
            crate::events::EventEmitter::init();

            // Register event handlers
            crate::Reporter::init();
        }
    });
}

/// Check if enhanced output is enabled in the current configuration
pub fn is_enhanced_output_enabled() -> bool {
    let config = crate::reporter::GLOBAL_CONFIG.read().unwrap();
    return config.enhanced_output;
}

/// Convert from one of the allowed string values to a boolean.
fn bool_from_str(val: &str, default: bool) -> bool {
    match val.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => {
            eprintln!(
                "WARNING: Unrecognized value for environment variable {}: {:?}. Defaulting to {}. (Allowed values: true, false, 1, 0, yes, no, on, off)",
                ENV_ENHANCED_OUTPUT, val, default,
            );
            default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::from_env(|_| None);

        assert_eq!(config.use_colors, true);
        assert_eq!(config.use_unicode_symbols, true);
        assert_eq!(config.show_success_details, true);
        assert_eq!(config.enhanced_output, true); // Default is true without env var
    }

    #[test]
    fn test_config_env_var_true() {
        let config = Config::from_env(|_| Some("true".into()));
        assert_eq!(config.enhanced_output, true);
    }

    #[test]
    fn test_config_env_var_false() {
        let config = Config::from_env(|_| Some("false".into()));
        assert_eq!(config.enhanced_output, false);
    }

    #[test]
    fn test_config_env_var_alternative_values() {
        // True values
        assert_eq!(Config::from_env(|_| Some("1".into())).enhanced_output, true);
        assert_eq!(Config::from_env(|_| Some("yes".into())).enhanced_output, true);
        assert_eq!(Config::from_env(|_| Some("on".into())).enhanced_output, true);

        // False values
        assert_eq!(Config::from_env(|_| Some("0".into())).enhanced_output, false);
        assert_eq!(Config::from_env(|_| Some("no".into())).enhanced_output, false);
        assert_eq!(Config::from_env(|_| Some("off".into())).enhanced_output, false);

        // Case-insensitivity
        assert_eq!(Config::from_env(|_| Some("TRUE".into())).enhanced_output, true);
        assert_eq!(Config::from_env(|_| Some("False".into())).enhanced_output, false);

        // Garbage input falls back to default
        assert_eq!(Config::from_env(|_| Some("garbage".into())).enhanced_output, DEFAULT_ENHANCED_OUTPUT);
    }

    #[test]
    fn test_config_builder_methods() {
        let config = Config::new().use_colors(false).use_unicode_symbols(false).show_success_details(false).enhanced_output(true);

        assert_eq!(config.use_colors, false);
        assert_eq!(config.use_unicode_symbols, false);
        assert_eq!(config.show_success_details, false);
        assert_eq!(config.enhanced_output, true);
    }

    #[test]
    fn test_config_clone() {
        let config1 = Config::from_env(|_| None).use_colors(false).enhanced_output(true);

        let config2 = config1.clone();

        assert_eq!(config1.use_colors, config2.use_colors);
        assert_eq!(config1.use_unicode_symbols, config2.use_unicode_symbols);
        assert_eq!(config1.show_success_details, config2.show_success_details);
        assert_eq!(config1.enhanced_output, config2.enhanced_output);
    }

    #[test]
    fn test_bool_from_str() {
        assert_eq!(bool_from_str("true", false), true);
        assert_eq!(bool_from_str("false", true), false);
        assert_eq!(bool_from_str("1", false), true);
        assert_eq!(bool_from_str("0", true), false);
        assert_eq!(bool_from_str("yes", false), true);
        assert_eq!(bool_from_str("no", true), false);
        assert_eq!(bool_from_str("on", false), true);
        assert_eq!(bool_from_str("off", true), false);
        assert_eq!(bool_from_str("invalid", true), true);
        assert_eq!(bool_from_str("invalid", false), false);
    }

    #[test]
    fn test_config_apply_sets_global_state() {
        // Apply a config with specific settings
        let config = Config::new().use_colors(false).use_unicode_symbols(false).show_success_details(false).enhanced_output(true);
        config.apply();

        // Verify via the global config
        {
            let global = crate::reporter::GLOBAL_CONFIG.read().unwrap();
            assert_eq!(global.use_colors, false);
            assert_eq!(global.use_unicode_symbols, false);
            assert_eq!(global.show_success_details, false);
            assert_eq!(global.enhanced_output, true);
        }

        // Restore defaults
        Config::new().apply();
    }

    #[test]
    fn test_config_apply_enhanced_output_flag() {
        let config = Config::new().enhanced_output(true);
        config.apply();

        assert!(is_enhanced_output_enabled(), "enhanced output should be enabled after apply");

        // Restore defaults
        Config::new().apply();
    }
}
