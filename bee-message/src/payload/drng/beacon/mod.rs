// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod collective_beacon;
pub mod regular_beacon;

#[doc = "BEACON_DISTRIBUTED_PUBLIC_KEY_LENGTH"]
pub const BEACON_DISTRIBUTED_PUBLIC_KEY_LENGTH: usize = 48;
#[doc = "BEACON_PARTIAL_PUBLIC_KEY_LENGTH"]
pub const BEACON_PARTIAL_PUBLIC_KEY_LENGTH: usize = 96;
#[doc = "BEACON_SIGNATURE_LENGTH"]
pub const BEACON_SIGNATURE_LENGTH: usize = 96;
