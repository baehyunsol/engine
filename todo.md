이전에는 table 안에 있는 codespan이 색이 안 보이는 문제가 있었지? 그것도 CSS로 처리하자!!

script를 html 안에 넣지말고 외부에 .js 파일로 따로 만들기
- 이미지 확대, 표 접기 등은 따로 빼도 됨!
- 코드 복사는 따로 못 뺌...

전체적으로 refactor 한번 하기... 이 엔진 아주 오래오래 써먹을 계획이기 때문에 잘 짜놔야함!

이걸로 보고서 쓸 수 있을 정도로 깔끔하게 만들기
- 사진/표/코드 등등은 페이지 중간에서 잘리지 않게 하기!
- 페이지 바꾸는 매크로나 vertical 공간 만드는 매크로 만들기..??

`/templates`, `/mdxts`, `/engine.exe`, `/configs` 이 4개만 옮기면 바로바로 사용가능하도록 하자!

footnote에 tooltip 띄우고 싶음... 이건 구현하는데 한참 걸리겠지? 학기 끝나고 하자!

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
- 그럼 tooltip을 mdxt가 아니고 html로 구현해야하는데...

footnote도 비슷하지만 더 쉬운 방식으로 구현하자. 걍 footnote cite의 내용들 tooltip에 다 넣어버리면 되지!

---

모바일 버전은 top bar 구현 다르게 하자. https://www.w3schools.com/howto/howto_js_mobile_navbar.asp 방식으로!

왜냐면 CoqStudy에서 top bar에 `index_by_chapter`랑 `index_by_keyword`를 넣을 거거든.

---

이미지 확대한 거 닫는 버튼을 따로 만들자. 위에 `click 어쩌구저쩌구 close 얼씨구절씨구`는 날려버리고

---

[[big]][[gold]]는 selection이 적용이 안되는데 [[gold]][[big]]은 됨. 반댄가? 쨌든 해보셈

---

`~_abc_~`를 select하면 글자색만 바뀌고 밑줄색은 안바뀜 ㅠㅠ

---

fenced code block::selection

- dark theme: 보이긴 함
- light theme: hover effect랑 selection effect가 색깔이 동일해서 안 보임

---

config로 mdxt render_options도 설정하고 싶음...

---

css랑 js를 html 안에 embed해서 한 파일로 만드는 기능도 추가하고 싶음!

그러려면 쟤네도 `/*<![CDATA[*/`로 감싸야겠네.

---

`--init`이란 인수 받으면 `/configs`, `/templates`, `/mdxts` 다 자동으로 생성하게 할까? 그럼 `/templates` 안의 내용도 전부 자동 생성임? 그 내용들은 어떻게 알아? engine 안에 하드코딩 해놔야해? 하드코딩 해놓으면 앞으로 template 수정할 때마다 engine도 이중으로 수정해야하는데?

이거는 좀 더 안정화가 되고 나서 하자. template들이 거의 바뀔 일이 없겠다 싶을 때 engine 안에 하드코딩으로 넣자.

---

localStorage 써서 theme이랑 horizontal padding 기억시키자!

---

print 할 때는 copy_button 숨기자!

---

fenced code block에서 button 없이 무식하게 복붙하면 `'\n'`이 과하게 많이 들어감

---

Browser-Compatibility에 footnote index 한번 봐봐. 쟤네 sort 못함?

---

github이나 youtube 같은 거 macro로 지원할까? [linus.dev](linus.dev)에 있는 거 같은 github 카드!

emoji도 지원했으면 좋겠음...
- https://www.w3schools.com/charsets/ref_emoji.asp
- https://www.alt-codes.net/flags
- char랑 겹치는 건 빼자!