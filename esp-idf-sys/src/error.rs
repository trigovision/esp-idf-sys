use cstr_core::CStr;

use crate::esp_err_t;
use core::fmt::Write;
use core::{
    fmt::{Debug, Display},
    ops::Try,
};

/// Error types mapped to ESP-IDF failure values.
#[derive(Debug)]
pub enum EspError {
    NoMem,
    InvalidArg,
    InvalidState,
    InvalidSize,
    NotFound,
    NotSupported,
    Timeout,
    InvalidResponse,
    InvalidCrc,
    InvalidVersion,
    InvalidMac,
    WifiBase,
    MeshBase,
    NVSNoFreePages,
    NVSNewVersionFound,
    WifiNotStarted,
    Unknown(i32),
}

/// Failed to convert an integer into an `enum` value
pub struct EnumFromIntError(esp_err_t);

/// Attempts to convert int returned from ESP-IDF into error `enum`
impl core::convert::From<esp_err_t> for EspError {
    fn from(value: esp_err_t) -> Self {
        use self::EspError::*;
        match value.0 as u32 {
            crate::ESP_ERR_NO_MEM => NoMem,
            crate::ESP_ERR_INVALID_ARG => InvalidArg,
            crate::ESP_ERR_INVALID_STATE => InvalidState,
            crate::ESP_ERR_INVALID_SIZE => InvalidSize,
            crate::ESP_ERR_NOT_FOUND => NotFound,
            crate::ESP_ERR_NOT_SUPPORTED => NotSupported,
            crate::ESP_ERR_TIMEOUT => Timeout,
            crate::ESP_ERR_INVALID_RESPONSE => InvalidResponse,
            crate::ESP_ERR_INVALID_CRC => InvalidCrc,
            crate::ESP_ERR_INVALID_VERSION => InvalidVersion,
            crate::ESP_ERR_INVALID_MAC => InvalidMac,
            crate::ESP_ERR_WIFI_BASE => WifiBase,
            crate::ESP_ERR_MESH_BASE => MeshBase,
            crate::ESP_ERR_NVS_NO_FREE_PAGES => NVSNoFreePages,
            crate::ESP_ERR_NVS_NEW_VERSION_FOUND => NVSNewVersionFound,
            crate::ESP_ERR_WIFI_NOT_STARTED => WifiNotStarted,
            _ => Unknown(value.0),
        }
    }
}

impl core::convert::From<EspError> for esp_err_t {
    fn from(value: EspError) -> Self {
        use self::EspError::*;
        let value = match value {
            NoMem => crate::ESP_ERR_NO_MEM,
            InvalidArg => crate::ESP_ERR_INVALID_ARG,
            InvalidState => crate::ESP_ERR_INVALID_STATE,
            InvalidSize => crate::ESP_ERR_INVALID_SIZE,
            NotFound => crate::ESP_ERR_NOT_FOUND,
            NotSupported => crate::ESP_ERR_NOT_SUPPORTED,
            Timeout => crate::ESP_ERR_TIMEOUT,
            InvalidResponse => crate::ESP_ERR_INVALID_RESPONSE,
            InvalidCrc => crate::ESP_ERR_INVALID_CRC,
            InvalidVersion => crate::ESP_ERR_INVALID_VERSION,
            InvalidMac => crate::ESP_ERR_INVALID_MAC,
            WifiBase => crate::ESP_ERR_WIFI_BASE,
            MeshBase => crate::ESP_ERR_MESH_BASE,
            NVSNoFreePages => crate::ESP_ERR_NVS_NO_FREE_PAGES,
            NVSNewVersionFound => crate::ESP_ERR_NVS_NEW_VERSION_FOUND,
            WifiNotStarted => crate::ESP_ERR_WIFI_NOT_STARTED,
            Unknown(value) => value as _,
        };

        Self(value as _)
    }
}

impl Try for esp_err_t {
    type Ok = ();

    type Error = EspError;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        if self.0 == 0 {
            Ok(())
        } else {
            Err(self.into())
        }
    }

    fn from_error(v: Self::Error) -> Self {
        v.into()
    }

    fn from_ok(_v: Self::Ok) -> Self {
        Self(0)
    }
}

impl Display for EspError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EspError::Unknown(code) => {
                let err_name = unsafe { CStr::from_ptr(crate::esp_err_to_name(esp_err_t(*code))) };

                f.write_fmt(format_args!("{:#x} ({:?})", code, err_name))
            }
            _ => Debug::fmt(&self, f),
        }
    }
}

impl From<EspError> for anyhow::Error {
    fn from(e: EspError) -> Self {
        anyhow::Error::msg("EspError").context(e)
    }
}
