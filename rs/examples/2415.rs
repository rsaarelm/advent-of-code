use aoc::prelude::*;

fn can_push(bounds: &Rect<i32>, grid: &[char], pos: IVec2, dir: IVec2) -> bool {
    match grid[bounds.idx(pos)] {
        '#' => false,
        '.' => true,
        'O' => can_push(bounds, grid, pos + dir, dir),
        // Crates act like boxes along x-axis are trickier along y.
        '[' => {
            can_push(bounds, grid, pos + dir, dir)
                && (dir.y == 0
                    || can_push(bounds, grid, pos + ivec2(1, 0) + dir, dir))
        }
        ']' => {
            can_push(bounds, grid, pos + dir, dir)
                && (dir.y == 0
                    || can_push(bounds, grid, pos - ivec2(1, 0) + dir, dir))
        }
        _ => panic!(),
    }
}

fn push(bounds: &Rect<i32>, grid: &mut [char], pos: IVec2, dir: IVec2) -> bool {
    if !can_push(bounds, grid, pos, dir) {
        return false;
    }

    match grid[bounds.idx(pos)] {
        'O' => {
            push(bounds, grid, pos + dir, dir);
            grid[bounds.idx(pos)] = '.';
            grid[bounds.idx(pos + dir)] = 'O';
        }
        // Crates act like boxes along x-axis are trickier along y.
        // XXX: Lots of repeated code between the two cases.
        '[' => {
            if dir.y != 0 {
                push(bounds, grid, pos + dir, dir);
                push(bounds, grid, pos + dir + ivec2(1, 0), dir);
            } else {
                push(bounds, grid, pos + dir * 2, dir);
            }
            grid[bounds.idx(pos)] = '.';
            grid[bounds.idx(pos + ivec2(1, 0))] = '.';
            grid[bounds.idx(pos + dir)] = '[';
            grid[bounds.idx(pos + dir + ivec2(1, 0))] = ']';
        }
        ']' => {
            if dir.y != 0 {
                push(bounds, grid, pos + dir, dir);
                push(bounds, grid, pos + dir - ivec2(1, 0), dir);
            } else {
                push(bounds, grid, pos + dir * 2, dir);
            }
            grid[bounds.idx(pos - ivec2(1, 0))] = '.';
            grid[bounds.idx(pos)] = '.';
            grid[bounds.idx(pos + dir - ivec2(1, 0))] = '[';
            grid[bounds.idx(pos + dir)] = ']';
        }
        _ => {}
    }
    true
}

fn run(bounds: &Rect<i32>, grid: &mut [char], dirs: &[IVec2]) -> i32 {
    // Find robot pos.
    let robot = grid.iter().position(|&a| a == '@').unwrap();
    grid[robot] = '.';
    let mut robot = IVec2::from(bounds.get(robot));

    for &dir in dirs {
        if push(bounds, grid, robot + dir, dir) {
            robot += dir;
        }
    }

    let mut ret = 0;
    for p @ [x, y] in *bounds {
        if grid[bounds.idx(p)] == 'O' || grid[bounds.idx(p)] == '[' {
            ret += x + y * 100;
        }
    }
    ret
}

fn main() {
    let input = stdin_string();
    let parts: Vec<_> = input.split("\n\n").collect();
    let (bounds, grid) = grid(parts[0]);
    let dirs = parts[1].chars().filter_map(char_to_dir).collect::<Vec<_>>();

    println!("{}", run(&bounds, &mut grid.clone(), &dirs));

    let grid2 = grid
        .iter()
        .map(|c| match c {
            '#' => "##",
            '.' => "..",
            'O' => "[]",
            '@' => "@.",
            '\n' => "\n",
            _ => panic!(),
        })
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .collect::<Vec<char>>();
    let bounds2 = area(bounds.width() * 2, bounds.height());

    println!("{}", run(&bounds2, &mut grid2.clone(), &dirs));
}
