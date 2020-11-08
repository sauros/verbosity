use ansi_term::Colour::{Cyan};

/// Show the level of verbosity iff verbosity level !off
#[macro_export]
macro_rules! verbosity_show {
    ($v:expr) => {
        vlow!($v, "Verbosity Display", format!("Current level <{}>", $v.get_level()).as_str());
    }
}

/// Show something iff level >= "low"
#[macro_export]
macro_rules! vlow {
    ($verbosity_obj:expr, $origin:expr, $output:expr) => {
        $verbosity_obj.out(verbosity::Level::Low, $origin, $output)
    }
}

/// Show something iff level >= "medium"
#[macro_export]
macro_rules! vmed {
    ($verbosity_obj:expr, $origin:expr, $output:expr) => {
        $verbosity_obj.out(verbosity::Level::Medium, $origin, $output)
    }
}

/// Show something iff level >= "high"
#[macro_export]
macro_rules! vhigh {
    ($verbosity_obj:expr, $origin:expr, $output:expr) => {
        $verbosity_obj.out(verbosity::Level::High, $origin, $output)
    }
}

/// Make a new Verbosity object instantiated at a "low" setting
#[macro_export]
macro_rules! verbosity_low {
    () => {
        verbosity::Verbosity::new(verbosity::Level::Low)
    }
}

/// Make a new Verbosity object instantiated at a "medium" setting
#[macro_export]
macro_rules! verbosity_medium {
    () => {
        verbosity::Verbosity::new(verbosity::Level::Medium)
    }
}

/// Make a new Verbosity object instantiated at a "high" setting
#[macro_export]
macro_rules! verbosity_high {
    () => {
        verbosity::Verbosity::new(verbosity::Level::High)
    }
}

/// Make a new Verbosity object instantiated at a "off" setting
#[macro_export]
macro_rules! verbosity_off {
    () => {
        verbosity::Verbosity::new(verbosity::Level::Off)
    }
}

/// Get the current function as a name that can be used in outputs
#[macro_export]
macro_rules! vfunc {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

/// Level of Verbosity that can be set
pub enum Level {
    Off,
    Low,
    Medium,
    High,
}

/// Verbosity object
pub struct Verbosity
{
    level: Level,
}

impl Verbosity {

    /// Create a new verbosity object set to a specific level
    pub fn new(level: Level) -> Self {
        Self {
            level: level
        }
    }

    /// Get a string representation of the level
    pub fn get_level(&self) -> &str {
        match self.level {
            Level::Off    => { "off" }
            Level::Low    => { "low" }
            Level::Medium => { "medium"}
            Level::High   => { "high"}
        }
    }

    /// Write something out taking into account the level to write at vs the level verbosity is set to
    pub fn out(&self, level: Level, source: &str, message: &str) {

        match self.level {
            Level::Off => { return; }

            Level::Low => {
                match level {
                    Level::Low => { println!("({}) : {} ", Cyan.paint(source), message); }
                    _ => { return; }
                }
            }

            Level::Medium => {
                match level {
                    Level::Low    => { println!("({}) : {} ", Cyan.paint(source), message); }
                    Level::Medium => { println!("({}) : {} ", Cyan.paint(source), message); }
                    _ => { return; }
                }
            }

            Level::High => {
                match level {
                    Level::Off => { return; }
                    _          => { println!("({}) : {} ", Cyan.paint(source), message); }
                }
            }
        }
    }
}