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

//! Block storage Volume Group commands

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_313;
pub mod create_from_src_314;
pub mod delete_313;
pub mod disable_replication_338;
pub mod enable_replication_338;
pub mod failover_replication_338;
pub mod list;
pub mod list_replication_targets_338;
pub mod reset_status_320;
pub mod set_313;
pub mod show;

/// Generic volume groups (groups)
///
/// Generic volume groups enable you to create a group of volumes and manage them together.
///
/// How is generic volume groups different from consistency groups? Currently consistency groups in
/// cinder only support consistent group snapshot. It cannot be extended easily to serve other
/// purposes. A project may want to put volumes used in the same application together in a group so
/// that it is easier to manage them together, and this group of volumes may or may not support
/// consistent group snapshot. Generic volume group is introduced to solve this problem. By
/// decoupling the tight relationship between the group construct and the consistency concept,
/// generic volume groups can be extended to support other features in the future.
#[derive(Parser)]
pub struct GroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: GroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum GroupCommands {
    #[command(visible_alias = "create")]
    Create313(Box<create_313::GroupCommand>),
    #[command(visible_alias = "create_from_src")]
    CreateFromSrc314(Box<create_from_src_314::GroupCommand>),
    #[command(visible_alias = "delete")]
    Delete313(Box<delete_313::GroupCommand>),
    #[command(visible_alias = "disable-replication")]
    DisableReplication338(Box<disable_replication_338::GroupCommand>),
    #[command(visible_alias = "enable-replication")]
    EnableReplication338(Box<enable_replication_338::GroupCommand>),
    #[command(visible_alias = "failover-replication")]
    FailoverReplication338(Box<failover_replication_338::GroupCommand>),
    List(Box<list::GroupsCommand>),
    #[command(visible_alias = "list-replication-targets")]
    ListReplicationTargets338(Box<list_replication_targets_338::GroupCommand>),
    #[command(visible_alias = "reset-status")]
    ResetStatus320(Box<reset_status_320::GroupCommand>),
    Set313(Box<set_313::GroupCommand>),
    Show(Box<show::GroupCommand>),
}

impl GroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupCommands::Create313(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::CreateFromSrc314(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Delete313(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::DisableReplication338(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            GroupCommands::EnableReplication338(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::FailoverReplication338(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            GroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::ListReplicationTargets338(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            GroupCommands::ResetStatus320(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Set313(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
