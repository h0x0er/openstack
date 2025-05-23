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

//! Shows a JSON schema document that represents an image member entity.
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::output::OutputProcessor;
use crate::Cli;
use structable::StructTableOptions;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};


use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use openstack_sdk::api::image::v2::schemas::member::get;
use openstack_sdk::api::RawQueryAsync;

/// Shows a JSON schema document that represents an image member entity.
#[derive(Args, Clone, Debug)]
pub struct MemberArgs {}

pub struct MemberCmd {
    pub args: MemberArgs,
}

/// Member
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Member {}

#[async_trait]
impl OSCCommand for MemberCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Member with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Schema::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Image)
            .await?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data: serde_json::Value = serde_json::from_slice(rsp.body())?;
        op.output_machine(data)?;
        Ok(())
    }
}
