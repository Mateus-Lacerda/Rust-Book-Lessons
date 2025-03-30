use rand::{rng as rand_range, RngCore};

fn build_array(initialize: bool) -> [[u32; 10];10] {
    let mut arr: [[u32; 10];10] = [
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
    ];
    if !initialize {return arr};
    for i in 0..10 {
        for j in 0..10 {
            arr[i][j] = rand_range().next_u32() % 2;
        }
    }
    return arr
}

fn print_visited(
    arr_to_search: [[u32; 10];10],
    current_position:(usize,usize)
) {
    for i in 0..10 {
        for j in 0..10 {
            if (i,j) == current_position{
                print!("X | ")
            }
            else {
                print!("{} | ", arr_to_search[i][j])
            }    
        }
        println!();
    }
}

pub fn setup() {
    let arr_to_search = build_array(true);
    println!("This is the array");
    for line in arr_to_search {
        for value in line {
            print!("{value} | ")
        }
        println!();
    }
    let mut visited = build_array(false);
    visited[0][0] = 1;
    search_dumb(arr_to_search, visited, (0,0), 0, 1);
}

// This dumb func aims to search an entire matrix to find out how many ones there are
fn search_dumb(
    arr_to_search: [[u32; 10];10],
    mut visited:[[u32; 10];10],
    mut current_position:(usize,usize),
    ones_count:u8,
    visited_count:u8
){
    print_visited(arr_to_search, current_position);
    if visited_count == 1 && arr_to_search[0][0] == 1 {
        search_dumb(arr_to_search, visited, current_position, ones_count+1, visited_count);
    }
    if visited_count >= 100 {
        println!("{ones_count} one were found.");
        return;
    }
    println!("current pos ({},{})", current_position.0, current_position.1);
    let dumb_move = rand_range().next_u32() % 4;
    println!("Selected dumb move {}", dumb_move);
    // 0 -> Move up 
    // 1 -> Move down
    // 2 -> Move left
    // 3 -> Move right
    match dumb_move {
        0 => {
            if current_position.1 == 0 {
                println!("Cant move up");
                println!("{}", current_position.1);
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            else if visited[current_position.0][current_position.1 - 1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            current_position.1 -= 1;
            visited[current_position.0][current_position.1] = 1;
            if arr_to_search[current_position.0][current_position.0 - 1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count+1, visited_count+1);
            }
            else {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count+1);
            }
        }
        1 => {
            if current_position.1 == 9 {
                println!("Cant move down");
                println!("{}", current_position.1);
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            else if visited[current_position.0][current_position.1 + 1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            current_position.1 += 1;
            visited[current_position.0][current_position.1] = 1;
            if arr_to_search[current_position.0][current_position.0 + 1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count+1, visited_count+1);
            }
            else {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count+1);
            }

        }
        2 => {
            if current_position.0 == 0 {
                println!("Cant move left");
                println!("{}", current_position.0);
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            else if visited[current_position.0 - 1][current_position.1 ] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            current_position.0 -= 1;
            visited[current_position.0][current_position.1] = 1;
            if arr_to_search[current_position.0 + 1][current_position.1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count+1, visited_count+1);
            }
            else {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count+1);
            }
        }
        3 => {
            if current_position.0 == 9 {
                println!("Cant move right");
                println!("{}", current_position.0);
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            else if visited[current_position.0 + 1][current_position.1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count);
            }
            current_position.0 += 1;
            visited[current_position.0][current_position.1] = 1;
            if arr_to_search[current_position.0 + 1][current_position.1] == 1 {
                search_dumb(arr_to_search, visited, current_position, ones_count+1, visited_count+1);
            }
            else {
                search_dumb(arr_to_search, visited, current_position, ones_count, visited_count+1);
            }

        }
        _ => {println!("WTH"); search_dumb(arr_to_search, visited, current_position, ones_count, visited_count+1);} 
    }
}

