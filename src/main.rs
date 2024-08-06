fn largest<T:std::cmp::PartialOrd>(list:&[T])->&T{
    let mut largest=&list[0];
    for item in list{
        if item>largest{
            largest=item;
        }
    }
    largest
}

fn main(){
    let number_list=vec![2,3,4,6];
    let result=largest(&number_list);
    println!("The larget number is {result}");
    let char_list=vec!['a','b','c','d'];
    let result=largest(&char_list);
    println!("The larget number is {result}");
}
