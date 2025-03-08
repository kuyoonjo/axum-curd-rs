use anyhow::Result;
use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
};
use rest_model::{
    method, pagination, DeleteParams, DeleteResult, Doc, PatchParams, RestModel, UpdateResult,
    UpsertResult,
};
use rest_model_postgres::Db;
use tokio::sync::OnceCell;

pub static DB: OnceCell<Db> = OnceCell::const_new();

pub trait Get<T>
where
    T: RestModel + method::Get<T, Db>,
{
    #[allow(async_fn_in_trait)]
    async fn route_get(
        Query(params): Query<pagination::PaginationParams>,
    ) -> Result<(StatusCode, HeaderMap, Json<Vec<Doc<T>>>)> {
        let res = T::get(DB.get().unwrap(), &params).await?;
        let mut headers = HeaderMap::new();

        headers.insert(
            pagination::HEADER_EXPOSE,
            HeaderValue::from_str("*").unwrap(),
        );
        headers.insert(
            pagination::HEADER_TOTAL_COUNT,
            res.pagination.total_count.into(),
        );
        headers.insert(
            pagination::HEADER_TOTAL_PAGES,
            res.pagination.total_pages.into(),
        );
        headers.insert(
            pagination::HEADER_CURRENT_PAGE,
            res.pagination.current_page.into(),
        );
        headers.insert(
            pagination::HEADER_ITEMS_PER_PAGE,
            res.pagination.items_per_page.into(),
        );
        Ok((StatusCode::OK, headers, Json(res.items)))
    }
}

pub trait GetWithId<T>
where
    T: RestModel + method::GetWithId<T, Db>,
{
    #[allow(async_fn_in_trait)]
    async fn route_get_with_id(Path(id): Path<String>) -> Result<(StatusCode, Json<Doc<T>>)> {
        let res = T::get_with_id(DB.get().unwrap(), &id).await?;
        Ok((StatusCode::OK, Json(res)))
    }
}

pub trait Put<T>
where
    T: RestModel + method::Put<T, Db> + Put<T>,
{
    #[allow(async_fn_in_trait)]
    async fn route_put(Json(items): Json<Vec<Doc<T>>>) -> Result<(StatusCode, Json<UpsertResult>)> {
        let db = DB.get().unwrap();
        T::pre_put(db, &items).await;
        let res = T::put(db, &items).await?;
        T::post_put(db, &items).await;
        Ok((StatusCode::OK, Json(res)))
    }
    #[allow(async_fn_in_trait)]
    async fn pre_put(_db: &Db, _docs: &[Doc<T>]) {}
    #[allow(async_fn_in_trait)]
    async fn post_put(_db: &Db, _docs: &[Doc<T>]) {}
}

pub trait Patch<T>
where
    T: RestModel + method::Patch<T, Db> + Patch<T>,
{
    #[allow(async_fn_in_trait)]
    async fn route_patch(
        Json(params): Json<PatchParams>,
    ) -> Result<(StatusCode, Json<UpdateResult>)> {
        let db = DB.get().unwrap();
        T::pre_patch(db, &params).await;
        let res = T::patch(db, &params).await?;
        T::post_patch(db, &params).await;
        Ok((StatusCode::OK, Json(res)))
    }

    #[allow(async_fn_in_trait)]
    async fn pre_patch(_db: &Db, _params: &rest_model::PatchParams) {}
    #[allow(async_fn_in_trait)]
    async fn post_patch(_db: &Db, _params: &rest_model::PatchParams) {}
}

pub trait Delete<T>
where
    T: RestModel + method::Delete<T, Db> + Delete<T>,
{
    #[allow(async_fn_in_trait)]
    async fn route_delete(
        Json(params): Json<DeleteParams>,
    ) -> Result<(StatusCode, Json<DeleteResult>)> {
        let db = DB.get().unwrap();
        T::pre_delete(db, &params).await;
        let res = T::delete(db, &params).await?;
        T::post_delete(db, &params).await;
        Ok((StatusCode::OK, Json(res)))
    }
    #[allow(async_fn_in_trait)]
    async fn pre_delete(_db: &Db, _params: &rest_model::DeleteParams) {}

    #[allow(async_fn_in_trait)]
    async fn post_delete(_db: &Db, _params: &rest_model::DeleteParams) {}
}
