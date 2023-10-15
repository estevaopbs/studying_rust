// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let tripled_data = data.iter().map(|x| x * 3).collect::<Vec<i32>>();
    let filtered_data = tripled_data
        .iter()
        .filter(|x| x.to_owned() > &10i32)
        .collect::<Vec<&i32>>()
        .iter()
        .map(|x| x.to_owned().to_owned())
        .collect::<Vec<i32>>();
    println!("{:?}", data);
    println!("{:?}", tripled_data);
    println!("{:?}", filtered_data);
    for n in filtered_data {
        println!("{:?}", n)
    }
}
