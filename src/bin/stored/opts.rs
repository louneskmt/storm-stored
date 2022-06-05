// Storage daemon (stored): microservice frontend for different storage backends
// used in LNP/BP nodes.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::path::PathBuf;

use clap::{Parser, ValueHint};
use internet2::ZmqSocketAddr;
use storedrpc::STORED_RPC_ENDPOINT;

#[cfg(any(target_os = "linux"))]
pub const STORED_DATA_DIR: &'static str = "~/.xengine";
#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub const STORED_DATA_DIR: &'static str = "~/.xengine";
#[cfg(target_os = "macos")]
pub const STORED_DATA_DIR: &'static str = "~/Library/Application Support/Xengine";
#[cfg(target_os = "windows")]
pub const STORED_DATA_DIR: &'static str = "~\\AppData\\Local\\MyCitadel";
#[cfg(target_os = "ios")]
pub const STORED_DATA_DIR: &'static str = "~/Documents";
#[cfg(target_os = "android")]
pub const STORED_DATA_DIR: &'static str = ".";

pub const STORED_CONFIG: &'static str = "{data_dir}/stored.toml";
pub const STORED_STORAGE_FILE: &'static str = "data";

/// Command-line arguments
#[derive(Parser)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[clap(author, version, name = "stored", about = "stored: storage microservice daemon")]
pub struct Opts {
    /// Set verbosity level
    ///
    /// Can be used multiple times to increase verbosity
    #[clap(short, long, global = true, parse(from_occurrences))]
    pub verbose: u8,

    /// Data directory path
    ///
    /// Path to the directory that contains stored data, and where ZMQ RPC
    /// socket files are located
    #[clap(
        short,
        long,
        global = true,
        default_value = STORED_DATA_DIR,
        env = "STORED_DATA_DIR",
        value_hint = ValueHint::DirPath
    )]
    pub data_dir: PathBuf,

    /// ZMQ socket name/address for stpred node RPC interface
    ///
    /// Internal interface for control PRC protocol communications.
    #[clap(
        short = 'x',
        long,
        env = "STORED_RPC_ENDPOINT",
        value_hint = ValueHint::FilePath,
        default_value = STORED_RPC_ENDPOINT
    )]
    pub rpc_endpoint: ZmqSocketAddr,
}