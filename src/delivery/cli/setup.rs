//
// Copyright:: Copyright (c) 2016 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use clap::{App, ArgMatches, SubCommand};
use cli::arguments::{
    a2_mode_arg, config_path_arg, pipeline_arg, project_arg, u_e_s_o_args, value_of,
};
use cli::Options;
use config::Config;
use types::DeliveryResult;

pub const SUBCOMMAND_NAME: &'static str = "setup";

#[derive(Debug)]
pub struct SetupClapOptions<'n> {
    pub user: &'n str,
    pub server: &'n str,
    pub ent: &'n str,
    pub org: &'n str,
    pub path: &'n str,
    pub pipeline: &'n str,
    pub project: &'n str,
    pub a2_mode: Option<bool>,
}

impl<'n> Default for SetupClapOptions<'n> {
    fn default() -> Self {
        SetupClapOptions {
            user: "",
            server: "",
            ent: "",
            org: "",
            path: "",
            pipeline: "master",
            project: "",
            a2_mode: None,
        }
    }
}

impl<'n> SetupClapOptions<'n> {
    pub fn new(matches: &'n ArgMatches<'n>) -> Self {
        SetupClapOptions {
            user: value_of(&matches, "user"),
            server: value_of(&matches, "server"),
            ent: value_of(&matches, "ent"),
            org: value_of(&matches, "org"),
            path: value_of(&matches, "config-path"),
            pipeline: value_of(&matches, "pipeline"),
            project: value_of(&matches, "project"),
            a2_mode: if matches.is_present("a2-mode") {
                Some(true)
            } else {
                None
            },
        }
    }
}

impl<'n> Options for SetupClapOptions<'n> {
    fn merge_options_and_config(&self, config: Config) -> DeliveryResult<Config> {
        let mut new_config = config
            .set_server(&self.server)
            .set_user(&self.user)
            .set_enterprise(&self.ent)
            .set_organization(&self.org)
            .set_pipeline(&self.pipeline)
            .set_project(&self.project)
            .set_a2_mode_if_def(self.a2_mode);
        // A2 mode requires SAML right now
        if new_config.a2_mode.unwrap_or(false) {
            new_config.saml = Some(true)
        }
        Ok(new_config)
    }
}

pub fn clap_subcommand<'c>() -> App<'c, 'c> {
    SubCommand::with_name(SUBCOMMAND_NAME)
        .about("Write a config file capturing specified options")
        .args(&vec![config_path_arg(), project_arg()])
        .args(&pipeline_arg())
        .args(&u_e_s_o_args())
        .args(&vec![a2_mode_arg()])
}
