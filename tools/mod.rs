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

// if function returns (65535, 65535) no matching content has been found
// market:bool is set to true if looking for a market, else it will look for a tree
fn find_stuff(robot:Bot, mut world:&mut robotics_lib::world::World, market:bool) -> (i32, i32) {
    let mut self_x = robot.get_coordinate().get_row() as i32;
    let mut self_y = robot.get_coordinate().get_col() as i32;

    let r_map = robot_map(&mut world).unwrap();

    let mut spiral_radius = 1;
    let mut current_tile = r_map[self_x][self_y];

    for i in 0..3 {
        for _ in 0..spiral_radius {
            match i {
                0 => {
                    self_x += 1
                },
                1 => {
                    self_y += 1
                },
                2 => {
                    self_x -= 1
                },
                3 => {
                    self_y -= 1
                },
                _ => {}
            }
            current_tile = r_map[self_x][self_y];

            match current_tile.content {
                Content::Tree(_) => {
                    if !market {
                        return (self_x, self_y)
                    }
                },
                Content::Market(_) => {
                    if market {
                        return (self_x, self_y)
                    }
                }
                _ => {}
            }
        }

        spiral_radius += 1;
    }
    return (65535, 65535)
}
