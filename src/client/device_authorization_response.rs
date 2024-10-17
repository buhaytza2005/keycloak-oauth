use std::str::FromStr;

use oauth2::{DeviceCodeErrorResponseType, StandardErrorResponse};

pub enum PollDeviceCodeEvent {
    AuthorizationPending,
    AuthorizationDeclined,
    BadVerificationCode,
    ExpiredToken,
    AccessDenied,
    SlowDown,
}

impl PollDeviceCodeEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            PollDeviceCodeEvent::AuthorizationPending => "authorization_pending",
            PollDeviceCodeEvent::AuthorizationDeclined => "authorization_declined",
            PollDeviceCodeEvent::BadVerificationCode => "bad_verification_code",
            PollDeviceCodeEvent::ExpiredToken => "expired_token",
            PollDeviceCodeEvent::AccessDenied => "access_denied",
            PollDeviceCodeEvent::SlowDown => "slow_down",
        }
    }
    pub fn as_message(&self) {
        match self {
            PollDeviceCodeEvent::AuthorizationPending => {
                println!("authorization_pending, continuing")
            }
            PollDeviceCodeEvent::AuthorizationDeclined => {
                println!("authorization_declined! exiting loop")
            }
            PollDeviceCodeEvent::BadVerificationCode => {
                println!("bad_verification_code! continuing")
            }
            PollDeviceCodeEvent::ExpiredToken => println!("expired_token, exiting loop"),
            PollDeviceCodeEvent::AccessDenied => println!("access_denied, exiting loop"),
            PollDeviceCodeEvent::SlowDown => println!("slow_down! adding 5 sec to interval"),
        }
    }
}

impl From<StandardErrorResponse<DeviceCodeErrorResponseType>> for PollDeviceCodeEvent {
    fn from(value: StandardErrorResponse<DeviceCodeErrorResponseType>) -> Self {
        let resp = match value.error() {
            DeviceCodeErrorResponseType::AccessDenied => PollDeviceCodeEvent::AccessDenied,
            DeviceCodeErrorResponseType::AuthorizationPending => {
                PollDeviceCodeEvent::AuthorizationPending
            }
            DeviceCodeErrorResponseType::SlowDown => PollDeviceCodeEvent::SlowDown,
            DeviceCodeErrorResponseType::ExpiredToken => PollDeviceCodeEvent::ExpiredToken,
            DeviceCodeErrorResponseType::Basic(basic_error_response_type) => {
                PollDeviceCodeEvent::from_str(basic_error_response_type.to_string().as_str())
                    .expect("parsed")
            }
        };

        resp
    }
}

impl FromStr for PollDeviceCodeEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authorization_pending" => Ok(PollDeviceCodeEvent::AuthorizationPending),
            "authorization_declined" => Ok(PollDeviceCodeEvent::AuthorizationDeclined),
            "bad_verification_code" => Ok(PollDeviceCodeEvent::BadVerificationCode),
            "expired_token" => Ok(PollDeviceCodeEvent::ExpiredToken),
            "access_denied" => Ok(PollDeviceCodeEvent::AccessDenied),
            "slow_down" => Ok(PollDeviceCodeEvent::SlowDown),
            _ => Err(()),
        }
    }
}
