// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net
// Commercial License, version 1.0 or later, or (2) The General Public License
// (GPL), version 3, depending on which licence you accepted on initial access
// to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project
// generally, you agree to be bound by the terms of the MaidSafe Contributor
// Agreement, version 1.0.
// This, along with the Licenses can be found in the root directory of this
// project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network
// Software distributed under the GPL Licence is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
//
// Please review the Licences for the specific language governing permissions
// and limitations relating to use of the SAFE Network Software.

use core::CoreError;
use core::SelfEncryptionStorageError;
use futures::sync::mpsc::SendError;
use maidsafe_utilities::serialisation::SerialisationError;
use self_encryption::SelfEncryptionError;
use std::error::Error;
use std::io::Error as IoError;
use std::sync::mpsc::{RecvError, RecvTimeoutError};

/// App error.
#[derive(Debug)]
pub enum AppError {
    /// Error from safe_core.
    CoreError(CoreError),
    /// Could not serialise or deserialise data
    SerialisationError(SerialisationError),
    /// Forbidden operation
    Forbidden,

    /// Invalid CipherOpt handle
    InvalidCipherOptHandle,
    /// Invalid encrypt (box_) key handle
    InvalidEncryptKeyHandle,
    /// Invalid MutableData enties handle
    InvalidMDataEntriesHandle,
    /// Invalid MutableData entry actions handle
    InvalidMDataEntryActionsHandle,
    /// Invalid MutableData permissions handle
    InvalidMDataPermissionsHandle,
    /// Invalid MutableData permission set handle
    InvalidMDataPermissionSetHandle,
    /// Invalid Self Encryptor handle
    InvalidSelfEncryptorHandle,
    /// Invalid sign key handle
    InvalidSignKeyHandle,
    /// Invalid XorName handle
    InvalidXorNameHandle,

    /// Error while self-encrypting data
    SelfEncryption(SelfEncryptionError<SelfEncryptionStorageError>),
    /// Invalid offsets (from-position and lenght combination) provided for
    /// reading form SelfEncryptor. Would have probably caused an overflow.
    InvalidSelfEncryptorReadOffsets,
    /// Input/output Error
    IoError(IoError),
    /// Unexpected error
    Unexpected(String),
}

impl From<CoreError> for AppError {
    fn from(err: CoreError) -> Self {
        AppError::CoreError(err)
    }
}

impl From<SerialisationError> for AppError {
    fn from(err: SerialisationError) -> Self {
        AppError::SerialisationError(err)
    }
}

impl From<SelfEncryptionError<SelfEncryptionStorageError>> for AppError {
    fn from(err: SelfEncryptionError<SelfEncryptionStorageError>) -> Self {
        AppError::SelfEncryption(err)
    }
}

impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::IoError(err)
    }
}

impl<'a> From<&'a str> for AppError {
    fn from(s: &'a str) -> Self {
        AppError::Unexpected(s.to_string())
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Unexpected(s)
    }
}

impl<T: 'static> From<SendError<T>> for AppError {
    fn from(err: SendError<T>) -> Self {
        AppError::from(err.description())
    }
}

impl From<RecvError> for AppError {
    fn from(err: RecvError) -> Self {
        AppError::from(err.description())
    }
}

impl From<RecvTimeoutError> for AppError {
    fn from(err: RecvTimeoutError) -> Self {
        AppError::from(err.description())
    }
}

// const APP_ERROR_START_RANGE: i32 = 0;

/// Conversion to integer error code.
impl Into<i32> for AppError {
    fn into(self) -> i32 {
        unimplemented!()
    }
}
