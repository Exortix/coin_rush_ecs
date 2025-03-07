use std::time::{Duration, Instant};

use crate::components::{
    BoundingBox, Coin, Collidable, Enemy, Health, PlayerControlled, Position, PowerUp, Score,
    Velocity,
};
use specs::prelude::*; // Ensure you have the necessary imports

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        ReadStorage<'a, Collidable>,
        ReadStorage<'a, PlayerControlled>,
        WriteStorage<'a, Coin>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Score>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, PowerUp>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            positions,
            bounding_boxes,
            collidables,
            player_controlleds,
            mut coins,
            enemies,
            mut scores,
            mut healths,
            mut powerups,
            mut velocities,
        ): Self::SystemData,
    ) {
        let mut coins_to_remove = Vec::new();
        let mut powerups_to_remove: Vec<Entity> = Vec::new();
        let mut healths_to_reduce: Vec<Entity> = Vec::new();
        let mut powerups_to_apply: Vec<(Entity, &PowerUp)> = Vec::new();

        let players_vec: Vec<(specs::Entity, &Position, &BoundingBox, &Velocity)> = (
            &entities,
            &positions,
            &bounding_boxes,
            &velocities,
            &player_controlleds,
            &collidables,
        )
            .join()
            .map(|(entity, pos, bbox, vel, _, _)| (entity, pos, bbox, vel))
            .collect();

        let enemies_vec: Vec<(specs::Entity, &Position, &BoundingBox)> = (
            &entities,
            &positions,
            &bounding_boxes,
            &enemies,
            &collidables,
        )
            .join()
            .map(|(entity, pos, bbox, _, _)| (entity, pos, bbox))
            .collect();

        let coins_vec: Vec<(specs::Entity, &Position, &BoundingBox)> =
            (&entities, &positions, &bounding_boxes, &coins, &collidables)
                .join()
                .map(|(entity, pos, bbox, _, _)| (entity, pos, bbox))
                .collect();

        let powerups_vec: Vec<(specs::Entity, &Position, &BoundingBox, &PowerUp)> = (
            &entities,
            &positions,
            &bounding_boxes,
            &powerups,
            &collidables,
        )
            .join()
            .map(|(entity, pos, bbox, powerup, _)| (entity, pos, bbox, powerup))
            .collect();

        for (player_entity, player_pos, player_bbox, _) in players_vec.iter() {
            for (coin_entity, coin_pos, coin_bbox) in coins_vec.iter() {
                if player_entity != coin_entity
                    && check_collision(player_pos, player_bbox, coin_pos, coin_bbox)
                {
                    coins_to_remove.push(*coin_entity);
                    for score in (&mut scores).join() {
                        score.value += 10;

                        // print!("\x1B[2J\x1B[1;1H"); // Clear the screen
                        println!("Score: {}", score.value);
                    }
                }
            }

            for (enemy_entity, enemy_pos, enemy_bbox) in enemies_vec.iter() {
                if player_entity != enemy_entity
                    && check_collision(player_pos, player_bbox, enemy_pos, enemy_bbox)
                {
                    healths_to_reduce.push(*player_entity);
                }
            }

            for (powerup_entity, powerup_pos, powerup_bbox, powerup) in powerups_vec.iter() {
                if player_entity != powerup_entity
                    && check_collision(player_pos, player_bbox, powerup_pos, powerup_bbox)
                {
                    powerups_to_remove.push(*powerup_entity);
                    // TODO: IMPLEMENT PowerUpSystem
                    // let expiration_time = Instant::now() + Duration::from_secs(5);
                    // active_powerups.insert(
                    //     *player_entity,
                    //     ActivePowerUp {
                    //         power_type: powerup.power_type.clone(),
                    //         expiration_time,
                    //     },
                    // );

                    // match powerup.power_type.as_str() {
                    //     "speed" => {
                    //         if let Some(velocity) = velocities.get_mut(*player_entity) {
                    //             velocity.dx *= 2.0;
                    //             velocity.dy *= 2.0;
                    //         }
                    //     }
                    //     "health" => {
                    //         if let Some(health) = healths.get_mut(*player_entity) {
                    //             health.value = health.value.saturating_add(10);
                    //         }
                    //     }
                    //     _ => {}
                    // }
                }
            }
        }

        // Remove collected powerups
        for powerup_entity in powerups_to_remove {
            powerups.remove(powerup_entity);
        }

        // Reduce health of players hit by enemies
        for player_entity in healths_to_reduce {
            if let Some(health) = healths.get_mut(player_entity) {
                health.value = health.value.saturating_sub(10);
                println!("Player {} Health: {}", player_entity.id(), health.value);
            }
        }
        // Remove collected coins
        for coin_entity in coins_to_remove {
            coins.remove(coin_entity);
        }
    }
}

fn check_collision(
    pos1: &Position,
    bbox1: &BoundingBox,
    pos2: &Position,
    bbox2: &BoundingBox,
) -> bool {
    let left1 = pos1.x;
    let right1 = pos1.x + bbox1.width;
    let top1 = pos1.y;
    let bottom1 = pos1.y + bbox1.height;

    let left2 = pos2.x;
    let right2 = pos2.x + bbox2.width;
    let top2 = pos2.y;
    let bottom2 = pos2.y + bbox2.height;

    // Check for overlap
    !(right1 <= left2 || left1 >= right2 || bottom1 <= top2 || top1 >= bottom2)
}
