script를 html 안에 넣지말고 외부에 .js 파일로 따로 만들기
- 이미지 확대, 표 접기 등은 따로 빼도 됨!
- 코드 복사는 따로 못 뺌...

전체적으로 refactor 한번 하기... 이 엔진 아주 오래오래 써먹을 계획이기 때문에 잘 짜놔야함!

이걸로 보고서 쓸 수 있을 정도로 깔끔하게 만들기
- 사진/표/코드 등등은 페이지 중간에서 잘리지 않게 하기!
- 페이지 바꾸는 매크로 만들기..??

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
- 이미지 열화
  - 열화된 거 먼저 보내고 나중에 제대로 된 거 보내기

---

미리보기 tooltip. Article끼리 link를 걸면, link의 hover로 article 미리보기 tooltip을 띄우자! 이거 HXML로 할 수 있음. 해당 link가 inner link인지 보고, 그럼 해당 article의 html을 읽어서 `<body>`의 가장 앞부분 내용만 (태그들 다 벗겨서) 긁어와서 tooltip으로 만들면 될 듯!
- 그럼 tooltip을 mdxt가 아니고 html로 구현해야하는데...

footnote도 비슷하지만 더 쉬운 방식으로 구현하자. 걍 footnote cite의 내용들 tooltip에 다 넣어버리면 되지!

---

이미지 확대한 거 닫는 버튼을 따로 만들자. 위에 `click 어쩌구저쩌구 close 얼씨구절씨구`는 날려버리고

---

```
[[big]][[gold]]big [[tiny]][[red]]tiny red[[/red]][[/tiny]] gold[[/gold]][[/big]]

[[gold]][[big]]gold [[red]][[tiny]]red tiny[[/tiny]][[/red]] big[[/big]][[/gold]]

[[big]][[red]]big [[tiny]][[gold]]tiny gold[[/gold]][[/tiny]] red[[/red]][[/big]]

[[red]][[big]]red [[gold]][[tiny]]gold tiny[[/tiny]][[/gold]] big[[/big]][[/red]]
```

selection 해서 색깔 제대로 나오는지 확인하기! 저러면 안쪽의 selection은 gold::selection랑 red::selection이 충돌하잖아? 그럼 항상 gold가 이기더라... css 파일에 정의된 순서대로인 듯! 둘중에 더 가까운 거 적용되도록 하려면 어떻게 해야하지...

---

`~_abc_~`를 select하면 글자색만 바뀌고 밑줄색은 안바뀜 ㅠㅠ
- chrome에선 제대로 동작하고 firefox에선 안 됨...
- text-decoration-color는 두 browser에서 둘다 영향 X
  - chrome은 그냥 color 기준으로 underline도 하는 듯??
  - border-bottom은 절대 쓰면 안됨!! text가 여러 줄이면 underline이 이상하게 될 거 아녀
- 나중에 또 고치면 하는 김에 `[[gold]]~_abc_~[[/gold]]`도 해보셈

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

at least: todo들은 다 없앤 다음에!

---

localStorage 써서 theme이랑 horizontal padding 기억시키자!

근데 이러면 config에서 theme이랑 horizontal padding 바꿔도 적용이 안되는데??

---

fenced code block에서 button 없이 무식하게 복붙하면 `'\n'`이 과하게 많이 들어감

---

`<script>`에 `async`나 `defer` 넣고 싶은데 HXML이 저 문법을 허용을 안함

큰 dilema긴 함... 저건 대놓고 XML이 아닌데? 근데 필요하긴 함...

---

Bottlenecks

폰트 loading하는 거: https://stackoverflow.com/questions/40624515/load-google-font-with-link-asynchronously-or-deferred-without-font-face-observ

image loading하는 거: 일단 빈 image를 넣고 js로 src를 고쳐버릴까??
- 그럼 page render를 여러번 해서 손해 아님??
  - ㄴㄴ 이렇게 해도 노상관인게: CPU 도는 속도가 네트워크로 파일전송하는 속도보다 압도적으로 빠름.
- 아니면 네이버처럼 일단은 저화질 이미지로 넣고 그다음에 본 이미지를 넣을까?
  - 엔진이 저화질 이미지 일일이 만들어야함...

https://pagespeed.web.dev/ <- 괜찮네

---

javascript strict mode 추가하기!

---

`[[gold]]D1[[/gold]]이 [[red]]D3[[/red]]보다 크지? 아까 말한 queue 때문에 그래.`를 select하면 색깔 다른 부분이 크기도 달라짐... 왜 그럴까
- chrome에선 안 그러고 firefox에서만 그럼... CSS 고친다고 해결될 문제가 아닌 듯?
- 로마자랑 한글이 원래 크기가 다른가 싶었는데 그것도 아님,,, 색깔 macro 없애니까 크기 동일함
- 이건 그냥 놔둘까?

---

zola에 보면 extra syntax highlighting 있음 Coq 좀 추가하셈 제발

https://packagecontrol.io/packages/Coq
https://github.com/whitequark/Sublime-Coq

쟤네 참고 ㄱㄱ

---

syntect 문서 잘 뒤져보면 oniguruma 대신 fancy-regex 쓰는 법 나와있음. 웬만해선 pure rust가 나으니까 저거로 갈아타자! 해보고 performance 차이가 너무 심하다 싶으면 다시 돌아오고 그게 아니면 계속 유지 ㄱㄱ

아니 근데 fancy-regex 최신 버전이 0.10.0인데 syntect는 0.7.0 쓰는데??? 이 쉐키들 관리 안하네...

---

css modularization: 안 쓰는 CSS는 굳이 import 하지 말자

각 html 파일별로 css를 따로 만들자: ex: A.html은 table만 쓰니까 그것만 들어간 A.css 제작
- 근데 A.html과 B.html이 동일한 css를 쓴다? 그럼 굳이 별개로 만들지 말고 둘이 같은 거 쓰게 하셈...
- 어떻게 하지?? 필요한 css의 기능이 table, blockquote, codespan이라고 하자? 그럼 `'table-blockquote-codespan'`이라는 문자열을 hash를 해. 걔가 ABCD라고 하자? 그럼 `ABCD.css`를 import하면 됨. `ABCD.css`가 존재하는지 아닌지는 engine이 관리하는 거고

만약에 js DOM으로 활성화되는 css면? ex: html만 보면은 `A`라는 class를 절대 못 찾음. 근데 js가 DOM을 이용해서 `A`라는 class를 만듦. 저런 css 날려버리면 안되잖아...

variables
- 이 페이지 전체에 `--yellow`라는 변수가 안 쓰임. 그럼 `--yellow`라는 변수 날려버릴 거임? 저거 날렸는데 js DOM으로 `--yellow` 조작하면 어떻게 됨? 에러임?
  - 저래도 error는 아닌 듯? 저렇게 하면 `--yellow`라는 변수가 새로 생김
  - 걍 js DOM에 있는 `--yellow`까지 날려버리는게 best기는 함. 근데 그러려면 js까지 이해하는 engine을 만들어야함...ㅠㅠ

---

cache system

scss 만드는 거나 md->html 하는 거나 css modularization 하는 거나 수정사항 없으면 걍 기존 거 사용하면 안됨??

아 근데 이게 구현이 복잡할텐데... cache + garbage collection하려면...

---

box에 attribute 추가했으면 reference.md도 고치셈

하는 김에 icon도 reference에 추가!

---

파일 관련된 API들 전부 case-insensitive하게 바꾸셈! `.CSS`든 `.cSs`든 다 작동하도록

---

settings menu에서 글씨 크기도 조절 가능하게 하자! 이러면 `rem`에 영향받는 모든 친구들이 다 바뀔 듯?

---

fenced code block을 light/dark 선택 가능하게 할까?

---

md가 여러개면 parallel하게 render할까?

---

print할 때 header를 숨길까?