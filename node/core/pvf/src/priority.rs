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

/// A priority assigned to execution of a PVF.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
	/// Jobs in this priority will be executed in the background, meaning that they will be only
	/// given spare CPU time.
	///
	/// This is mainly for cache warmings.
	Background,
	/// Normal priority for things that do not require immediate response, but still need to be
	/// done pretty quick.
	///
	/// Approvals and disputes fall into this category.
	Normal,
	/// This priority is used for requests that are required to be processed as soon as possible.
	///
	/// For example, backing is on critical path and require execution as soon as possible.
	Critical,
}

impl Priority {
	/// Returns `true` if `self` is `Crticial`
	pub fn is_critical(self) -> bool {
		self == Priority::Critical
	}

	/// Returns `true` if `self` is `Background`
	pub fn is_background(self) -> bool {
		self == Priority::Background
	}
}
