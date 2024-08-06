fn main(){
    let number_list=vec![34,50,25,100,65];
    let mut largest_number=&number_list[0];
    for number in &number_list{
        if number>largest_number{
            largest_number=number;
        }
    }
    println!("The largest number is {largest_number}");
}