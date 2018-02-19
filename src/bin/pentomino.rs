extern crate polymate;
use polymate::*;
use std::time::Instant;

fn main() {
    let pieces_base = vec![
        vec![
            ".#.",
            "###",
            ".#.",
        ],
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
            "#..",
            "##.",
            ".##",
        ],
    ];
    let pieces = pieces_base.into_iter().map(|g| (Shape::from_grid(&g), 1)).collect::<Vec<_>>();
    let target = Shape::filled(Coord { x: 10, y: 6, z: 1 });

    let problem = Puzzle { pieces, target };

    let start = Instant::now();
    let ans = solve(&problem);
    let end = start.elapsed();
    
    println!("Solution: {} (Cost: {}.{:03}[s])", ans.count, end.as_secs(), end.subsec_nanos() / 1000000);
    println!("Steps: {}", ans.search_steps);
    println!("First answer:");
    let ans1 = &ans.answer[0];
    for y in 0..ans1.size().y {
        for x in 0..ans1.size().x {
            print!("{:2} ", ans1[Coord { x, y, z: 0 }].0);
        }
        println!();
    }
}
