이전에는 table 안에 있는 codespan이 색이 안 보이는 문제가 있었지? 그것도 CSS로 처리하자!!

script를 html 안에 넣지말고 외부에 .js 파일로 따로 만들기
- 이미지 확대, 표 접기 등은 따로 빼도 됨!
- 코드 복사는 따로 못 뺌...

전체적으로 refactor 한번 하기... 이 엔진 아주 오래오래 써먹을 계획이기 때문에 잘 짜놔야함!

이걸로 보고서 쓸 수 있을 정도로 깔끔하게 만들기
- 사진/표/코드 등등은 페이지 중간에서 잘리지 않게 하기!
- 페이지 바꾸는 매크로나 vertical 공간 만드는 매크로 만들기..??

`/templates`, `/mdxts`, `/engine.exe`, `/configs` 이 4개만 옮기면 바로바로 사용가능하도록 하자!

multiline table header는 행간 구분선 넣자!

footnote에 tooltip 띄우고 싶음... 이건 구현하는데 한참 걸리겠지? 학기 끝나고 하자!

js랑 C에서 `//`로 쓰는 주석들 끝이 제대로 안 남. 분명히 줄바꿈을 했는데 계속 글씨가 회색임.

---

```markdown
[[box]]
교수님의 질문: 내 폰이랑 공유기랑 통신하고 니네 폰이랑 공유기랑도 통신하지? 그럼 니네 폰에서 공유기로 가는 packet을 내 폰에서도 볼 수 있을까?
- yes.
- 근데 header 확인해보고 나랑 관련없는 packet은 걍 버림. 그래서 내 폰에서 공유기로 가는 정보를 다른 폰에서 못 보는 거임.
  - 엥? 근데 이럼 보안 구멍 아님? 내 폰에서 쓰는 데이터를 친구 폰에서 볼 수 있는 거잖아?
  - ㅇㅇ 그래서 application layer에서 암호화를 해버림. 그럼 중간에 가로채도 알 방법이 없음.
    - 왜 application layer냐? 민감한 정보는 다 저기 있을 거 아녀? 다른 layer에는 공유기 ip 주소같은 안 민감한 정보만 있을 거잖아.
[[/box]]
```

버그 찾았음!

---

```markdown
[[box]]
a
[[/box]]
b
[[box]]
c
[[/box]]
```

이거 지금처럼 되는게 맞아?

---

지금 `render` 구현을 보면 `remove_results`를 가장 먼저 하고 시작하잖아? 저걸 조금 늦게할 순 없나? 그럼 파일이 없는 시간이 훨씬 줄어들텐데.

---

image processing도 추가하고 싶음.

- 크기 조절
  - 크기 자체는 이미 충분히 조절이 가능하지만 용량을 줄이고 싶을 수도 있으니까!
- 흑백 반전
  - 필기할 때 요긴할 듯

---

미리보기 tooltip. Article끼리 link를 걸면, link의 hover로 article 미리보기 tooltip을 띄우자! 이거 HXML로 할 수 있음. 해당 link가 inner link인지 보고, 그럼 해당 article의 html을 읽어서 `<body>`의 가장 앞부분 내용만 (태그들 다 벗겨서) 긁어와서 tooltip으로 만들면 될 듯!

footnote도 비슷하지만 더 쉬운 방식으로 구현하자. 걍 footnote cite의 내용들 tooltip에 다 넣어버리면 되지!

---

모바일 버전은 top bar 구현 다르게 하자. https://www.w3schools.com/howto/howto_js_mobile_navbar.asp 방식으로!

왜냐면 CoqStudy에서 top bar에 `index_by_chapter`랑 `index_by_keyword`를 넣을 거거든.

---

이미지 확대한 거 닫는 버튼을 따로 만들자. 위에 `click 어쩌구저쩌구 close 얼씨구절씨구`는 날려버리고