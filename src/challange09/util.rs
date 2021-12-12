use crate::challange09::point::Point;

pub fn find_low_points(height_map: &Vec<u8>, width: usize) -> Vec<usize> {
    let mut low_points: Vec<usize> = Vec::new();

    for i in 0..height_map.len() {
        if get_neighbours(height_map, width, i).iter().all(|p| p.value > height_map[i]) {
            low_points.push(i);
        }
    }

    low_points
}

pub fn get_neighbours(height_map: &Vec<u8>, width: usize, index: usize) -> Vec<Point> {
    let mut neighbours: Vec<Point> = Vec::new();

    if index >= width {
        neighbours.push(Point::new(index - width, height_map[index - width]));
    }

    if index + width < height_map.len() {
        neighbours.push(Point::new(index + width, height_map[index + width]));
    }

    if index % width > 0 {
        neighbours.push(Point::new(index - 1, height_map[index - 1]));
    }

    if index % width < width - 1 {
        neighbours.push(Point::new(index + 1, height_map[index + 1]));
    }

    neighbours
}