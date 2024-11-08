use anyhow::anyhow;

use askama_axum::IntoResponse;
use axum::{debug_handler, extract::{Path, State}, Extension, Form};
use axum_messages::Messages;
use garde::{Report, Validate};
use tracing::instrument;

use crate::{application::{crud::schema::UpdateAssetItemSchema, errors::ApplicationError, state::AppState, templates::{pages::{asset_item_edit::AssetItemEditTemplate, asset_item_view::AssetItemViewTemplate}, partials::form_alert::FormAlertTemplate}}, domain::identityaccess::model::users::SessionUser};


#[instrument(skip_all)]
pub async fn get_asset_item_edit(
    messages: Messages,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
) -> Result<impl IntoResponse, ApplicationError> {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    let asset_item = state.crud_service.get_asset_item(id)
        .await
        .map_err(|e| ApplicationError::internal_server_error(anyhow!(e)))?
        .ok_or(ApplicationError::internal_server_error(anyhow!("asset type not found")))?;

    Ok(([("Cache-Control", "no-store")], AssetItemEditTemplate::new(session_user, message, asset_item)))
}

#[instrument(skip_all)]
pub async fn post_asset_item_edit(
    messages: Messages,
    State(state): State<AppState>,
    Extension(session_user): Extension<SessionUser>,
    Path(id): Path<i32>,
    Form(update_asset_item ): Form<UpdateAssetItemSchema>,
) -> Result<impl IntoResponse, ApplicationError> {

    if let Err(report) = update_asset_item.validate() {
        return Err(ApplicationError::bad_request(anyhow!("invalid form"), FormAlertTemplate::global_new(report).to_string()));
    }

    //TODO: perform edit
    let asset_item = state.crud_service.get_asset_item(id)
        .await
        .map_err(|e| ApplicationError::internal_server_error(anyhow!(e)))?
        .ok_or(ApplicationError::internal_server_error(anyhow!("asset type not found")))?;


    messages.success("asset item saved");

    Ok(([("HX-Redirect", format!("/asset_items/{}", id))], "success"))}

#[instrument(skip_all)]
pub async fn delete_asset_item(
    messages: Messages,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApplicationError> {

    

    messages.success("asset item deleted");
    Ok(([("HX-Redirect", "/asset_items")], "success"))
}

/*
    // convert the `AsyncRead` into a `Stream` e.g tokio::file
    let stream = ReaderStream::new(file);    

    //Content-Disposition: attachment; filename=fname.ext // will force download
    let content_disposition = format!("inline;filename={}", filename);
    // return streaming file or bytes as response
     Ok((
        [
            ("Content-Type", content_type.to_string()),
            ("Content-Disposition", content_disposition)
        ],
        Body::from_stream(stream)
    ))  
 */