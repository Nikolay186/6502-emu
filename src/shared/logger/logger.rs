use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Mutex, OnceLock,
    },
};

use crate::shared::logger::log_lvl_enum::LogLevel;

pub struct Logger {
    is_enabled: AtomicBool,
    global_level: AtomicU8,
    hw_levels: Mutex<HashMap<&'static str, LogLevel>>,
    use_colors: AtomicBool,
    use_timestamps: AtomicBool,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() -> &'static Logger {
    LOGGER.get_or_init(|| Logger::new())
}

impl Logger {
    fn new() -> Self {
        Self {
            is_enabled: AtomicBool::new(true),
            global_level: AtomicU8::new(LogLevel::Info.into()),
            hw_levels: Mutex::new(HashMap::with_capacity(5)),
            use_colors: AtomicBool::new(true),
            use_timestamps: AtomicBool::new(true),
        }
    }

    pub fn is_log_level_enabled_for_hw(&self, hw: &'static str, log_level: LogLevel) -> bool {
        if !self.is_enabled.load(Ordering::Relaxed) {
            return false;
        }

        let hw_levels_map = self.hw_levels.lock().unwrap();
        if let Some(hw_level) = hw_levels_map.get(hw) {
            return log_level >= *hw_level;
        }

        let global_level = self.global_level.load(Ordering::Relaxed);

        log_level >= global_level.into()
    }

    pub fn set_global_level(&self, level: LogLevel) {
        self.global_level.store(level.into(), Ordering::Relaxed);
    }

    pub fn set_hw_level(&self, hw: &'static str, level: LogLevel) {
        let mut hw_levels_map = self.hw_levels.lock().unwrap();
        hw_levels_map.insert(hw, level);
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.is_enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn use_colors(&self) -> bool {
        self.use_colors.load(Ordering::Relaxed)
    }

    pub fn set_use_colors(&self, use_colors: bool) {
        self.use_colors.store(use_colors, Ordering::Relaxed);
    }

    pub fn use_timestamps(&self) -> bool {
        self.use_timestamps.load(Ordering::Relaxed)
    }

    pub fn set_use_timestamps(&self, use_timestamps: bool) {
        self.use_timestamps.store(use_timestamps, Ordering::Relaxed);
    }
}
