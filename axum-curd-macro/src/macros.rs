#[macro_export]
macro_rules! impl_get {
    ($model:ty) => {
        impl curd::Get<$model> for $model {}
    };
}

#[macro_export]
macro_rules! impl_get_with_id {
  ($model:ty) => {
      impl curd::GetWithId<$model> for $model {}
  };
}

#[macro_export]
macro_rules! impl_put {
    ($model:ty) => {
        impl curd::Put<$model> for $model {}
    };
}

#[macro_export]
macro_rules! impl_patch {
    ($model:ty) => {
        impl curd::Patch<$model> for $model {}
    };
}

#[macro_export]
macro_rules! impl_delete {
    ($model:ty) => {
        impl curd::Delete<$model> for $model {}
    };
}

#[macro_export]
macro_rules! impl_all {
    ($model:ty) => {
        axum_curd_macro::impl_curd!($model, get, get_with_id, put, patch, delete);
    };
}

#[macro_export]
macro_rules! impl_curd {
    ($model:ty, $( $extra:expr ),* ) => {
        $(
            paste::paste! {
                axum_curd_macro::[<impl_ $extra>]!($model);
            }
        )*
    };
}

#[macro_export]
macro_rules! route_get {
    ($app:expr, $path:expr, $model:ty) => {
        let $app = $app.route($path, axum::routing::get($model::route_get));
    };
}

#[macro_export]
macro_rules! route_get_with_id {
    ($app:expr, $path:expr, $model:ty) => {
       let $app = $app.route(concat!($path, "/:id"), axum::routing::get($model::route_get_with_id));
    };
}

#[macro_export]
macro_rules! route_put {
    ($app:expr, $path:expr, $model:ty) => {
        let $app = $app.route($path, axum::routing::put($model::route_put));
    };
}

#[macro_export]
macro_rules! route_patch {
    ($app:expr, $path:expr, $model:ty) => {
        let $app = $app.route($path, axum::routing::patch($model::route_patch));
    };
}

#[macro_export]
macro_rules! route_delete {
    ($app:expr, $path:expr, $model:ty) => {
        let $app = $app.route($path, axum::routing::delete($model::route_delete));
    };
}

#[macro_export]
macro_rules! route_all {
    ($app:expr, $path:expr, $model:ty) => {
        let $app = $app.route($path, axum::routing::get($model::route_get));
        let $app = $app.route(concat!($path, "/:id"), axum::routing::get($model::route_get_with_id));
        let $app = $app.route($path, axum::routing::put($model::route_put));
        let $app = $app.route($path, axum::routing::patch($model::route_patch));
        let $app = $app.route($path, axum::routing::delete($model::route_delete));
    }
}

#[macro_export]
macro_rules! route_curd {
    ($app:expr, $path:expr, $model:ty, $( $extra:expr ),* ) => {
        $(
            paste::paste! {
                axum_curd_macro::[<route_ $extra>]!($app, $path, $model);
            }
        )*
    };
}