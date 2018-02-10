use super::*;

fn grid_to_shape(grid: &[&'static str]) -> Shape {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut ret = Shape::new(Coord { x: width, y: height, z: 1 });

    for y in 0..height {
        let mut it = grid[y as usize].chars();
        for x in 0..width {
            ret.set(Coord { x, y, z: 0 }, it.next() == Some('#'));
        }
    }
    ret
}

#[test]
fn test_pentomino() {
    let pieces_base = vec![
        vec![
            "#####",
        ],
        vec![
            "####",
            "#...",
        ],
        vec![
            "####",
            ".#..",
        ],
        vec![
            ".###",
            "##..",
        ],
        vec![
            "###",
            "##.",
        ],
        vec![
            "###",
            "#.#",
        ],
        vec![
            "###",
            ".#.",
            ".#.",
        ],
        vec![
            "###",
            "#..",
            "#..",
        ],
        vec![
            "##.",
            ".#.",
            ".##",
        ],
        vec![
            ".##",
            "##.",
            ".#.",
        ],
        vec![
            ".#.",
            "###",
            ".#.",
        ],
        vec![
            "#..",
            "##.",
            ".##",
        ],
    ];
    let pieces = pieces_base.into_iter().map(|g| (grid_to_shape(&g), 1)).collect::<Vec<_>>();
    let target = Shape::filled(Coord { x: 10, y: 6, z: 1 });

    let problem = Puzzle { pieces, target };

    let n_sol = solve(&problem);
    assert_eq!(n_sol, 4 * 2339);
}
