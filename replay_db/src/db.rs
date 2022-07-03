use sqlx::{QueryBuilder, Sqlite};

use crate::schema::{PersonalVehicle, PlayerInfo, Replay, ReplayCommon, Vehicle};

pub struct ReplayDatabase {
    conn: sqlx::Pool<Sqlite>,
}

impl ReplayDatabase {
    pub fn new(conn: sqlx::Pool<Sqlite>) -> ReplayDatabase {
        ReplayDatabase { conn }
    }

    pub async fn insert_replay(&self, replay: Replay) {
        let mut tx = self.conn.begin().await.unwrap();

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(ReplayCommon::insert_query_start());
        query_builder.push_values([replay.common], ReplayCommon::push_bindings);
        query_builder.build().execute(&mut tx).await.unwrap();

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(PlayerInfo::insert_query_start());
        query_builder.push_values(replay.player_infos.into_values(), PlayerInfo::push_bindings);
        query_builder.build().execute(&mut tx).await.unwrap();

        tx.commit().await.unwrap();

        let mut tx = self.conn.begin().await.unwrap();

        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new(PersonalVehicle::insert_query_start());
        query_builder.push_values(
            replay.personal_vehicle.into_values(),
            PersonalVehicle::push_bindings,
        );
        query_builder.build().execute(&mut tx).await.unwrap();

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(Vehicle::insert_query_start());
        query_builder.push_values(replay.vehicles.into_values(), Vehicle::push_bindings);
        query_builder.build().execute(&mut tx).await.unwrap();

        tx.commit().await.unwrap();
    }
}
