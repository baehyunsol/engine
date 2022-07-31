# Spring

### 전체적인 컨셉

1. 웹을 순수 html이 아닌 jsp를 이용해서 작성
  - jsp는 jsx의 java버전이라고 생각하면 될 듯!
    - 아니네 다르네. jsx는 js 안에다가 html 넣는 거고 jsp는 html 안에다가 java 넣는 거고
    - ㅋㅋ 모르겠다 아님 말고
  - 순수 html 안에다가 특수한 tag를 집어 넣을 수 있음.
    - `<% ... %>` 안에다가 자바코드 집어 넣고
    - `<%= ... %>` 안에다가 자바 expression 집어넣고
    - `<%@ ... %>`는 뭐임? annotation 넣는 거임?
    - `${}`의 형태로 expression을 집어넣을 수도 있음.
1. 자바 코드를 이용해서 위에서 만든 jsp를 제어
  - 굳이 스프링 안 써도 되긴 한데, 스프링 안에 유용한 디자인 패턴들이 많은 듯

```jsp
<p>Counting to three:</p>
<% for (int i=1; i<4; i++) { %>
    <p>This number is <%= i %>.</p>
<% } %>
<p>OK.</p>
```
저걸 jsp로 돌리면 아래처럼 나온대! tera나 handlebars랑 비슷한데 좀 더 원시적인 느낌이네.

```
Counting to three:

This number is 1.

This number is 2.

This number is 3.

OK.
```

[[box]]

##### pom.xml

Rust 언어에서 Cargo.toml랑 비슷한 역할인 듯? 자바 버전 지정하고, dependency 추가해주고, 기타 등등

이클립스에서만 쓰는 건지 Maven에서만 쓰는 건지 잘 모르겠음. 쨌든 튜토리얼에 가끔 나오네.

[[/box]]

[[box]]

##### `throws Exception`

쟤는 statement가 아니고 method 정의에 쓰이는 notation임. `throw` keyword가 Python의 `raise`에 대응됨. 원래 자바의 메소드는 에러를 던지면 안된대. 메소드 안에 모든 에러 처리 로직을 포함시켜 놓아야 함. 근데 메소드 정의에 `throws A`라고 붙여 놓으면 `A`라는 에러는 처리할 필요가 없음. `이 메소드는 A라는 에러를 던질 수도 있어요`라고 선언하는 거니까 에러 처리를 바깥의 메소드한테 짬 때리는 거임.

`throws Exception`은 귀차니즘 끝판왕임. `Exception`이 모든 에러의 parent class인가봐. 즉, 이 메소드는 뭐가 됐든 에러를 던질 수 있고, 그거 처리는 알아서 하라고 선언하는 거지. Bad practice네.

[[/box]]

[[box]]

##### Annotations

Rust에서 `[]`로 쓰는 매크로랑 비슷한 느낌인 듯?

[[/box]]

[[box]]

##### POJO

Plain, old java object. 그냥 vanila java로 짠 object.

[[/box]]

[[box]]

##### Beans

- Spring에선 pojo를 Bean이라고 부름.
- Bean들을 전부 XML에 정의를 해둠.
- Spring에서 `getBean`이라는 메소드를 통해서 Bean에 접근 가능
- Bean은 singleton일 수도 있고, prototype일 수도 있고, 다른 것도 있고...
  - singleton은 `static mut`
  - prototype은 `getBean`할 때마다 새로 생성

의존성 주입이 이건 듯. 클래스를 직접 만들어서 직접 참조하는게 아니고, Bean을 통해 만들고 Bean을 통해 참조하는 방식...
- Interface는 코드 안에 있고, 실제 구현은 Bean으로 참조하는 거인 듯
- 굳이 왜 그렇게 하는 거임?

[[/box]]

[[box]]

##### MVC Pattern

자주 보게되는 용어...

- Model
  - the application's dynamic data structure, independent of the user interface
- View
  - the representation of information
- Controller
  - it responds to the user input and performs interaction on the data model objects

[[/box]]

## Spring MVC

### RequestMapping

```java
import java.util.Locale;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;

@Controller
@RequestMapping(value="/board/*")
public class BoardController {

	@RequestMapping(value = "/list", method = RequestMethod.GET)
	public void list(Locale locale, Model model) {
		//
	}

}
```

- `@RequestMapping`
  - http request가 들어오면 저 메소드랑 연결하라는 뜻인 듯??
  - 근데 왜 return type이 `void`임...?
  - 보니까 `@Controller`대신 `@ResponseBody` 붙어있는 친구도 있음. 걔는 return type이 `String`이네.
  - 아 이건 걍 코드를 덜 짜서 `void`로 돼 있는 듯??
- `@Controller`
  - MVC의 C임.
  - Bean 객체가 돼서 serverlet의 container에 자동으로 들어감. 그럼 다른 스프링 코드에서 Bean을 참조할 수 있음.
  - 블로그 글에는 http request/response를 처리한다고 나와 있음

`/board/list`로 get request를 날리면 `list` 메소드가 호출되는 듯!

### Repository, Service

```java
package com.board.dao;

import java.util.List;
import javax.inject.Inject;  // standard java extension
import org.apache.ibatis.session.SqlSession;
import org.springframework.stereotype.Repository;

@Repository
public class BoardDAOImpl implements BoardDAO {

	@Inject
	private SqlSession sqlSession;

	private static String namespace = "com.board.mappers.board";

	@Override
	public List list() throws Exception {
		return sqlSession.selectList(namespace + ".list");
	}

}
```

굳이 `BoardDAO`라는 인터페이스를 만들고 `BoardDAOImpl`이라는 클래스로 그걸 구현함.

- DAO
  - Data Access Object
  - DB와 연결된 객체. CRUD를 얘가 함.
- DTO
  - 위의 코드에는 안 나오는데 블로그에 DTO도 나오더라
  - DB 안에 데이터가 있지? `name: String`, `age: Number` 이런 식으로. 걔네랑 동일한 이름/타입의 필드를 가지는 클래스
  - 보통 DB에서 읽은 값을 자바에서 사용하려고 만듦. 보통 로직은 없음
- `@Repository`
  - 해당 클래스를 root container에 Bean 객체로 생성.
  - DB나 파일같은 외부 IO 담당
- `@Service`
  - 해당 클래스를 root container에 Bean 객체로 생성.
  - 내부 로직을 담당
- `@Inject`
  - 의존성 주입이랑 관련된 건데 뭔지 모르겠음...
  - `getBean`을 간편하게 하는 건가??

저 코드는 DB에서 list를 읽어오는 쿼리를 날리는 함수임!

### 중간 정리

```java
package com.board.service;

import java.util.List;
import javax.inject.Inject;
import org.springframework.stereotype.Service;
import com.board.dao.BoardDAO;
import com.board.domain.BoardDTO;

@Service
public class BoardServiceImpl implements BoardService {

	@Inject
	private BoardDAO dao;

	@Override
	public List list() throws Exception {
		return dao.list();
	}
	
	@Override
	public int regi(BoardDTO dto) throws Exception {
		
		if (dao.getMaxSeq() == null) { // 게시글이 존재하지 않을 때
			dto.setSeq(1); // SEQ는 1
		} else { // 게시글이 존재할 때
			dto.setSeq(dao.getMaxSeq() + 1); // 최대값에 +1
		}
		
		return dao.regi(dto);
	}

}
```

지금까지 배운 거 가지고 저 코드 정리.

1. `BoardServiceImpl`은 `BoardDAO`라는 클래스에 의존함. 근데 그 의존성을 자바 코드 안에 집어 넣는게 아니고 `@Inject`로 해결
1. `dao`는 DB와 직접 소통하는 객체고, `dto`는 게시글임.
1. 얜 `@Service`이니까 내부 로직을 담당함.
  - `BoardService.regi`에서는 게시글에 번호를 부여하고
  - `dao.regi(dto)`에서는 게시글을 DB에 등록함.

```java
package com.board.controller;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.List;
import java.util.Locale;

import javax.inject.Inject;
import javax.servlet.http.HttpServletRequest;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.ResponseBody;

import com.board.domain.BoardDTO;
import com.board.service.BoardService;

@Controller
@RequestMapping(value="/board/*")
public class BoardController {
	
	 @Inject
	 private BoardService service;
	
	@RequestMapping(value = "/list", method = RequestMethod.GET)
	public String list(Locale locale, Model model) throws Exception {
		  List list = service.list();
		  model.addAttribute("list", list);
		  
		  return "/board/list";
	}
	
	@RequestMapping(value = "/regiView", method = RequestMethod.GET)
	public String regiView(Locale locale, Model model) throws Exception {
		return "/board/regi";
	}
	
	@ResponseBody
	@RequestMapping(value = "/regi", method = RequestMethod.POST)
	public  String regi(Locale locale, Model model, BoardDTO dto) throws Exception {
		
		Date date = new Date(System.currentTimeMillis());
		SimpleDateFormat format = new SimpleDateFormat("yyyyMMddHHmmss"); 
		
		dto.setReg_date(format.format(date));
		
		if(service.regi(dto) == 1) {
			return "Y";
		}else {
			return "N";
		}
	}
	
	@RequestMapping(value = "/view", method = RequestMethod.POST)
	public String view(Locale locale, Model model, HttpServletRequest request) throws Exception {
		BoardDTO dto = service.view(Integer.parseInt((String)request.getParameter("seq")));
		model.addAttribute("view", dto);
		return "/board/view";
	}
}
```

오잉 `RequestMapping`이 http body를 return하는 거 아녔음? 왜 모양이 저렇지... 그냥 라우팅만 하는 건가?

블로그에 있는 jQuery AJAX 코드를 보면 게시글을 등록할 때 `/board/regi`로 POST를 날림. 그럼 서버가 `Y`나 `N`을 보내겠지? 그래서 AJAX 코드 안에 `Y`면 게시글을 등록하고 아니면 하지 말라고 하는 코드가 있음.