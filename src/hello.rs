use windows::{
    core::HSTRING,
    Security::Credentials::{
        KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialStatus,
    },
};

use crate::AuthErrors;

pub fn win_hello() -> Result<bool, AuthErrors> {
    let supported = KeyCredentialManager::IsSupportedAsync()?.get()?;
    if supported {
        let result = KeyCredentialManager::RequestCreateAsync(
            &HSTRING::from("login"),
            KeyCredentialCreationOption::ReplaceExisting,
        )?.get()?;
        return if result.Status().unwrap() == KeyCredentialStatus::Success {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        return Err(AuthErrors::Unsupported())
    }
}