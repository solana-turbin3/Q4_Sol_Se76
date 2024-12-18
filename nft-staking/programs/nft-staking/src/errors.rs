use anchor_lang::error_code;


#[error_code]
pub enum Error {
    #[msg("Max stake reached")]
    MaxStakeReached,
    #[msg("Frezze period not passed")]
    FreezePeriodNotPassed,
}