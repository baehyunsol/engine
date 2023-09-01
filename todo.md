script를 html 안에 넣지말고 외부에 .js 파일로 따로 만들기
- 이미지 확대, 표 접기 등은 따로 빼도 됨!
- 코드 복사는 따로 못 뺌...

전체적으로 refactor 한번 하기... 이 엔진 아주 오래오래 써먹을 계획이기 때문에 잘 짜놔야함!

이걸로 보고서 쓸 수 있을 정도로 깔끔하게 만들기
- 사진/표/코드 등등은 페이지 중간에서 잘리지 않게 하기!
- 페이지 바꾸는 매크로 만들기..??

`/templates`, `/mdxts`, `/engine.exe`, `/configs` 이 4개만 옮기면 바로바로 사용가능하도록 하자! -> `/extra_syntaxes`도 옮겨야 할 듯?? 이건 있어도 그만이고 없어도 그만

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

---

버그: 이름이 동일한 파일이 여러개 있으면 (ex: documents에도 index.md가 있고 articles에도 index.md가 있음) engine이 둘을 구분을 못함

---

```
[[big]][[gold]]big [[tiny]][[red]]tiny red[[/red]][[/tiny]] gold[[/gold]][[/big]]

[[gold]][[big]]gold [[red]][[tiny]]red tiny[[/tiny]][[/red]] big[[/big]][[/gold]]

[[big]][[red]]big [[tiny]][[gold]]tiny gold[[/gold]][[/tiny]] red[[/red]][[/big]]

[[red]][[big]]red [[gold]][[tiny]]gold tiny[[/tiny]][[/gold]] big[[/big]][[/red]]
```

selection 해서 색깔 제대로 나오는지 확인하기! 저러면 안쪽의 selection은 gold::selection랑 red::selection이 충돌하잖아? 그럼 항상 gold가 이기더라... css 파일에 정의된 순서대로인 듯! 둘중에 더 가까운 거 적용되도록 하려면 어떻게 해야하지...

아이디어 1:
- `.color-gold { .color-red { color: red; } }` 이런 식으로 모든 경우의 수를 다 넣는 거임!
- 저거 만드는 거야 tera가 만드니까 상관없고,
- CSS 최적화 도구 있으면 필요한 것만 남길테니까 상관없음!
  - 이거 만드는게 우선이네

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

문서 제목도 metadata로 설정하고 싶음...

---

css랑 js를 html 안에 embed해서 한 파일로 만드는 기능도 추가하고 싶음!

그러려면 쟤네도 `/*<![CDATA[*/`로 감싸야겠네.

---

`--init`이란 인수 받으면 `/configs`, `/templates`, `/mdxts` 다 자동으로 생성하게 할까? 그럼 `/templates` 안의 내용도 전부 자동 생성임? 그 내용들은 어떻게 알아? engine 안에 하드코딩 해놔야해? 하드코딩 해놓으면 앞으로 template 수정할 때마다 engine도 이중으로 수정해야하는데?

이거는 좀 더 안정화가 되고 나서 하자. template들이 거의 바뀔 일이 없겠다 싶을 때 engine 안에 하드코딩으로 넣자.

at least: todo들은 다 없앤 다음에!

쟤네가 하드코딩 돼 있으면 추가적인 이점이 있음.
- engine이 돌다가 `./templates/scss/markdown.tera`란 파일이 필요한데 없다?? 그럼 걍 하드코딩된 데이터에서 뽑아 쓰면 됨!!

---

`<script>`에 `async`나 `defer` 넣고 싶은데 HXML이 저 문법을 허용을 안함

큰 dilema긴 함... 저건 대놓고 XML이 아닌데? 근데 필요하긴 함...

이제 더 큰 문제!! video나 audio 태그에서 대놓고 HXML이랑 충돌함!

---

`[[gold]]D1[[/gold]]이 [[red]]D3[[/red]]보다 크지? 아까 말한 queue 때문에 그래.`를 select하면 색깔 다른 부분이 크기도 달라짐... 왜 그럴까
- chrome에선 안 그러고 firefox에서만 그럼... CSS 고친다고 해결될 문제가 아닌 듯?
- 로마자랑 한글이 원래 크기가 다른가 싶었는데 그것도 아님,,, 색깔 macro 없애니까 크기 동일함
- 이건 그냥 놔둘까?

---

syntect 문서 잘 뒤져보면 oniguruma 대신 fancy-regex 쓰는 법 나와있음. 웬만해선 pure rust가 나으니까 저거로 갈아타자! 해보고 performance 차이가 너무 심하다 싶으면 다시 돌아오고 그게 아니면 계속 유지 ㄱㄱ

아니 근데 fancy-regex 최신 버전이 0.10.0인데 syntect는 0.7.0 쓰는데??? syntect가 최신 버전으로 갈아탈 때까지 일단 존버...

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

좀 더 coarse하게 할까? css 파일을 여러개 만드는 거임! code_fence.css, table.css처럼..!! 이러면 파일 여러개 로드하느라 오히려 오래 걸리려나? 근데 css를 html 안에 embed하는 기능을 쓴다고 치면 이게 더 효율적일 수도 있음!

---

cache system

scss 만드는 거나 md->html 하는 거나 css modularization 하는 거나 수정사항 없으면 걍 기존 거 사용하면 안됨??

아 근데 이게 구현이 복잡할텐데... cache + garbage collection하려면...

---

fenced code block을 light/dark 선택 가능하게 할까?

---

어떤 code-fence의 X번 줄에 link 걸기. 그거 클릭하면 그 줄이 highlight되는 거임! 구현은 무지 쉬움. 해당 줄의 span tag에다가 `highlight`라는 class만 toggle 해주면 됨!!

이거 하려면 각 code fence에도 이름을 붙여야 함... 쉬움?

---

병렬: 구현은 했는데 XML이 병렬화가 안 돼서 별 의미 X -> 빨랑 thread-safe하게 ㄱㄱ

tera는 병렬로 돌려도 별 차이가 안 남... 막아놨나?

---

table 안의 code span은 background color를 다르게 처리하잖아? 근데 또 table 안의 tooltip 안의 code span은 한번 더 꼬아야함...

---

reference들을 전부 날려버리니까 `#mdxt` 태그 페이지가 죽음... 레퍼런스 보다가 저기로 가려고 하면 바로 죽음!

---

table -> 항목별로 정렬하는 기능 넣기
