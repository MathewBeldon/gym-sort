use std::io;
use rand::{seq::SliceRandom, Rng};

const RUN_ITERATIONS:i32 = 100000;
const RUN_USERS:usize = 10;
const SIMULATION_ITERATIONS:i32 = 1000;

const TAKE_FROM_FLOOR_CHANCE:f64 = 0.1;
const IS_GYM_BRO_CHANCE:f64 = 1.0;
const IS_LAZY_CHANCE:f64 = 0.2;

fn main() -> io::Result<()> {

    let mut saved_records = Vec::<i32>::new();

    for _ in 0..SIMULATION_ITERATIONS {

        let mut rack_dumbbells = vec![2, 12, 26, 10, 10, 18, 6, 22, 14, 14, 12, 2, 6, 36, 48, 8, 20, 44, 42, 46, 20, 16, 14, 38, 14, 10, 18, 34, 16, 8, 4, 40, 50, 26, 30, 24, 32, 28, 16, 12, 10, 4, 16, 24, 22, 12];
        let rack_space = vec![2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 10, 10, 12, 12, 12, 12, 14, 14, 14, 14, 16, 16, 16, 16, 18, 18, 20, 20, 22, 22, 24, 24, 26, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50];
        let mut floor_dumbbells = Vec::<i32>::new();
        
        let mut gym_users: Vec<(i32, i32)> = Vec::new();

        for iter in 0..RUN_ITERATIONS {

            if gym_users.len() < RUN_USERS {

                let available_weights = rack_dumbbells
                    .iter()
                    .filter(|&&x| x != 0)
                    .cloned()
                    .collect::<Vec<_>>();

                let chosen_weight = *available_weights
                    .choose(&mut rand::thread_rng())
                    .unwrap();

                let mut new_user:(i32, i32) = (rand::thread_rng().gen_range(1..10), 0);
                
                let rack_positions: Vec<usize> = rack_dumbbells
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &value)| if value == chosen_weight { Some(index) } else { None })
                    .collect();

                let floor_positions: Vec<usize> = floor_dumbbells
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &value)| if value == chosen_weight { Some(index) } else { None })
                    .collect();

                let use_floor = rand::thread_rng().gen::<f64>() <= TAKE_FROM_FLOOR_CHANCE && !floor_positions.is_empty() || rack_positions.is_empty();

                new_user.1 = if use_floor {
                    let pos = *floor_positions.choose(&mut rand::thread_rng()).unwrap();
                    let ret = floor_dumbbells[pos];
                    floor_dumbbells.remove(pos);
                    ret
                } else {
                    let pos = *rack_positions.choose(&mut rand::thread_rng()).unwrap();
                    let ret: i32 = rack_dumbbells[pos];
                    rack_dumbbells[pos] = 0;
                    ret
                };

                let mut remove_index:Vec<usize> = Vec::new();
                for (index, gym_user) in gym_users.iter_mut().enumerate() {

                    if gym_user.0 > 1 {
                        continue;
                    }

                    remove_index.push(index);

                    let rack_space_index_range:Vec<usize> = rack_space
                        .iter()
                        .enumerate()
                        .filter_map(|(index, &value)| if value == gym_user.1 { Some(index) } else { None })
                        .collect();

                    let great_rack_spaces: Vec<usize> = rack_space_index_range
                        .iter()
                        .enumerate()
                        .filter_map(|(_, &value)| if rack_space_index_range.contains(&value) && rack_dumbbells[value] == 0 { Some(value) } else { None })
                        .collect();
                    
                    let good_rack_spaces: Vec<usize> = rack_space_index_range
                        .iter()
                        .enumerate()
                        .filter_map(|(_, &value)| if rack_space_index_range.contains(&value) && rack_dumbbells[value] != gym_user.1 { Some(value) } else { None })
                        .collect();

                    let any_rack_space: Vec<usize> = rack_dumbbells
                        .iter()
                        .enumerate()
                        .filter_map(|(index, &value)| if value == 0 { Some(index) } else { None })
                        .collect();

                    let is_gym_bro = rand::thread_rng().gen::<f64>() <= IS_GYM_BRO_CHANCE;
                    let is_lazy = rand::thread_rng().gen::<f64>() <= IS_LAZY_CHANCE;

                    if is_gym_bro {
                        if great_rack_spaces.len() > 0 {
                            let selected_great_rack_space = *great_rack_spaces.choose(&mut rand::thread_rng()).unwrap();
                            rack_dumbbells[selected_great_rack_space] = gym_user.1;
                        } else {
                            let selected_good_rack_space = *good_rack_spaces.choose(&mut rand::thread_rng()).unwrap();
                            floor_dumbbells.push(rack_dumbbells[selected_good_rack_space]);
                            rack_dumbbells[selected_good_rack_space] = gym_user.1;
                        }
                    } else {
                        if great_rack_spaces.len() > 0 && !is_lazy {
                            let selected_great_rack_space = *great_rack_spaces.choose(&mut rand::thread_rng()).unwrap();
                            rack_dumbbells[selected_great_rack_space] = gym_user.1;
                        } else {
                            let selected_any_rack_space = *any_rack_space.choose(&mut rand::thread_rng()).unwrap();
                            rack_dumbbells[selected_any_rack_space] = gym_user.1;
                        }
                    }
                }

                let mut removed_count = 0;
                for index in remove_index {
                    gym_users.remove(index - removed_count);
                    removed_count += 1;
                }

                gym_users.push(new_user);
            }

            for gym_user in gym_users.iter_mut(){
                gym_user.0 -= 1;
            }

            let is_sorted = rack_dumbbells.iter().enumerate().all(|(i, &x)| x == 0 || x == rack_space[i]);

            if is_sorted {
                saved_records.push(iter);
                break;
            }
        }
    }

    let sum: i32 = saved_records.iter().sum();
    let count = saved_records.len() as i32;
    
    println!("avg = {:?}", sum / count);

    Ok(())
}

