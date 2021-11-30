fn main() {
    let input = include_str!("./input.txt").trim_end();

    let grid = input
        .split("\n")
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part_1(&grid);
    part_2(&grid);
}

fn calculate_slope(grid: &Vec<Vec<bool>>, (right, down): (usize, usize)) -> i32 {
    let mut count = 0;

    let mut x = (0..usize::MAX).step_by(right);

    for y in (0..grid.len()).step_by(down) {
        if grid[y][x.next().unwrap() % grid[y].len()] {
            count += 1;
        }
    }

    count
}

fn part_1(grid: &Vec<Vec<bool>>) {
    println!("{}", calculate_slope(grid, (3, 1)));
}

fn part_2(grid: &Vec<Vec<bool>>) {
    let value = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope| calculate_slope(grid, *slope))
        .product::<i32>();

    println!("{}", value);
}
