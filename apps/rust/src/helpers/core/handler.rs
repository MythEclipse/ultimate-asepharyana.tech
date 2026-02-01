//! Handler helper macros and utilities.

/// Macro to create a simple CRUD handler set.
///
/// # Example
///
/// ```ignore
/// use rustexpress::helpers::crud_handlers;
///
/// crud_handlers!(User, users);
/// // Generates: list_users, get_user, create_user, update_user, delete_user
/// ```
#[macro_export]
macro_rules! crud_handlers {
    ($entity:ident, $name:ident) => {
        paste::paste! {
            pub async fn [<list_ $name>](
                State(db): State<Arc<DatabaseConnection>>,
                Query(params): Query<PaginationParams>,
            ) -> impl IntoResponse {
                use sea_orm::PaginatorTrait;

                let paginator = $entity::Entity::find()
                    .paginate(&*db, params.limit);

                let total = paginator.num_items().await.unwrap_or(0);
                let items = paginator
                    .fetch_page(params.page.saturating_sub(1))
                    .await
                    .unwrap_or_default();

                Json(Paginated::from_params(items, &params, total))
            }

            pub async fn [<get_ $name:snake>](
                State(db): State<Arc<DatabaseConnection>>,
                Path(id): Path<String>,
            ) -> Result<impl IntoResponse, ErrorResponse> {
                let item = $entity::Entity::find_by_id(&id)
                    .one(&*db)
                    .await
                    .map_err(|e| ErrorResponse::internal(e.to_string()))?
                    .ok_or_else(|| ErrorResponse::not_found(concat!(stringify!($entity), " not found")))?;

                Ok(Json(json_ok(item)))
            }

            pub async fn [<delete_ $name:snake>](
                State(db): State<Arc<DatabaseConnection>>,
                Path(id): Path<String>,
            ) -> Result<impl IntoResponse, ErrorResponse> {
                let result = $entity::Entity::delete_by_id(&id)
                    .exec(&*db)
                    .await
                    .map_err(|e| ErrorResponse::internal(e.to_string()))?;

                if result.rows_affected > 0 {
                    Ok(no_content())
                } else {
                    Err(ErrorResponse::not_found(concat!(stringify!($entity), " not found")))
                }
            }
        }
    };
}

/// Wrap a handler result with consistent error handling.
///
/// # Example
///
/// ```ignore
/// use rustexpress::helpers::handler::try_handler;
///
/// async fn my_handler() -> impl IntoResponse {
///     try_handler(async {
///         let data = fetch_data().await?;
///         Ok(data)
///     }).await
/// }
/// ```
pub async fn try_handler<T, E, F, Fut>(f: F) -> Result<crate::helpers::JsonResponse<T>, crate::helpers::ErrorResponse>
where
    T: serde::Serialize,
    E: std::fmt::Display,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    match f().await {
        Ok(data) => Ok(crate::helpers::JsonResponse::ok(data)),
        Err(e) => Err(crate::helpers::ErrorResponse::internal(e.to_string())),
    }
}

/// Extract user ID from JWT claims (helper).
pub fn get_user_id_from_claims(claims: &serde_json::Value) -> Option<String> {
    claims.get("sub").and_then(|v| v.as_str()).map(String::from)
}
