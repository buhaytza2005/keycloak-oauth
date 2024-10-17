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
}
