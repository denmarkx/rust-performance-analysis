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

fn randomize_array_set(a : u32, b : u32, c : usize) -> Vec<Vec<u32>> {
    let mut rng;

    // Create c arrays of len c
    let mut array = vec![vec![0; c]; c];

    // Insert into array:
    for i in 0..c {
        for j in 0..c {
            rng = rand::rng().random_range(a..b+1);
            array[i][j] = rng.try_into().unwrap();
            // array.push(rng.try_into().unwrap());
        }
    }
    array
}
