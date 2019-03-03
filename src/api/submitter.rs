//! Module containing all the actix request handlers for the `/api/v1/submitters/` endpoints

use super::PCResponder;
use crate::{
    error::PointercrateError,
    middleware::{auth::Token, cond::HttpResponseBuilderExt},
    model::submitter::{PatchSubmitter, Submitter, SubmitterPagination, SubmitterWithRecords},
    state::PointercrateState,
};
use actix_web::{AsyncResponder, FromRequest, HttpMessage, HttpRequest, HttpResponse, Path};
use log::info;
use tokio::prelude::future::{Future, IntoFuture};

/// `GET /api/v1/users/` handler
pub fn paginate(req: &HttpRequest<PointercrateState>) -> PCResponder {
    info!("GET /api/v1/submitters/");

    let query_string = req.query_string();
    let pagination = serde_urlencoded::from_str(query_string)
        .map_err(|err| PointercrateError::bad_request(&err.to_string()));

    let state = req.state().clone();
    let auth = req.extensions_mut().remove().unwrap();

    pagination
        .into_future()
        .and_then(move |pagination: SubmitterPagination| {
            state.paginate::<Token, Submitter, _>(
                pagination,
                "/api/v1/submitters/".to_owned(),
                auth,
            )
        })
        .map(|(users, links)| HttpResponse::Ok().header("Links", links).json(users))
        .responder()
}

get_handler!(
    "/api/v1/submitters/[id]",
    i32,
    "Submitter ID",
    SubmitterWithRecords
);
patch_handler_with_authorization!(
    "/api/v1/submitters/[id]/",
    i32,
    "Submitter ID",
    PatchSubmitter,
    Submitter
);
