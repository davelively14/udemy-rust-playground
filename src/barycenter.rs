extern crate itertools;
extern crate rayon;
use itertools::Itertools;
use rayon::prelude::*;

mod bodies;
use bodies::get_values;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

impl Body {
    fn new(x: f64, y: f64, z: f64, mass: f64) -> Body {
        Body {
            x: x,
            y: y,
            z: z,
            mass: mass,
        }
    }
}

fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn average_with_mass(a: f64, b: f64, amass: f64, bmass: f64) -> f64 {
    average(a * amass, b * bmass) / (amass + bmass)
}

fn merge_two_bodies(a: Body, b: Body) -> Body {
    Body {
        x: average_with_mass(a.x, b.x, a.mass, b.mass),
        y: average_with_mass(a.y, b.y, a.mass, b.mass),
        z: average_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass,
    }
}

// Takes a slice of bodies and returns a single Body
fn merge_all_bodies_recursive(bodies: &[Body]) -> Body {
    println!("Bodies remaining: {}", bodies.len());

    // Terminating condition: we've processed all bodies. Return the only body.
    if bodies.len() == 1 {
        return bodies[0];
    }

    // Takes pair of bodies and merges.
    // .iter => returns an iterator over a slice
    // .tuples => groups the items in tuples of a specific size, 2 in this case. If odd, it leaves off the last one.
    // .collect => turn any iterator into a relevant collection
    let tuples: Vec<(_)> = bodies.iter().tuples().collect();

    // merged_bodies => vec of bodies already operated on
    // inter_par_iter => parallel iterator from rayon
    // The result is roughly half of previous set
    let mut merged_bodies: Vec<_> = tuples
        .into_par_iter()
        .map(|(a, b)| merge_two_bodies(*a, *b))
        .collect();

    // If we had an odd number, .tuples above abandoned the last one. We made
    // merged_bodies mutable just so we could add in the odd item out if
    // necessary.
    if bodies.len() % 2 != 0 {
        merged_bodies.push(bodies[bodies.len() - 1]);
    }

    return merge_all_bodies_recursive(&merged_bodies);
}

pub fn run_examples() {
    let bodies = get_values();
    let barycenter = merge_all_bodies_recursive(&bodies);

    println!(
        "Barrycenter: ({}, {}, {})\nMass: {}",
        barycenter.x, barycenter.y, barycenter.z, barycenter.mass
    );
}
