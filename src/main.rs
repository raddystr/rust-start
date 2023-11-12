fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let values = [8, 30, 1, 3];
    let mut sum = 0;
    let mut slice_sum = 0; 
    let vec_sum = 0;
    let mut values_vec = vec![8, 30];
    
    values_vec.push(10);
    values_vec.push(11);



    for n in 0..4 {
        sum = add(sum, values[n]); 
    }

    for n in &values[2..4] {
        slice_sum = add(slice_sum, *n);
    }
    
    for n in values_vec {
        sum = add(vec_sum, n); 
    }





    println!("sum = {}", sum);
    println!("extra_sum = {}", slice_sum); 
    println!("vec_sum = {}", vec_sum); 

}
