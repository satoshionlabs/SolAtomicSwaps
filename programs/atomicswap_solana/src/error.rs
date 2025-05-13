use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    #[msg("Invalid secret key")]
    InvalidSecretKey,
    #[msg("Invalid seller pubkey")]
    InvalidSellerPubkey,
    #[msg("Invalid buyer pubkey")]
    InvalidBuyerPubkey,
    #[msg("Invalid redeem time")]
    InvalidRedeemTime,
    #[msg("Invalid refund time")]
    InvalidRefundTime,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid status")]
    InvalidStatus,
    #[msg("Not enough balance")]
    NotEnoughBalance,
    #[msg("Can not redeem")]
    CanNotRedeem,
    #[msg("Can not refund")]
    CanNotRefund,
}
