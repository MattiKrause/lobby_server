pub mod sql_token_repository;
#[cfg(feature = "postgres_user_repository")]
pub mod postgres_user_repository;

#[macro_export]
macro_rules! sqlx_generic_fn {
    ($name: ident($e: ident$(, $param_name: ident: $param_type: ty)*) $(-> $ret_type: ty)?: with $b: lifetime $($used_types: ty),* = $body: expr) => {
        async fn $name<'a, DB: sqlx::Database, E: sqlx::Executor<'a, Database = DB>>($($param_name: $param_type,)* $e: E) $(-> $ret_type)? where $(for <$b> $used_types: sqlx::Type<DB> + sqlx::Encode<$b, DB> + sqlx::Decode<$b, DB>,)* usize: sqlx::ColumnIndex<DB::Row>, for <'b> &'b str: sqlx::ColumnIndex<DB::Row>, for<'b> <DB as sqlx::database::HasArguments<'b>>::Arguments: sqlx::IntoArguments<'b, DB> { $body }
    };
}