script를 html 안에 넣지말고 외부에 .js 파일로 따로 만들기
- 이미지 확대, 표 접기 등은 따로 빼도 됨!
- 코드 복사는 따로 못 뺌...

전체적으로 refactor 한번 하기... 이 엔진 아주 오래오래 써먹을 계획이기 때문에 잘 짜놔야함!

이걸로 보고서 쓸 수 있을 정도로 깔끔하게 만들기
- 사진/표/코드 등등은 페이지 중간에서 잘리지 않게 하기!
- 페이지 바꾸는 매크로 만들기..??

`/templates`, `/mdxts`, `/engine`, `/configs` 이 4개만 옮기면 바로바로 사용가능하도록 하자! -> `/extra_syntaxes`도 옮겨야 할 듯?? 이건 있어도 그만이고 없어도 그만

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

아무 것도 없는 상태로 실행해보기! -> mdxts 폴더 없이 어떻게 되는지 보자!
-> 일단 목표는 git clone한 다음에 바로 실행 가능한 상태로 만드는 거임
