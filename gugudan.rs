// fn main() {
//     for y in 1..10 {
//         for x in 1..10 {
//             print!("{:3},",x*y);
//         }
//         println!("");
//     }
// }

fn main() {
    for y in 1..10{
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))   //문자열 벡터 생성 {:3}을 사용하여 문자열의 필드폭을 3글자로 설정해준다. 
            .collect::<Vec<String>>().join(","); //join 메소드를 사용하여 합치는 방식을 이용한다.
        println!("{}", s);
    }
}