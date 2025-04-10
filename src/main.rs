use ai::{
    exploration::Agent,
    frontiers::{AStar, MinCost},
};
use models::hp_2d_protein_folding::{Alphabet, AminoAcid, Energy, Pos, Protein};
use rayon::prelude::*;
use std::time::{Duration, Instant};
pub mod models;

fn main() {
    use Alphabet::*;

    const LENGTH: usize = 20;
    const EXECUT: usize = 10000;

    // let sequences: Vec<Vec<_>> = (0..EXECUT)
    //     .map(|i| {
    //         (1..LENGTH)
    //             .map(|v| {
    //                 let cnt = ((i as f64 / EXECUT as f64) * (LENGTH as f64)) as usize;
    //                 if v < cnt { H } else { P }
    //             })
    //             .collect()
    //     })
    //     .collect();
    //
    // let simulation_time = Instant::now();
    // let times: Vec<_> = sequences
    //     .into_par_iter()
    //     .map(|s| {
    //         let h_count = s.iter().filter(|a| matches!(a, H)).count();
    //
    //         let time = Instant::now();
    //         let mut agent = Agent::new(Protein::new(s.clone()));
    //         while agent
    //             .tree_function::<AminoAcid, MinCost<Pos, Energy>>(AminoAcid::default())
    //             .is_some()
    //         {}
    //
    //         (time.elapsed(), h_count)
    //     })
    //     .collect();
    //
    // println!("simulation: {:?}", simulation_time.elapsed());

    // let mut rng = rng();
    // if rng.random_bool((i as f64 / EXECUT as f64).clamp(0.0, 1.0)) {
    // let mut h_count = 0;
    // for (i, a) in s[..s.len() - 2].iter().enumerate() {
    //     if matches!((a, s[i + 2].clone()), (H, H)) {
    //         h_count += 1;
    //     }
    // }

    // il costo è numero di contatti non realizzati

    // let mut sums = [Duration::new(0, 0); LENGTH];
    // let mut cnts = [0; LENGTH];
    //
    // for (t, h_count) in times {
    //     cnts[h_count] += 1;
    //     sums[h_count] += t;
    // }
    //
    // let avg: Vec<_> = sums
    //     .iter()
    //     .zip(cnts)
    //     .map(|(s, n)| s.as_millis() as f64 / n as f64)
    //     .collect();
    //
    // println!("{:?}", avg);

    // let mut conformation = vec![(0, 0)];
    // conformation.push(pos);

    // let sequence = vec![
    //     H, H, P, H, P, P, H, H, H, P, P, P, P, H, H, P, H, P, H, P, P, H, P, H, P, H, H,
    // ];
    let sequence = vec![H, H, P, H, P, P, H, H, H, P, P, P, P, H, H, P, H, P, H, P];
    // let sequence = vec![H, H, P, H, P, P, H, H, H, P, P, P, P, H, H, P];
    // println!("{}", sequence.len());
    // let mut rng = rng();
    // let sequence: Sequence = (0..22)
    //     .filter_map(|_| [P, H].choose(&mut rng))
    //     .map(Clone::clone)
    //     .collect();

    let mut agent = Agent::new(Protein::new(sequence.clone()));

    let mut conformation = vec![(0, 0)];
    let time = Instant::now();
    while let Some(pos) =
        agent.tree_function::<AminoAcid, MinCost<Pos, Energy>>(AminoAcid::default())
    {
        conformation.push(pos);
    }
    println!("{:?}", time.elapsed());

    let max_col = conformation.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_row = conformation.iter().map(|(_, y)| *y).max().unwrap_or(0);
    let min_col = conformation.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let min_row = conformation.iter().map(|(_, y)| *y).min().unwrap_or(0);

    for y in min_row..max_row + 2 {
        for x in min_col..max_col + 2 {
            if let Some((i, _)) = conformation
                .iter()
                .enumerate()
                .find(|(_, (p_x, p_y))| x == *p_x && y == *p_y)
            {
                if x == 0 && y == 0 {
                    print!("X")
                } else {
                    print!("{:?}", sequence[i])
                }
            } else {
                print!(".")
            }
        }
        println!()
    }

    for (i, pos) in conformation.iter().enumerate() {
        println!("{:?}: {:?}", sequence[i], pos)
    }

    let mut energy = 0;
    for (i, &(u_x, u_y)) in conformation.iter().enumerate() {
        for (j, &(v_x, v_y)) in conformation[..0.max(i.abs_diff(1))].iter().enumerate() {
            if let (H, H) = (&sequence[i], &sequence[j]) {
                if u_x.abs_diff(v_x) + u_y.abs_diff(v_y) == 1 {
                    energy += 1;
                }
            }
        }
    }
    println!("-{energy}\n\n");

    let mut agent = Agent::new(Protein::new(sequence.clone()));

    let mut conformation = vec![(0, 0)];
    let time = Instant::now();
    while let Some(pos) = agent.tree_function::<AminoAcid, AStar<Pos, Energy>>(AminoAcid::default())
    {
        conformation.push(pos);
    }
    println!("\n{:?}", time.elapsed());

    let max_col = conformation.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_row = conformation.iter().map(|(_, y)| *y).max().unwrap_or(0);
    let min_col = conformation.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let min_row = conformation.iter().map(|(_, y)| *y).min().unwrap_or(0);

    for y in min_row..max_row + 2 {
        for x in min_col..max_col + 2 {
            if let Some((i, _)) = conformation
                .iter()
                .enumerate()
                .find(|(_, (p_x, p_y))| x == *p_x && y == *p_y)
            {
                if x == 0 && y == 0 {
                    print!("X")
                } else {
                    print!("{:?}", sequence[i])
                }
            } else {
                print!(".")
            }
        }
        println!()
    }

    for (i, pos) in conformation.iter().enumerate() {
        println!("{:?}: {:?}", sequence[i], pos)
    }

    let mut energy = 0;
    for (i, &(u_x, u_y)) in conformation.iter().enumerate() {
        for (j, &(v_x, v_y)) in conformation[..0.max(i.abs_diff(1))].iter().enumerate() {
            if let (H, H) = (&sequence[i], &sequence[j]) {
                if u_x.abs_diff(v_x) + u_y.abs_diff(v_y) == 1 {
                    energy += 1;
                }
            }
        }
    }
    println!("-{energy}\n\n");
}

// let sequence = vec![H, H, H, P, P, H, P, H, H, P, H, H, P, H, H, P, H, P];

// let sequence = vec![H, H, P, H, P, P, H, H, H, P, P, P, P, H, H, P];
//let sequence = vec![P, H, H, P, H, P, P, H, P];

//let sequence = vec![P, P, P, P, P, P, P, P, P];
//let sequence = vec![P, P, P, P, P, P, P, P, H];
//let sequence = vec![P, P, P, P, P, P, H, P, H];
// let sequence = vec![H, P, P, P, P, H, P, P, P, H, P, P, P, P, H, P, P, P, H];
//let sequence = vec![P, P, H, P, H, P, H, P, H];
//let sequence = vec![H, P, H, P, H, P, H, P, H];
//let sequence = vec![H, P, H, P, H, P, H, H, H];
//let sequence = vec![H, P, H, P, H, H, H, H, H];
//let sequence = vec![H, P, H, H, H, H, H, H, H];
//let sequence = vec![H, H, H, H, H, H, H, H, H];

// TODO: la distanza "pari" fra due H è interessante... si potrebbe precalcolare
// guacamole: la distanza "pari" gioca un ruolo fondamentale, vaccaccia

//while let Some(pos) = agent.function::<Conformation, MinCost<Conformation, Pos, Energy>>(vec![])

// use ai::problem::Utility;
// use models::n_queens::NQueens;

//for i in 1..=5_i32 {
//    for j in 1..=5 {
//        if i != j {
//            println!("{{Q_{i}, Q_{j}}}");
//
//            println!("{{");
//            for x in 1..=5_i32 {
//                for y in 1..=5 {
//                    if i.abs_diff(j) != x.abs_diff(y) {
//                        print!("({x}, {y}), ")
//                    }
//                }
//            }
//            println!("\n}}")
//        }
//    }
//}

//use ai::iterative_search::{
//    hill_climping, parallel_steepest_ascent, simulated_annealing, steepest_ascent,
//};
//use rayon::prelude::*;
//use std::cmp::Reverse;
//fn bench<F>(n_queens: &NQueens, f: F)
//where
//    F: Fn() -> Option<Vec<usize>>,
//{
//    let time = Instant::now();
//    let optimal = f();
//    println!("{:?}", time.elapsed());
//    if let Some(optimal) = optimal {
//        println!("{:?}", n_queens.utility(&optimal))
//    }
//}

//let n_queens = NQueens(128);
//
//bench(&n_queens, || {
//    simulated_annealing(
//        &n_queens,
//        &mut rand::rng(),
//        |t| -0.0001 * (t as f64) + 20f64,
//        |Reverse(u1), Reverse(u2)| u1.abs_diff(*u2) as f64,
//    )
//});
//
//bench(&n_queens, || {
//    simulated_annealing(
//        &n_queens,
//        &mut rand::rng(),
//        |t| 1f64 / (t as f64),
//        |Reverse(u1), Reverse(u2)| u1.abs_diff(*u2) as f64,
//    )
//});
//
//bench(&n_queens, || {
//    (0..=100)
//        .into_par_iter()
//        .filter_map(|_| parallel_steepest_ascent(&n_queens, &mut rand::rng()))
//        .max_by_key(|b| n_queens.utility(b))
//});
//
//bench(&n_queens, || {
//    (0..=100)
//        .into_par_iter()
//        .filter_map(|_| steepest_ascent(&n_queens, &mut rand::rng()))
//        .max_by_key(|b| n_queens.utility(b))
//});
//
//bench(&n_queens, || {
//    (0..=100)
//        .into_par_iter()
//        .filter_map(|_| hill_climping(&n_queens, &mut rand::rng(), 1000))
//        .max_by_key(|b| n_queens.utility(b))
//});
