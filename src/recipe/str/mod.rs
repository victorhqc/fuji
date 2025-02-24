use snafu::Snafu;
use strum_macros::Display;

mod clarity;
mod color;
mod color_chrome_effect;
mod color_chrome_fx_blue;
mod film_simulation;
mod grain_effect;
mod high_iso_noise_reduction;
mod monochromatic_color;
mod monochromatic_shift;
mod sharpness;
mod tone_curve;
mod wb_shift;
mod white_balance;

#[derive(Debug, Snafu, PartialEq)]
pub enum Error {
    #[snafu(display("Failed to Parse {}: {}", key, reason))]
    Parse { key: ParseKey, reason: String },
}

#[derive(Debug, Display, PartialEq)]
pub enum ParseKey {
    FilmSimulation,
    WhiteBalance,
    WhiteBalanceShift,
    GrainEffect,
    MonochromaticColor,
    MonochromaticColorShift,
}
