fn main(){
    let mut x = 45;
    // By default we cannot re assign or change the value in variables decleared using let , so inorder to overcome the such problem we can use a keyword called mut means mutable , so we can change the value inside  the particularvariables
    println!("The value of the x is {}" , x);
    x=60;
    println!("The value of the x is {}" , x);
}