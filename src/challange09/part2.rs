use crate::challange09::input::ChallangeInput09;
use crate::challange09::point::Point;
use crate::challange09::util;
use std::collections::HashSet;

pub fn run(input: ChallangeInput09) -> String {
    let low_points = util::find_low_points(&input.values, input.width);
    let mut basins: Vec<usize> = Vec::new();

    for low_point in low_points {
        basins.push(get_basin_size(&input.values, input.width, low_point));
    }

    if basins.len() < 3 {
        panic!("Nooo to few basins found");
    }

    basins.sort_by(|a, b| b.cmp(a));
    (basins[0] * basins[1] * basins[2]).to_string()
}

fn get_basin_size(height_map: &Vec<u8>, width: usize, index: usize) -> usize {
    let mut indicies: HashSet<usize> = HashSet::from([ index ]);
    let mut points: Vec<Point> = get_non_peak_neigbours(height_map, width, index);
    let mut index: usize = 0;

    while index < points.len() {
        let current = &points[index];

        if indicies.insert(current.index) {
            for neighbour in get_non_peak_neigbours(height_map, width, current.index) {
                if !indicies.contains(&neighbour.index) {
                    points.push(neighbour);
                }
            }
        }

        index += 1;
    }

    indicies.len()
}

fn get_non_peak_neigbours(height_map: &Vec<u8>, width: usize, index: usize) -> Vec<Point> {
    util::get_neighbours(height_map, width, index).iter().filter_map(|p| {
        if p.value < 9 { Some(*p) } else { None }
    }).collect::<Vec<Point>>()
}