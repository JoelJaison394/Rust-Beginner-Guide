fn main(){

    let numbers = 30..51; 
    //we use .. for specifying the range , exclusive in the upper rage
    for i in 1..11 {
        println!("The number is {}",i);
    }

    //to print the nuumber set 

    for i in numbers {
        println!("The number is {}",i);
    }

    let animals = vec!["Rabbit" , "Dog" , "Cat"];

    //when we are lopping through vectors we need to use iterator
    for i in animals.iter() {
        println!("The animal is {}",i);
    }

    //we can find out the index of the object we can use enumerate method
    for (index, a) in animals.iter().enumerate() {
        println!("The index is {} and  The animal is {}",index ,a);
    }

}