use chrono::Utc;

use super::logger;

use super::LogLevel;

pub trait LoggingHw {
    fn hw_name(&self) -> &'static str;
    fn get_ctx(&self) -> Option<String> {
        None
    }

    fn log(&self, level: LogLevel, operation: &str, message: &str) {
        let hw = self.hw_name();

        if !logger().is_log_level_enabled_for_hw(hw, level) {
            return;
        }

        let mut log_string_builder = String::new();
        let use_colors = logger().use_colors();

        if use_colors {
            log_string_builder.push_str(level.ansi_color_code());
        }

        log_string_builder.push_str(&format!("[{}] [{}]::[{}] ", level.as_str(), hw, operation));

        match self.get_ctx() {
            Some(ctx) => {
                log_string_builder.push_str(&format!("[{}] ", ctx));
            }
            None => {}
        }

        if logger().use_timestamps() {
            log_string_builder.push_str(&format!("[{}]", Utc::now().format("%Y-%m-%d %H:%M:%S")));
        }

        log_string_builder.push_str(&format!(": {}", message));

        if use_colors {
            log_string_builder.push_str(LogLevel::ANSI_RESET);
        }

        println!("{}", log_string_builder);
    }

    fn log_verbose(&self, operation: &str, message: &str) {
        self.log(LogLevel::Verbose, operation, message);
    }

    fn log_debug(&self, operation: &str, message: &str) {
        self.log(LogLevel::Debug, operation, message);
    }

    fn log_info(&self, operation: &str, message: &str) {
        self.log(LogLevel::Info, operation, message);
    }

    fn log_warning(&self, operation: &str, message: &str) {
        self.log(LogLevel::Warn, operation, message);
    }

    fn log_error(&self, operation: &str, message: &str) {
        self.log(LogLevel::Error, operation, message);
    }
}
