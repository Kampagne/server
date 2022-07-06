use sea_orm::entity::prelude::*;
use async_graphql::{ Object, SimpleObject, InputObjectType, InputObject };

// Sea-Orm Model
#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject, InputObject
)]
#[graphql(name = "Player")]
#[sea_orm(table_name = "player")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String
}
type Player = Model;

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// GraphQL Query
#[derive(Default)]
pub struct PlayerQuery;

#[Object]
impl PlayerQuery {
    async fn create_player(&self, name: String) -> String {
        format!( "Creating player named '{}'", name )
    }

    /// Returns a list of all the players in the database
    async fn get_players(&self, ctx: &async_graphql::Context<'_>,) -> Vec<Player> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Entity::find().all( conn ).await.unwrap()
    }

    async fn add_player(&self, ctx: &async_graphql::Context<'_>, player_obj: Player) -> bool {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        
        true
    }
}