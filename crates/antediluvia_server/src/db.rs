//! Database layer (stubs). Uses sqlx for Postgres.

use anyhow::Result;
use sqlx::{Pool, Postgres};
use std::env;
use tracing::info;
use serde::{Serialize, Deserialize};

/// Database pool wrapper.
pub struct DbPool {
    pub pool: Pool<Postgres>,
}

impl DbPool {
    /// Connect to Postgres using DATABASE_URL env.
    pub async fn connect() -> Result<Self> {
        let url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                info!("DATABASE_URL not set. Attempting localhost connection...");
                "postgres://postgres@localhost:5432/antediluvia".to_string()
            });
        let pool = Pool::<Postgres>::connect(&url).await?;
        info!("Connected to Postgres");
        Ok(Self { pool })
    }

    /// Initialize schema if missing.
    pub async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                id BIGINT PRIMARY KEY,
                name TEXT NOT NULL,
                lineage TEXT NOT NULL,
                corruption REAL NOT NULL,
                position_x REAL NOT NULL,
                position_y REAL NOT NULL,
                position_z REAL NOT NULL,
                inventory_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS world (
                id BIGINT PRIMARY KEY,
                corruption REAL NOT NULL,
                flood_phase TEXT NOT NULL,
                server_time_days INT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Placeholder: load player state.
    pub async fn load_player(&self, player_id: u64) -> Result<Option<PlayerRecord>> {
        let rec = sqlx::query_as::<_, PlayerRecord>(
            r#"
            SELECT id, name, lineage, corruption, position_x, position_y, position_z, inventory_json
            FROM players WHERE id = $1
            "#,
        )
        .bind(player_id as i64)
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    /// Placeholder: save player state.
    pub async fn save_player(&self, player: &PlayerRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO players (id, name, lineage, corruption, position_x, position_y, position_z, inventory_json)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
            ON CONFLICT (id) DO UPDATE
            SET name = EXCLUDED.name,
                lineage = EXCLUDED.lineage,
                corruption = EXCLUDED.corruption,
                position_x = EXCLUDED.position_x,
                position_y = EXCLUDED.position_y,
                position_z = EXCLUDED.position_z,
                inventory_json = EXCLUDED.inventory_json;
            "#,
        )
        .bind(player.id)
        .bind(&player.name)
        .bind(&player.lineage)
        .bind(player.corruption)
        .bind(player.position_x)
        .bind(player.position_y)
        .bind(player.position_z)
        .bind(&player.inventory_json)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn load_world(&self) -> Result<Option<WorldRecord>> {
        let rec = sqlx::query_as::<_, WorldRecord>(
            r#"
            SELECT id, corruption, flood_phase, server_time_days
            FROM world WHERE id = 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn save_world(&self, world: &WorldRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO world (id, corruption, flood_phase, server_time_days)
            VALUES ($1,$2,$3,$4)
            ON CONFLICT (id) DO UPDATE
            SET corruption = EXCLUDED.corruption,
                flood_phase = EXCLUDED.flood_phase,
                server_time_days = EXCLUDED.server_time_days;
            "#,
        )
        .bind(world.id)
        .bind(world.corruption)
        .bind(&world.flood_phase)
        .bind(world.server_time_days)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

/// Player record (persistence model).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlayerRecord {
    pub id: i64,
    pub name: String,
    pub lineage: String,
    pub corruption: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub inventory_json: String,
}

/// World record (persistence model).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorldRecord {
    pub id: i64,
    pub corruption: f32,
    pub flood_phase: String,
    pub server_time_days: i32,
}
