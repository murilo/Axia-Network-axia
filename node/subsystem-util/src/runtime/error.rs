// Copyright 2021 AXIA Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.
//

//! Error handling related code and Error/Result definitions.

use futures::channel::oneshot;
use thiserror::Error;

use axia_node_subsystem::errors::RuntimeApiError;
use axia_primitives::v1::SessionIndex;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors for `Runtime` cache.
#[derive(Debug, Error, derive_more::From)]
#[error(transparent)]
pub enum Error {
	/// All fatal errors.
	Fatal(Fatal),
	/// All nonfatal/potentially recoverable errors.
	NonFatal(NonFatal),
}

/// Fatal runtime errors.
#[derive(Debug, Error)]
pub enum Fatal {
	/// Runtime API subsystem is down, which means we're shutting down.
	#[error("Runtime request got canceled")]
	RuntimeRequestCanceled(oneshot::Canceled),
}

/// Errors for fetching of runtime information.
#[derive(Debug, Error)]
pub enum NonFatal {
	/// Some request to the runtime failed.
	/// For example if we prune a block we're requesting info about.
	#[error("Runtime API error")]
	RuntimeRequest(RuntimeApiError),

	/// We tried fetching a session info which was not available.
	#[error("There was no session with the given index")]
	NoSuchSession(SessionIndex),
}

/// Receive a response from a runtime request and convert errors.
pub(crate) async fn recv_runtime<V>(
	r: oneshot::Receiver<std::result::Result<V, RuntimeApiError>>,
) -> Result<V> {
	let result = r
		.await
		.map_err(Fatal::RuntimeRequestCanceled)?
		.map_err(NonFatal::RuntimeRequest)?;
	Ok(result)
}
