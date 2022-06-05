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

use microservices::rpc;

#[derive(Debug, Display, From, Error)]
#[display(doc_comments)]
#[non_exhaustive]
pub enum Error {
    #[from]
    Transport(internet2::transport::Error),

    #[from]
    Presentation(internet2::presentation::Error),

    /// RPC error: {0}
    #[from]
    Rpc(rpc::Error),

    /// other error type with string explanation
    #[display(inner)]
    #[from(internet2::addr::NoOnionSupportError)]
    Other(String),
}

impl microservices::error::Error for Error {}
