pub mod bot;
pub use bot::{HoneyPotBot, channels_of_interest};
pub mod error;
pub mod log;

pub use miette::Result;

/// Default imports for binary.
/// 
/// ```rust,no-run
/// use discord
/// ```
pub mod prelude {
    pub use crate::bot::HoneyPotBot;
    pub use miette::Result;
}
