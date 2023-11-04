///
///
/// # Arguments
///
/// * `wex`:
/// * `wey`:
/// * `list`:
///
/// returns: Vec<Vec<u32, Global>, Global>
///
/// # Examples
///
/// ```
///
/// ```
pub fn queen_actions(wex: u32, wey: u32, mut list: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(i) {
            if x <= 8 {
                if let Some(y) = wey.checked_add(i) {
                    if y <= 8 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(i) {
            if x <= 8 {
                if let Some(y) = wey.checked_sub(i) {
                    if y > 0 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_sub(i) {
            if x > 0 {
                if let Some(y) = wey.checked_add(i) {
                    if y <= 8 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_sub(i) {
            if x > 0 {
                if let Some(y) = wey.checked_sub(i) {
                    if y > 0 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }



    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(0) {
            if x <= 8 {
                if let Some(y) = wey.checked_add(i) {
                    if y <= 8 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(0) {
            if x <= 8 {
                if let Some(y) = wey.checked_sub(i) {
                    if y > 0 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_sub(i) {
            if x > 0 {
                if let Some(y) = wey.checked_add(0) {
                    if y <= 8 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(i) {
            if x <= 8 {
                if let Some(y) = wey.checked_sub(0) {
                    if y > 0 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    return list
}

///
///
/// # Arguments
///
/// * `wex`:
/// * `wey`:
/// * `list`:
///
/// returns: Vec<Vec<u32, Global>, Global>
///
/// # Examples
///
/// ```
///
/// ```
pub fn elephant_actions(wex: u32, wey: u32, mut list: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(i) {
            if x <= 8 {
                if let Some(y) = wey.checked_add(i) {
                    if y <= 8 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_add(i) {
            if x <= 8 {
                if let Some(y) = wey.checked_sub(i) {
                    if y > 0 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_sub(i) {
            if x > 0 {
                if let Some(y) = wey.checked_add(i) {
                    if y <= 8 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    for i in 1..=8u32 {
        if let Some(x) = wex.checked_sub(i) {
            if x > 0 {
                if let Some(y) = wey.checked_sub(i) {
                    if y > 0 {
                        list.push(vec![x, y]);
                    }
                }
            }
        }
    }
    return list
}

///
///
/// # Arguments
///
/// * `list`:
/// * `x`:
/// * `y`:
/// * `figure1_name`:
/// * `figure2_name`:
/// * `action`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn is_piece_within_moves(list: Vec<Vec<u32>>, x: u32, y: u32, figure1_name: &str, figure2_name: &str, action: &str ) {
    let figure_coords = vec![x, y];

    if list.iter().any(|coords| coords == &figure_coords) {
        println!("{} {} {}", figure1_name, action, figure2_name);
    } else {
        println!("{} do not {} {}", figure1_name, action, figure2_name);
    }
}
