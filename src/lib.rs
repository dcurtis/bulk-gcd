extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::ops::Pow;
use rug::Integer;

pub struct ComputeOptions {
    pub debug: bool,
}

struct ProductTree {
    levels: Vec<Vec<Integer>>,
}

fn compute_product_tree(moduli: Vec<Integer>) -> ProductTree {
    // Root
    if moduli.len() == 1 {
        return ProductTree { levels: vec![ moduli ] }
    }

    // Node
    let level = (0..(moduli.len() / 2))
        .into_par_iter()
        .map(|i| {
            Integer::from(&moduli[i * 2] * &moduli[i * 2 + 1])
        })
        .collect();

    let mut res = compute_product_tree(level);
    res.levels.push(moduli);
    return res;
}

fn compute_remainders(tree: &ProductTree,
                      options: &ComputeOptions) -> Vec<Integer> {
    if options.debug {
        eprintln!("computing remainders");
    }

    let root = tree.levels[0].clone();
    return tree.levels[1..].iter().fold(root, |acc, curr| {
        curr.par_iter().enumerate().map(|(i, value)| {
            let parent = &acc[i / 2];
            let square = Integer::from(value.pow(2));
            return Integer::from(parent % square);
        }).collect()
    })
}

fn compute_gcds(remainders: &Vec<Integer>,
                moduli: &Vec<Integer>,
                options: &ComputeOptions) -> Vec<Integer> {
    if options.debug {
        eprintln!("computing quotients and gcd");
    }

    // TODO(indutny): parallelize this!
    remainders
        .iter()
        .zip(moduli)
        .map(|(remainder, modulo)| {
            let quotient = Integer::from(remainder / modulo);
            quotient.gcd(modulo)
        })
        .collect()
}

pub fn compute(mut moduli: Vec<Integer>,
               options: &ComputeOptions) -> Vec<Option<Integer>> {
    // Pad to the power-of-two len
    let mut pad_size: usize = 1;
    loop {
        if pad_size >= moduli.len() {
            break;
        }
        pad_size <<= 1;
    }
    pad_size -= moduli.len();

    if options.debug {
        eprintln!("adding {} padding to moduli", pad_size);
    }

    for _ in 0..pad_size {
        moduli.push(Integer::from(1));
    }

    if options.debug {
        eprintln!("computing product tree");
    }

    let product_tree = compute_product_tree(moduli);
    let remainders = compute_remainders(&product_tree, options);
    let gcds = compute_gcds(&remainders,
                            product_tree.levels.last().unwrap(),
                            options);

    let one = Integer::from(1);

    gcds.into_iter().map(|gcd| {
        if gcd == one {
            None
        } else {
            Some(gcd)
        }
    }).collect()
}
