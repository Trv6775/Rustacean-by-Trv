fn largest(list:&[i32])->&i32{
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
    let number_list=vec![2000,3,400,60];
    let result=largest(&number_list);
    println!("The larget number is {result}");
}