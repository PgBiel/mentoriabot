macro_rules! repo_insert {
    ($conn:expr, $table:expr; $new_entity:expr) => {
        {
            let entity = diesel::insert_into($table)
                .values($new_entity)
                .get_result($conn)
                .await?;

            $crate::error::Result::Ok(entity)
        }
    }
}

macro_rules! repo_update {
    ($conn:expr; $old_entity:expr => $new_entity:expr) => {
        {
            let entity = diesel::update($old_entity)
                .set($new_entity)
                .get_result($conn)
                .await?;

            $crate::error::Result::Ok(entity)
        }
    }
}

macro_rules! repo_remove {
    ($conn:expr; $entity:expr) => {
        {
            diesel::delete($entity)
                .execute($conn)
                .await?;

            $crate::error::Result::Ok(())
        }
    }
}

macro_rules! repo_get_by_id {
    ($conn:expr, $table:expr, $id_column:expr; $id:expr) => {
        $table.filter($id_column.eq($id))
            .first($conn)
            .await
            .optional()
            .map_err(From::from)
    }
}

pub(crate) use repo_insert;
pub(crate) use repo_update;
pub(crate) use repo_remove;
pub(crate) use repo_get_by_id;
