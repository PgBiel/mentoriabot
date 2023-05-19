macro_rules! repo_get {
    ($self:ident, $table:expr; $pk:expr) => {
        $table
            .find($pk)
            .first(&mut $self.lock_connection().await?)
            .await
            .optional()
            .map_err(From::from)
    };
}

macro_rules! repo_insert {
    ($self:ident, $table:expr; $new_entity:expr) => {{
        let entity = diesel::insert_into($table)
            .values($new_entity)
            .get_result(&mut $self.lock_connection().await?)
            .await?;

        $crate::error::Result::Ok(entity)
    }};
}

macro_rules! repo_upsert {
    ($self:ident, $table:expr; $conflict_columns:expr; $new_entity:expr) => {{
        let entity = diesel::insert_into($table)
            .values($new_entity)
            .on_conflict($conflict_columns)
            .do_update()
            .set($new_entity)
            .get_result(&mut $self.lock_connection().await?)
            .await?;

        $crate::error::Result::Ok(entity)
    }};
}

macro_rules! repo_update {
    ($self:ident; $old_entity:expr => $new_entity:expr) => {{
        let entity = diesel::update($old_entity)
            .set($new_entity)
            .get_result(&mut $self.lock_connection().await?)
            .await?;

        $crate::error::Result::Ok(entity)
    }};
}

macro_rules! repo_remove {
    ($self:ident; $entity:expr) => {{
        diesel::delete($entity)
            .execute(&mut $self.lock_connection().await?)
            .await
            .map_err(From::from)
    }};
}

macro_rules! repo_find_by_first {
    ($self:ident, $table:expr; $column:expr; $value:expr) => {
        $table
            .filter($column.eq($value))
            .first(&mut $self.lock_connection().await?)
            .await
            .optional()
            .map_err(From::from)
    };
}

macro_rules! repo_find_by {
    ($self:ident, $table:expr; $filter_expr:expr) => {
        $table
            .filter($filter_expr)
            .get_results(&mut $self.lock_connection().await?)
            .await
            .map_err(From::from)
    };

    ($self:ident, $table:expr; $filter_expr:expr; @order_by: $order_by:expr) => {
        $table
            .filter($filter_expr)
            .order_by($order_by)
            .get_results(&mut $self.lock_connection().await?)
            .await
            .map_err(From::from)
    };
}

macro_rules! repo_find_all {
    ($self:ident, $table:expr, $table_ty:ty) => {{
        $table
            .select(<$table_ty as diesel::Table>::all_columns())
            .get_results(&mut $self.lock_connection().await?)
            .await
            .map_err(From::from)
    }};

    ($self:ident, $table:expr, $table_ty:ty; @order_by: $order_by:expr) => {{
        $table
            .select(<$table_ty as diesel::Table>::all_columns())
            .order_by($order_by)
            .get_results(&mut $self.lock_connection().await?)
            .await
            .map_err(From::from)
    }};
}

pub(crate) use repo_find_all;
pub(crate) use repo_find_by;
pub(crate) use repo_find_by_first;
pub(crate) use repo_get;
pub(crate) use repo_insert;
pub(crate) use repo_remove;
pub(crate) use repo_update;
pub(crate) use repo_upsert;
