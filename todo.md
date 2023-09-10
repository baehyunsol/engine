이걸로 보고서 쓸 수 있을 정도로 깔끔하게 만들기
- 사진/표/코드 등등은 페이지 중간에서 잘리지 않게 하기!
- 페이지 바꾸는 매크로 만들기..??

`/templates`, `/mdxts`, `/engine`, `/configs` 이 4개만 옮기면 바로바로 사용가능하도록 하자! -> `/extra_syntaxes`도 옮겨야 할 듯?? 이건 있어도 그만이고 없어도 그만

---

config로 mdxt render_options도 설정하고 싶음...

문서 제목도 metadata로 설정하고 싶음...

---

`--init`이란 인수 받으면 `/configs`, `/templates`, `/mdxts` 다 자동으로 생성하게 할까? 그럼 `/templates` 안의 내용도 전부 자동 생성임? 그 내용들은 어떻게 알아? engine 안에 하드코딩 해놔야해? 하드코딩 해놓으면 앞으로 template 수정할 때마다 engine도 이중으로 수정해야하는데?

이거는 좀 더 안정화가 되고 나서 하자. template들이 거의 바뀔 일이 없겠다 싶을 때 engine 안에 하드코딩으로 넣자.

at least: todo들은 다 없앤 다음에!

쟤네가 하드코딩 돼 있으면 추가적인 이점이 있음.
- engine이 돌다가 `./templates/scss/markdown.tera`란 파일이 필요한데 없다?? 그럼 걍 하드코딩된 데이터에서 뽑아 쓰면 됨!!
