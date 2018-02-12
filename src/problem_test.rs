use super::*;

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
    let pieces = pieces_base.into_iter().map(|g| (Shape::from_grid(&g), 1)).collect::<Vec<_>>();
    let target = Shape::filled(Coord { x: 10, y: 6, z: 1 });

    let problem = Puzzle { pieces, target };

    let n_sol = solve(&problem);
    assert_eq!(n_sol, 4 * 2339);
}
