use robotics_lib::interface::*;
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;
use robotics_lib::interface::{Direction, robot_view};

/// Sadly this tool didn't become much, since there was ultimately no access to the world map to put any pathfinding into place,
/// and the crafting of sticks from stones (which gave the whole name and point to the tool) was removed.

/// We are left with two functions to chop and sell wood in all directions :/


/// Returns a Vec with the direction of the breaking attempt combined with its result from the `destroy()` interface
pub fn chop_wood(robot: &mut impl Runnable, world: &mut World) -> Vec<(Direction, Result<usize, LibError>)> {
    let r_view = robot_view(robot, world);
    let mut directions_to_remove = Vec::new();

    match &r_view[0][1] {
        Some(tile) => match tile.content {
            Content::Tree(_) => { directions_to_remove.push(Direction::Up) }
            _ => {}
        },
        _ => {}
    }
    match &r_view[2][1] {
        Some(tile) => match tile.content {
            Content::Tree(_) => { directions_to_remove.push(Direction::Down) }
            _ => {}
        },
        _ => {}
    }
    match &r_view[1][0] {
        Some(tile) => match tile.content {
            Content::Tree(_) => { directions_to_remove.push(Direction::Left) }
            _ => {}
        },
        _ => {}
    }
    match &r_view[1][2] {
        Some(tile) => match tile.content {
            Content::Tree(_) => { directions_to_remove.push(Direction::Right) }
            _ => {}
        },
        _ => {}
    }

    let mut action_result: Vec<(Direction, Result<usize, LibError>)> = Vec::new();
    for direction in directions_to_remove {
        action_result.push((direction.clone(), destroy(robot, world, direction.clone())));
    }
    action_result
}

/// Returns a Vec with the direction of the breaking attempt combined with its result from the `put()` interface
pub fn sell_wood(robot: &mut impl Runnable, world: &mut World, quantity: usize) -> Vec<(Direction, Result<usize, LibError>)> {
    let r_view = robot_view(robot, world);
    let mut directions_to_remove = Vec::new();

    match &r_view[0][1] {
        Some(tile) => match tile.content {
            Content::Market(_) => { directions_to_remove.push(Direction::Up) }
            _ => {}
        },
        _ => {}
    }
    match &r_view[2][1] {
        Some(tile) => match tile.content {
            Content::Market(_) => { directions_to_remove.push(Direction::Down) }
            _ => {}
        },
        _ => {}
    }
    match &r_view[1][0] {
        Some(tile) => match tile.content {
            Content::Market(_) => { directions_to_remove.push(Direction::Left) }
            _ => {}
        },
        _ => {}
    }
    match &r_view[1][2] {
        Some(tile) => match tile.content {
            Content::Market(_) => { directions_to_remove.push(Direction::Right) }
            _ => {}
        },
        _ => {}
    }

    let content_in = Content::Tree(quantity);
    let mut action_result: Vec<(Direction, Result<usize, LibError>)> = Vec::new();
    for direction in directions_to_remove {
        action_result.push((direction.clone(), put(robot, world, content_in.clone(), quantity.clone(), direction.clone())));
    }
    action_result
}
