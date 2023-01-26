# 크롬 VS 파이어폭스

|                     |[[colspan=2]] 크롬  |[[colspan=2]] 파이어폭스  |                 |
| 항목                | CPU    | Memory    | CPU     | Memory        | 결과            |
|--------------------|--------|-----------|----------|---------------|------------------|
| 가계부 [^ggb]       | 58%    | 170 MB    | 68%     | 515 MB         | 크롬 압승        |
| 군것질 마우스 [^ggg] | 13%    | 175 MB    | 15%     | 492 MB         | 크롬 압승        |
| 유튜브 [^ytb]        | 0%    | 326 MB    | 0%      | 680 MB         | 크롬 압승        |
| blog [^blog]        | 32%    | 192 MB    | 25%     | 475 MB        | 크롬 압승        |
| 여러 탭 [^tabs]     | 0%     | 540 MB    | 0%      | 554 MB         | 크롬 근소한 승리  |
| canvas cycle [^cc]  | 10%    | 207 MB    | 28%     | 530 MB        | 크롬 압승        |

[^ggb]: 브라우저가 없는 상태에서 가계부 메인 페이지로 곧바로 접속. 열리는 과정에서 CPU 사용량의 최댓값과 메모리 사용량의 최댓값을 기록

[^ggg]: 가계부의 군것질 항목에서 달력 위에서 커서 천천히 움직이면서 CPU 사용량과 메모리 사용량 관찰

[^ytb]: 유튜브를 켠 뒤 (둘 다 로그인 돼 있음), CPU 사용량이 0%로 떨어질 때까지 기다린 뒤, 메모리 사용량 관찰

[^blog]: 빈 탭이 열려있는 상태에서 CPU 사용량이 0%가 될 때까지 기다린 다음에 [내 블로그](https://baehyunsol.github.io) 접속. 접속 과정에서 CPU와 메모리의 최댓값 관찰

[^tabs]: 순서대로 [w3schools](https://w3schools.com), [crates.io](https://crates.io), [내 블로그](https://baehyunsol.github.io), [네이버](https://www.naver.com), [위키피디아](https://en.wikipedia.org/), [Rust std](https://docs.rs/std) 탭을 띄운 뒤, 모든 탭의 로딩이 완료될 때까지 대기. 그 다음에 CPU 사용량이 0%가 될 때까지 기다렸다가 메모리 사용량 측정

[^cc]: [canvas cycle](http://www.effectgames.com/demos/canvascycle/)에 접속해서 60fps가 되는 걸 확인한 뒤, 조금 기다렸다가 CPU 사용량과 메모리 사용량 관찰

안돼 내 Rust...