use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(description = "API for the WhatsUp backend"),
    paths(
        crate::controller::users::logic::get_account,
        crate::controller::users::logic::join_group,
        crate::controller::users::logic::toggle_ready,
        crate::controller::groups::logic::create_group,
        crate::controller::groups::logic::get_groups,
        crate::controller::health::health,
    ),
    components(schemas(
        crate::controller::users::responses::JoinGroupRequest,
        crate::controller::users::responses::UserResponse,
        crate::controller::groups::responses::GroupResponse,
    ),)
)]
struct ApiDoc;

pub(crate) async fn openapi() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap()
}
