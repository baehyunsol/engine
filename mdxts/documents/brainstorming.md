[[toc]]

# Modules

## `main.v`

- `compute_top`과 `interface_top`만 덩그러니 있음
- input으로는 uart와 clk, reset이 있고
- output으로는 uart와, ddr3(와 관련된 아주 많은 것들)가 있음!

## `compute_top.v`

- `fc_top`, `conv_top`, `pool_top` 3개만 있음. 끝!
- 각 module이 APB/AXI를 통해서 외부와 소통하지? 그 wire들이 `compute_top`의 input/output으로 전부 꽂혀있음

## `interface_top.v`

- input output이 무지하게 많음
  - uart도 여기에 direct로 들어오고
  - ddr3와 관련된 output/inout도 엄청 많고
  - vdma도 3종류(conv, fc, pool)가 있고
  - apd도 3종류(conv, fc, pool)가 있음

## `*_module`

디렉토리가 3개가 있음: `conv_module`, `pool_module`, `fc_module`. 셋이 내용이 거의 비슷함. 그니까 여기선 걍 conv만 보자

- `conv_top`
  - `conv_apb`, `conv_module`, `clk_counter`만 연결돼 있음! 저 모듈들의 port를 수정한게 아닌 이상 딱히 수정할 부분이 없음
- `clk_counter`
  - Conv 계산하는데 몇 cycle 걸렸는지 세는 module! 수정하지 말래
- `conv_module.v`
  - ports
    - 얘가 master일 때와 slave일 때의 AXIS wire들이 쭈루룩 있음
      - data 보낼 땐 master고 받을 땐 slave임
    - clk, reset을 받음
    - conv_start를 받고 conv_done을 보냄
  - `conv_top`한테 `conv_start`를 받으면 열심히 작업을 한 다음에 결과를 AXIS로 보내고, `conv_top`한테 `conv_done`을 보냄
  - 족보 분석
    - `command`라는 2bit짜리 input을 받음. 얘는 `conv_apb`한테 받음. 아마 Python이 준 거겠지?
      - 첫번째 command: feature를 AXIS로 받아서 DRAM에 저장
      - 두번째 command: bias를 AXIS로 받아서 DRAM에 저장
      - 세번째 command: weight를 AXIS로 받아서 DRAM에 저장하고 conv를 계산
    - feature DRAM과 bias DRAM, weight DRAM이 각각 따로 있음. 전부 ip를 사용하는 듯?
    - AXIS로 입력을 받는 거랑 DRAM에 저장하는 거랑 전부 과정이 아주 길기 때문에 FSM을 이용해서 control함. 그래서 코드가 긺
    - 계산은 여기서 안함. `mac3x3` module을 따로 만들어두고 걔한테 모든 계산을 시킴
      - 계산도 여러 cycle 동안 하기 때문에 그거 관리하는 커다란 FSM이 있음
- `conv_apb.v`
  - Python이 보낸 신호가 얘를 거쳐서 `conv_module`한테 감. 즉, Python이 계산 시작하라고 시키면 얘가 `conv_module`한테 `conv_start` 보내고, `conv_module`이 `conv_done` 보내면 얘가 Python한테 apb로 알려주는 듯?
  - `PWRITE`가 들어오면 `PWDATA`를 읽어서 내부 reg에 값을 씀
  - `PWRITE`가 안 들어오면 `prdata_reg`에 값을 씀
    - 무슨 값을 쓸지는 `PADDR`을 보고 결정
    - `prdata_reg`는 `PRDATA`라는 output에 연결돼 있고 쟤는 `interface_top`을 통해서 Python과 연결돼 있음

# AXIS

- READY
  - slave가 data 받을 준비 되면 READY를 1로 띄움
- VALID
  - master가 data 보낼 준비 되면 VALID를 1로 띄움
  - 띄우자마자 DATA도 같이 보냄
  - DATA가 가는 동안 계속 1
- DATA
  - master가 보내는 actual data
  - 매 clock마다 DATA_WIDTH만큼의 data를 보냄. default 값은 32bit로 돼 있네
- LAST
  - master가 DATA를 보내다가 마지막 DATA가 갈 때 LAST를 1로 띄움
  - PPT 보면 LAST가 1 되고 그 다음 clock에 VALID가 0됨
- KEEP
  - 이게 1이어야지만 같이 가는 DATA가 qualified인가봐... 잘 모르겠음 ㅎㅎ
  - PPT에서는 `4'b1111`로 고정이라는데?
- USER
  - 추가로 보내고 싶은 정보가 있으면 이거로 보내래
  - PPT에서는 `1'b0`으로 고정이라는데?

# APB

Protocol 자체는 단순한 듯? addr, rdata, wdata 등등 있고, addr의 값을 읽거나 addr에 wdata를 쓰거나... 아주 간단함!

원래대로면 `conv_apb`를 구현할 때 거대한 array를 만들고 i번 addr을 읽으려고 하면 `array[i]`를 보내는게 정석이지? 근데 족보는 그렇게 안함. command, conv_done, conv_start 등등의 reg를 전부 따로 만들어 두고, command의 addr이 들어오면 prdata에 command를 쓰고 conv_done의 addr이 들어오면 prdata에 conv_done을 쓰는 방식으로 구현돼 있음. skeleton 준 거 보면 그런 식으로 짜는게 더 편할 듯?

어느 주소에 어느 값을 mapping할지는 우리 마음임. 그래서 Python code 수정해야함. (ex: Python이 FPGA한테 3이라는 command를 쓰고 싶다? 근데 command가 6번 주소에 있다? 저 주소가 6인 건 내가 정한 거여서 Python에도 반영해줘야함)

Python이 control register로 FPGA에 명령 내리라는게 이 얘긴가??

# 기타

`` `ifndef ``같은 것도 나옴 ㅋㅋ