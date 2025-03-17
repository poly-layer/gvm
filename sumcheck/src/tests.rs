use goldilocks::{Goldilocks, SmallField};
use crate::util::barycentric_weights;

#[test]
fn test_barycentric_weights() {

    let points: Vec<Goldilocks> = vec![
        Goldilocks::from(1u64),
        Goldilocks::from(2u64),
        Goldilocks::from(3u64),
        Goldilocks::from(5u64),
        Goldilocks::from(7u64),
    ];

    // Call barycentric_weights function
    let weights = barycentric_weights(&points);

    // Print weights
    for (i, w) in weights.iter().enumerate() {
        println!("Weight {}: {}", i, w);
    }

}