use g_collections_enums::{get_arrow_coords, Coord, Shot};

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let shots: Vec<Shot> = arrow_coords
        .iter()
        .map(|coord| {
            coord.print_description();
            match coord.distance_from_center() {
                x if x < 1.0 => Shot::Bullseye,
                x if x <= 5.0 => Shot::Hit(x),
                _ => Shot::Miss,
            }
        })
        .collect();

    let total: i32 = shots.iter().map(|shot| shot.points()).sum();

    println!("Final point total is: {}", total);
}
