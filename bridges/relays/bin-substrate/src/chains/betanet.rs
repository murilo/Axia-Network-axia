// Copyright 2019-2021 AXIA Technologies (UK) Ltd.
// This file is part of AXIA Bridges Common.

// AXIA Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

use codec::Decode;
use frame_support::weights::{DispatchClass, DispatchInfo, Pays, Weight};
use relay_betanet_client::BetaNet;
use sp_version::RuntimeVersion;

use crate::cli::{
	bridge,
	encode_call::{Call, CliEncodeCall},
	encode_message, CliChain,
};

/// Weight of the `system::remark` call at BetaNet.
///
/// This weight is larger (x2) than actual weight at current Rooco runtime to avoid unsuccessful
/// calls in the future. But since it is used only in tests (and on test chains), this is ok.
pub(crate) const SYSTEM_REMARK_CALL_WEIGHT: Weight = 2 * 1_345_000;

impl CliEncodeCall for BetaNet {
	fn max_extrinsic_size() -> u32 {
		bp_betanet::max_extrinsic_size()
	}

	fn encode_call(call: &Call) -> anyhow::Result<Self::Call> {
		Ok(match call {
			Call::Remark { remark_payload, .. } => {
				relay_betanet_client::runtime::Call::System(relay_betanet_client::runtime::SystemCall::remark(
					remark_payload.as_ref().map(|x| x.0.clone()).unwrap_or_default(),
				))
			}
			Call::BridgeSendMessage {
				lane,
				payload,
				fee,
				bridge_instance_index,
			} => match *bridge_instance_index {
				bridge::BETANET_TO_WOCOCO_INDEX => {
					let payload = Decode::decode(&mut &*payload.0)?;
					relay_betanet_client::runtime::Call::BridgeMessagesWococo(
						relay_betanet_client::runtime::BridgeMessagesWococoCall::send_message(lane.0, payload, fee.0),
					)
				}
				_ => anyhow::bail!(
					"Unsupported target bridge pallet with instance index: {}",
					bridge_instance_index
				),
			},
			_ => anyhow::bail!("The call is not supported"),
		})
	}

	fn get_dispatch_info(call: &relay_betanet_client::runtime::Call) -> anyhow::Result<DispatchInfo> {
		match *call {
			relay_betanet_client::runtime::Call::System(relay_betanet_client::runtime::SystemCall::remark(_)) => {
				Ok(DispatchInfo {
					weight: SYSTEM_REMARK_CALL_WEIGHT,
					class: DispatchClass::Normal,
					pays_fee: Pays::Yes,
				})
			}
			_ => anyhow::bail!("Unsupported BetaNet call: {:?}", call),
		}
	}
}

impl CliChain for BetaNet {
	const RUNTIME_VERSION: RuntimeVersion = bp_betanet::VERSION;

	type KeyPair = sp_core::sr25519::Pair;
	type MessagePayload = ();

	fn ss58_format() -> u16 {
		42
	}

	fn max_extrinsic_weight() -> Weight {
		bp_wococo::max_extrinsic_weight()
	}

	fn encode_message(_message: encode_message::MessagePayload) -> Result<Self::MessagePayload, String> {
		Err("Sending messages from BetaNet is not yet supported.".into())
	}
}
