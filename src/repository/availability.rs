use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Datelike, TimeZone};
use diesel::{
    dsl::{exists, not},
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods,
    NullableExpressionMethods, OptionalExtension, QueryDsl,
};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use super::{
    repo_find_all, repo_get, repo_insert, repo_remove, repo_update, repo_upsert, Repository,
    UpdatableRepository,
};
use crate::{
    error::Result,
    model::{Availability, NewAvailability, PartialAvailability, Session, Teacher, Weekday},
    schema::{availability, sessions},
};

/// Manages Availability instances.
#[derive(Clone)]
pub struct AvailabilityRepository {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl AvailabilityRepository {
    /// Creates a new AvailabilityRepository operating with the given
    /// connection pool.
    pub fn new(pool: &Arc<Pool<AsyncPgConnection>>) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Get all Availability instances attached to a certain teacher.
    pub async fn find_by_teacher(&self, teacher: &Teacher) -> Result<Vec<Availability>> {
        Availability::belonging_to(&teacher)
            .get_results(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }

    /// Get the Availability a Session is attached to.
    pub async fn find_by_session(&self, session: &Session) -> Result<Option<Availability>> {
        availability::table
            .inner_join(sessions::table)
            .filter(sessions::id.eq(session.id))
            .first(&mut self.lock_connection().await?)
            .await
            .optional()
            .map(|v: Option<(Availability, Session)>| v.map(|x| x.0))
            .map_err(From::from)
    }

    /// Finds all non-taken availabilities at the given datetime.
    /// It is assumed that availability times stored
    /// in the DB are in UTC-3.
    pub async fn find_nontaken_at_date(
        &self,
        datetime: chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<Vec<Availability>> {
        let utc = chrono::Utc.from_utc_datetime(&datetime.naive_utc());
        let weekday: Weekday = datetime.naive_local().weekday().into();

        // let query = diesel::sql_query(
        //     "SELECT * FROM availability
        //     WHERE weekday = (?) AND time_start > (?)
        //     EXCEPT (
        //         SELECT availability.* FROM availability
        //         INNER JOIN sessions
        //         ON availability.id = sessions.availability_id
        //     )",
        // );
        // query
        //     .bind::<diesel::sql_types::SmallInt, _>(weekday)
        //     .bind::<diesel::sql_types::Time, _>(datetime.time()) // UTC-3 on both operands
        //     .get_results(&mut self.lock_connection().await?)
        //     .await
        //     .map_err(From::from)

        // get all 'Availability' which occur later today (same weekday)
        // except for those linked to sessions
        availability::table
            .select(availability::all_columns)
            .filter(availability::weekday.eq(weekday))
            .filter(not(exists(
                sessions::table.filter(
                    sessions::availability_id
                        .eq(availability::id.nullable())
                        .and(sessions::end_at.ge(utc)),
                ),
            )))
            .get_results(&mut self.lock_connection().await?)
            .await
            .map_err(From::from)
    }
}

#[async_trait]
impl Repository for AvailabilityRepository {
    type Table = availability::table;

    type Entity = Availability;

    type NewEntity = NewAvailability;

    type PrimaryKey = i64;

    const TABLE: Self::Table = availability::table;

    fn get_connection_pool(&self) -> Arc<Pool<AsyncPgConnection>> {
        Arc::clone(&self.pool)
    }

    /// Gets an Availability by its ID.
    async fn get(&self, id: i64) -> Result<Option<Availability>> {
        repo_get!(self, availability::table; id)
    }

    async fn insert(&self, avail: &NewAvailability) -> Result<Availability> {
        repo_insert!(self, availability::table; avail)
    }

    async fn remove(&self, avail: &Availability) -> Result<usize> {
        repo_remove!(self; avail)
    }

    async fn find_all(&self) -> Result<Vec<Availability>> {
        repo_find_all!(self, availability::table, availability::table)
    }
}

#[async_trait]
impl UpdatableRepository for AvailabilityRepository {
    type PartialEntity = PartialAvailability;

    async fn upsert(&self, avail: &NewAvailability) -> Result<Availability> {
        repo_upsert!(self, availability::table; /*conflict_columns=*/availability::id; avail)
    }

    async fn update(
        &self,
        old_avail: &Availability,
        new_avail: PartialAvailability,
    ) -> Result<Availability> {
        repo_update!(self; old_avail => new_avail)
    }
}
