fn main() {
    for i in 1..101 { //1부터 100까지 반복문
        if i % 3 == 0 && i % 5 == 0 {   //3,5 공배수 라면 'FizzBizz'를 출력한다.
            println!("FizzBizz");
        }
        else if i % 3 == 0{ //3의 배수라면 'Fizz'를 출력한다.
            println!("Fizz");
        }
        else if i % 5 == 0 { //5의 배수라면 'Bizz'를 출력한다.
            println!("Bizz");
        }
        else{
            println!("{}",i);
        }
    }
}