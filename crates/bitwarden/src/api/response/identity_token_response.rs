use serde::{Deserialize, Serialize};

use crate::{
    api::response::{
        IdentityCaptchaResponse, IdentityTokenFailResponse, IdentityTokenPayloadResponse,
        IdentityTokenRefreshResponse, IdentityTokenSuccessResponse, IdentityTwoFactorResponse,
    },
    error::{Error, Result},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IdentityTokenResponse {
    Authenticated(IdentityTokenSuccessResponse),
    Payload(IdentityTokenPayloadResponse),
    Refreshed(IdentityTokenRefreshResponse),
    TwoFactorRequired(IdentityTwoFactorResponse),
    CaptchaRequired(IdentityCaptchaResponse),
}

pub fn parse_identity_response(response: &str) -> Result<IdentityTokenResponse> {
    if let Ok(r) = serde_json::from_str::<IdentityTokenSuccessResponse>(response) {
        Ok(IdentityTokenResponse::Authenticated(r))
    } else if let Ok(r) = serde_json::from_str::<IdentityTokenPayloadResponse>(response) {
        Ok(IdentityTokenResponse::Payload(r))
    } else if let Ok(r) = serde_json::from_str::<IdentityTokenRefreshResponse>(response) {
        Ok(IdentityTokenResponse::Refreshed(r))
    } else if let Ok(r) = serde_json::from_str::<IdentityTwoFactorResponse>(response) {
        Ok(IdentityTokenResponse::TwoFactorRequired(r))
    } else if let Ok(r) = serde_json::from_str::<IdentityCaptchaResponse>(response) {
        Ok(IdentityTokenResponse::CaptchaRequired(r))
    } else if let Ok(r) = serde_json::from_str::<IdentityTokenFailResponse>(response) {
        Err(Error::IdentityFail(r))
    } else {
        Err(Error::Internal("Failed to parse IdentityTokenResponse"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success() {
        let expected = IdentityTokenSuccessResponse::default();
        let success = serde_json::to_string(&expected).unwrap();
        let expected = IdentityTokenResponse::Authenticated(expected);
        let actual = parse_identity_response(&success).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn two_factor() {
        let expected = IdentityTwoFactorResponse::default();
        let two_factor = serde_json::to_string(&expected).unwrap();
        let expected = IdentityTokenResponse::TwoFactorRequired(expected);
        let actual = parse_identity_response(&two_factor).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn captcha() {
        let expected = IdentityCaptchaResponse::default();
        let captcha = serde_json::to_string(&expected).unwrap();
        let expected = IdentityTokenResponse::CaptchaRequired(expected);
        let actual = parse_identity_response(&captcha).unwrap();
        assert_eq!(expected, actual);
    }
}
