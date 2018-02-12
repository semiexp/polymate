extern crate polymate;
use polymate::*;
use std::time::Instant;

fn main() {
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

    let start = Instant::now();
    let n_sol = solve(&problem);
    let end = start.elapsed();
    
    println!("Solution: {} (Cost: {}.{:03}[s])", n_sol, end.as_secs(), end.subsec_nanos() / 1000000);
}
