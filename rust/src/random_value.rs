use rand::Rng;

// TODO: is this used still?
/*
// Places random values between a, b for len c.
fn randomize_array(a : u32, b : u32, c : usize) -> Vec<u32> {
    let mut rng;

    // Create array:
    let mut array = Vec::new();

    // // Insert into array:
    for _i in 0..c {
    	rng = rand::rng().random_range(a..b+1);
    	array.push(rng.try_into().unwrap());
    }
    array
}
*/

/*
* Given three parameters, a, b, c, returns a 2D vector of randomized values.
* a (u32), b (u32) -- random_range(a..b+1) for each element in inner vector.
* c (usize): Number of arrays.
*
* The length of the inner array is determined by:
*   [0..random_range(b / 2..=b)].
*/
pub fn randomize_array_set(a : u32, b : u32, c : usize) -> Vec<Vec<u32>> {
    let mut rng;

    // Create c arrays
    let mut array: Vec<Vec<u32>> = vec![];
    let mut rng_gen = rand::rng();

    // Insert into array:
    for _i in 0..c {
        // get random size for vec between a and b.
        let length : usize= rng_gen.random_range(b / 2..=b).try_into().unwrap();
        let mut vec = vec![0; length];
        for _j in 0..length {
            rng = rng_gen.random_range(a..b+1);
            vec.push(rng.try_into().unwrap());
        }
        array.push(vec);
    }
    array
}
