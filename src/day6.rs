use crate::utils::get_input;
use anyhow::Context;

enum GuardDirection {
    Up,
    Down,
    Left,
    Right
}

pub fn solve() -> anyhow::Result<()> {
    let input = get_input()?;

    let lines : Vec<&str> = input.split("\r\n").collect();
    
    let mut map : Vec<Vec<bool>> = Vec::new();
    let mut guard_tracking_map : Vec<Vec<bool>> = Vec::new();


    let mut guard_position: (isize, isize) = (0, 0);
    let mut guard_direction: GuardDirection = GuardDirection::Up;
    for (line_index, line) in lines.iter().enumerate() {
        map.push(Vec::new());
        guard_tracking_map.push(Vec::new());
        for (character_index, character) in line.chars().enumerate() {
            let current_map_line = map.last_mut().context("No vecs in map")?;
            let current_guard_map_line = guard_tracking_map.last_mut().context("No vecs in guard map")?;

            let current_pos = (isize::try_from(character_index)?, isize::try_from(line_index)?);
            match character {
                '.' => {
                    current_map_line.push(false);
                    current_guard_map_line.push(false);

                },
                '#' => {
                    current_map_line.push(true);
                    current_guard_map_line.push(false);
                },
                '^' => {
                    current_map_line.push(false);
                    current_guard_map_line.push(true);
                    guard_position = current_pos;
                    guard_direction = GuardDirection::Up;
                }
                'v' => {
                    current_map_line.push(false);
                    current_guard_map_line.push(true);
                    guard_position = current_pos;
                    guard_direction = GuardDirection::Down;

                },
                '>' => {
                    current_map_line.push(false);
                    current_guard_map_line.push(true);
                    guard_position = current_pos;
                    guard_direction = GuardDirection::Right;
                }
                '<' => {
                    current_map_line.push(false);
                    current_guard_map_line.push(true);
                    guard_position = current_pos;
                    guard_direction = GuardDirection::Left;
                }
                _ => anyhow::bail!("Character: {character} not allowed")
            }
        }
    }

    fn move_until_collision ((x_dir, y_dir): (isize, isize), 
        guard_position: &mut (isize, isize), 
        map: &Vec<Vec<bool>>, 
        guard_direction: &mut GuardDirection,
        guard_tracking_map: &mut Vec<Vec<bool>>
    ) -> bool {

        let new_x_pos: isize = guard_position.0 + x_dir;
        let new_y_pos: isize = guard_position.1 + y_dir;

        if  new_x_pos < 0 
            || new_y_pos < 0 
            || new_x_pos >= isize::try_from(map[0].len()).unwrap() 
            || new_y_pos >= isize::try_from(map[0].len()).unwrap() {
            return true;
        }

        if map[usize::try_from(new_y_pos).unwrap()][usize::try_from(new_x_pos).unwrap()] {
            *guard_direction = match guard_direction {
                GuardDirection::Up => GuardDirection::Right,
                GuardDirection::Right => GuardDirection::Down,
                GuardDirection::Down => GuardDirection::Left,
                GuardDirection::Left => GuardDirection::Up
            }
        }
        else {
            *guard_position = (new_x_pos, new_y_pos);
            guard_tracking_map[usize::try_from(new_y_pos).unwrap()]
                [usize::try_from(new_x_pos).unwrap()] = true;
        }

        false
    }

    loop {
        let escaped = match guard_direction {
            GuardDirection::Up => move_until_collision((0, -1),
                &mut guard_position, &map, &mut guard_direction, &mut guard_tracking_map),
            GuardDirection::Down => move_until_collision((0, 1),
                &mut guard_position, &map, &mut guard_direction, &mut guard_tracking_map),
            GuardDirection::Left => move_until_collision((-1, 0),
                &mut guard_position, &map, &mut guard_direction, &mut guard_tracking_map),
            GuardDirection::Right => move_until_collision((1, 0),
                &mut guard_position, &map, &mut guard_direction, &mut guard_tracking_map),
        };

        if escaped { break; }
    }

    let mut path_count = 0;

    guard_tracking_map.iter().for_each(|vec| vec.iter().for_each(|b| if *b { path_count+= 1}));

    //crate::utils::print_bool_map(&guard_tracking_map, '.', 'X');
    println!("Path Count: {path_count}");

    Ok(())
}

