// #![allow(unused)]
use std::path::PathBuf;
use structopt::StructOpt;

mod convex_hull;
mod functions;
mod point;
mod plane;
mod edge;
mod hash_stuff;

use convex_hull::*;
use functions::*;
use point::*;


#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "100")]
    num_points: u32,
    
    #[structopt(short, long, default_value = "convex_hull.json", parse(from_os_str))]
    output_file: PathBuf,
}

fn main() {
    let args = Cli::from_args();  // Get parsed arguments

    // let points: Vec<Point> = populate_point_vec();
    let points: Vec<Point> = create_rng_ponts(args.num_points);

    let mut convex_hull = ConvexHull::new(points.to_vec());

    let start = std::time::Instant::now();
    convex_hull.quick_hull();
    let duration = start.elapsed();

    println!("# of planes: {}", convex_hull.planes.len());

    // save the convex hull to a file
    save_to_json(&args.output_file.to_str().unwrap(), &convex_hull);

    println!("Time elapsed in expensive_function() is: {:?}", duration);

}
