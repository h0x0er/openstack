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

//! Block storage Volume Attachment commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_327;
pub mod create_354;
pub mod delete;
pub mod list;
pub mod os_complete;
pub mod set_327;
pub mod show;

/// Attachments (attachments)
///
/// Lists all, lists all with details, shows details for, creates, and deletes attachment.
///
/// Note
///
/// Everything except for Complete attachment is new as of the 3.27 microversion. Complete attachment is new as of the 3.44 microversion.
///
/// When you create, list, update, or delete attachment, the possible status values are:
///
///   - attached: A volume is attached for the attachment.
///
///   - attaching: A volume is attaching for the attachment.
///
///   - detached: A volume is detached for the attachment.
///
///   - reserved: A volume is reserved for the attachment.
///
///   - error_attaching: A volume is error attaching for the attachment.
///
///   - error_detaching: A volume is error detaching for the attachment.
///
///   - deleted: The attachment is deleted.
#[derive(Parser)]
pub struct AttachmentCommand {
    /// subcommand
    #[command(subcommand)]
    command: AttachmentCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AttachmentCommands {
    Complete(Box<os_complete::AttachmentCommand>),
    #[command(visible_alias = "create")]
    Create354(Box<create_354::AttachmentCommand>),
    Create327(Box<create_327::AttachmentCommand>),
    Delete(Box<delete::AttachmentCommand>),
    List(Box<list::AttachmentsCommand>),
    #[command(visible_alias = "set")]
    Set327(Box<set_327::AttachmentCommand>),
    Show(Box<show::AttachmentCommand>),
}

impl AttachmentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AttachmentCommands::Complete(cmd) => cmd.take_action(parsed_args, session).await,
            AttachmentCommands::Create354(cmd) => cmd.take_action(parsed_args, session).await,
            AttachmentCommands::Create327(cmd) => cmd.take_action(parsed_args, session).await,
            AttachmentCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AttachmentCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AttachmentCommands::Set327(cmd) => cmd.take_action(parsed_args, session).await,
            AttachmentCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
