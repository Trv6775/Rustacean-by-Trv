fn largest(list:&[i32])->&i32{
    let mut largest=&list[0];
    for item in list{
        if item>largest{
            largest=item;
        }
    }
    largest
}
fn largest_char(c:&[char])->&char{
    let mut largest=&c[0];
    for char in c{
        if char>largest{
            largest=char;
        }
    }
    largest
}
fn main(){
    let number_list=vec![2,3,4,6];
    let result=largest(&number_list);
    println!("The larget number is {result}");
    let char_list=vec!['a','b','c','d'];
    let result=largest_char(&char_list);
    println!("The larget number is {result}");
}
