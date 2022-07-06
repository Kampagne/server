#![allow(non_snake_case)]
pub mod Player;
use Player::PlayerQuery;

use async_graphql::MergedObject;


#[derive(MergedObject, Default)]
pub struct ModelsQuery(

    // Add models to merge here
    PlayerQuery,

);

