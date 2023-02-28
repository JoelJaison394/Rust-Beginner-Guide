fn main(){
    let mut n = 0;

    loop{
        n +=1;

        if n > 10 {
            break;
        }

        if n == 7 {
            continue;
        }
        println!("The value of n is {}", n);
    }
}