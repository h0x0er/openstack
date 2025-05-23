// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
//! Response type for the GET `types/{id}` operation

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use structable::{StructTable, StructTableOptions};

/// Type response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct TypeResponse {
    /// The volume type description.
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    /// A key and value pair that contains additional specifications that are
    /// associated with the volume type. Examples include capabilities,
    /// capacity, compression, and so on, depending on the storage driver in
    /// use.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub extra_specs: Option<BTreeMap<String, Option<String>>>,

    /// The UUID of the volume type.
    #[structable()]
    pub id: String,

    /// Whether the volume type is publicly visible.
    #[serde(default)]
    #[structable(optional)]
    pub is_public: Option<bool>,

    /// The name of the volume type.
    #[structable()]
    pub name: String,

    /// Whether the volume type is publicly visible.
    #[serde(default, rename = "os-volume-type-access:is_public")]
    #[structable(optional, title = "os-volume-type-access:is_public")]
    pub os_volume_type_access_is_public: Option<bool>,

    /// The QoS specifications ID.
    #[serde(default)]
    #[structable(optional)]
    pub qos_specs_id: Option<String>,
}
