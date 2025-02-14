fn main() {
	// 기본적인 출력
	println!("Hello, Rust!");
	
	// 변수와 데이터 타입
    let x = 10; // 기본적으로 immutable
    let mut y = 20; // mutable 변수
    println!("x: {}, y: {}", x, y);
    y = 30; // 변경 가능
    println!("변경된 y: {}", y);
	
	// 반복문과 조건문
    let number = 7;

    if number < 10 {
        println!("10보다 작음!");
    } else {
        println!("10 이상!");
    }

    for i in 1..5 { // 1부터 4까지 반복
        println!("i: {}", i);
    }

}
