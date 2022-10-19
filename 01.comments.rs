fn main() {
    // This is an example of a line comment
    
    /* 
     * This is another type of comment, a block comment. In general,
     */

    /*
       This is another type of comment
       Rust는 들여쓰기로 스페이스*4개를 사용한다.(Tab이 아니다!)
       println! 은 rust의 매크로를 호출한다. 만약 매크로가 아니라 함수를 호출하려면 println 이라고만 써줘야한다. 
       매크로에 대해서는 좀 더 배운 다음에 알아볼 것이다. 
       지금 우리는! 를 쓰는 것이 함수가 아닌 매크로를 쓰겠다는 의미라는 것만 알아두어도 좋다.
       println 은 문자열을 파라미터로 받아 화면에 출력해주었다.
       코드의 라인은 ; 을 이용해 끈맺었다. 이렇게 해줌으로 다음 라인과 구별 가능하게 해준다.
       컴파일과 실행은 분리돼있다
       Rust 프로그램이 실행되기 전에 rustc 명령어를 이용해 이를 꼭 컴파일해줘야한다.

       $ rustc main.rs
       
       위에 보이는 것이 컴파일 요청 명령어다. 컴파일이 완료됐다면 Rust는 실행파일을 반환한다. 
       위 명령어를 실행했던 폴더에 어떤 파일이 생겼는지 확인해보자.

       $ ls
       >> main main.rs
       
       우리가 만든 파일은 main.rs 밖에 없었는데 main 이라는 파일이 새로 생겼다. 
       확장자가 없는 main 은 실행 가능한 파일로 다음과 같이 파일명을 입력하면 프로그램이 실행된다.
       (원도우에서는 main.ex)

       $ ./main
       >> Hello, world!

       간단한 프로그램이라면 rustc 컴파일하는 것만으로 충분하다. 하지만 사이즈가 커지면 환경설정이나 
       프로세스를 조금 더 단순하게 만들 필요가 있다.
       이를 위해 Cargo라는 도구를 사용할 것이다!
    */

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
