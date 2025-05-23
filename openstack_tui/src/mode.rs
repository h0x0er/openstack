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

use serde::{Deserialize, Serialize};

/// TUI Modes (screens)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
    Describe,

    // Cinder
    /// Cinder backups
    BlockStorageBackups,
    /// Cinder snapshots
    BlockStorageSnapshots,
    /// Cinder volumes
    BlockStorageVolumes,

    // Nova
    ComputeAggregates,
    ComputeFlavors,
    ComputeHypervisors,
    ComputeServers,
    /// Server os-instance-actions
    ComputeServerInstanceActions,
    /// Server os-instance-action events
    ComputeServerInstanceActionEvents,

    // DNS
    /// DNS Zones
    DnsZones,
    /// DNS Zone recordsets
    DnsRecordsets,

    // Keystone
    IdentityApplicationCredentials,
    IdentityGroups,
    IdentityGroupUsers,
    IdentityProjects,
    IdentityUsers,

    // Glance
    ImageImages,

    // Octavia
    /// LB Loadbalancers
    LoadBalancers,
    /// LB Listeners
    LoadBalancerListeners,
    /// LB Pools
    LoadBalancerPools,
    /// LB pool members
    LoadBalancerPoolMembers,
    /// LB HealthMonitors
    LoadBalancerHealthMonitors,

    // Neutron
    NetworkNetworks,
    NetworkRouters,
    NetworkSecurityGroups,
    NetworkSecurityGroupRules,
    NetworkSubnets,
}
