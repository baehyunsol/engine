---
date: [2022, 8, 14]
tags: [loc, opensource, comparison]
preview: "I counted loc(lines of code) of some repositories, just for fun!"
---

# LOC

I counted loc(lines of code) of some open-source repositories, just for fun!

Here is how it works.

1. `git clone --depth=1 https://github.com/rust-lang/rust`
  - First, clone the repository that I want.
  - The `--depth` option prevents downloading the history. It saves **a lot** of time and space.
1. `tokei rust`
  - I used the awesome [tokei] crate to count the lines. It's astonishingly fast.
  - This command will print the output.
1. `rm -r rust`
  - Clean the disk.

[tokei]: https://github.com/XAMPPRocky/tokei

[[box]]
[[center]]
Table of Contents
[[/center]]

[[toc]]

[[/box]]

## Rust

`git clone --depth=1 https://github.com/rust-lang/rust;tokei rust;rm -r rust;`

Rust is the best programming language ever existed. All its source code is on Github. It includes the compiler, std, and documentation. It doesn't include LLVM.


```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Alex                    1           23           18            0            5
 AsciiDoc                2         1483         1196           70          217
 GNU Style Assembly      5          394          268          103           23
 Autoconf                1           88           70            3           15
 Batch                   3           23           14            3            6
 C                      69         1706         1305          138          263
 C Header                1          121          105            1           15
 CMake                   1           33           25            0            8
 C++                    12         4295         3509          278          508
 CSS                    10         4222         3471          195          556
 Dockerfile             51         2404         1812          179          413
 FreeMarker             16          913          660            0          253
 JavaScript             78         7997         6356         1049          592
 JSON                   22          912          911            0            1
 Makefile              295         5123         3363          929          831
 Module-Definition       3           13           13            0            0
 Pascal                 12          301           64          221           16
 PowerShell              1           28           16            8            4
 Python                 32         6258         4903          483          872
 Shell                 114         5256         3626          823          807
 SVG                     9          320          316            2            2
 Plain Text            161         7185            0         6989          196
 TOML                  286         5115         4200          261          654
 TypeScript             19         3976         3243          222          511
 XSL                     2           58           44            8            6
 XML                     4          120          102           18            0
 YAML                    1          754          527          107          120
-------------------------------------------------------------------------------
 HTML                   82         1594         1430           18          146
 |- CSS                 24         1250         1160            9           81
 |- JavaScript           6          259          247            0           12
 (Total)                           3103         2837           27          239
-------------------------------------------------------------------------------
 Markdown              846        56083            0        43438        12645
 |- BASH                31          485          404           61           20
 |- Batch                1            2            2            0            0
 |- C                    2            3            3            0            0
 |- HTML                 1            1            1            0            0
 |- JavaScript           1          203           88          115            0
 |- JSON                 4            7            7            0            0
 |- Kotlin               1            3            3            0            0
 |- Markdown             5           26            0           21            5
 |- PowerShell           1            4            4            0            0
 |- Rust                86         4199         3222          435          542
 |- Shell                7          273          250           14            9
 |- TOML                25          112          105            2            5
 |- TypeScript           1          163          118           40            5
 |- YAML                 1            7            7            0            0
 (Total)                          61571         4214        44126        13231
-------------------------------------------------------------------------------
 Rust                22981      1773777      1397269       160734       215774
 |- Markdown          3467       178752         7991       136171        34590
 (Total)                        1952529      1405260       296905       250364
===============================================================================
 Total               25120      1890575      1438836       216280       235459
===============================================================================
```

The language is perfectly self-hosted, containing 1.77 million lines of Rust code. It has tons of markdown and html files, for documentation. It also has a few lines of C/C++ code, which are for low-level language features I guess.

## Linux Kernel

`git clone --depth=1 https://github.com/torvalds/linux;tokei linux;rm -r linux;`

It's the biggest open-source project, ever.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 ASN.1                  16          588          437           32          119
 Assembly                5         3256         3003            0          253
 GNU Style Assembly   1312       369695       269423        56812        43460
 Autoconf                5          345          301           21           23
 Automake                3           31           23            3            5
 BASH                   56         1744         1159          336          249
 C                   31802     22264207     16514034      2541199      3208974
 C Header            23368      8876950      6842562      1338241       696147
 C++                     8         2401         2071           72          258
 C++ Header              2          125           59           55           11
 CSS                     2          162           90           37           35
 Device Tree          4433      1243598      1017234        68106       158258
 Gherkin (Cucumber)      1          267          187           50           30
 Happy                   9         6019         5321            0          698
 HEX                     1           86           86            0            0
 INI                     1            7            3            3            1
 JSON                  575       305037       305035            0            2
 LD Script               8          340          264           29           47
 Makefile             2761        71184        48701        11764        10719
 Module-Definition       2          124          109            0           15
 Objective-C             1           89           72            0           17
 Perl                   56        42331        32649         3920         5762
 Python                151        44139        34688         3543         5908
 ReStructuredText     3200       644985       489142            0       155843
 Ruby                    1           29           25            0            4
 Shell                 756       139301        96856        17728        24717
 SVG                    65        43088        41845         1162           81
 TeX                     1          226          147           73            6
 Plain Text           2125       172052            0       140950        31102
 Unreal Script           5          716          446          162          108
 Vim script              1           42           33            6            3
 XSL                    15          300          183           78           39
 YAML                 2869       302134       243270        12186        46678
===============================================================================
 Total               73616     34535598     25949458      4196568      4389572
===============================================================================
```

It has 30 million lines of C. That's huge.

## MDxt

`git clone --depth=1 https://github.com/baehyunsol/MDxt;tokei MDxt;rm -r MDxt;`

This is the markdown engine of my blog. I implemented the entire thing by my self.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 CSS                     1          541          513           13           15
 Sass                    3          848          638           13          197
 TOML                    1           16           13            1            2
-------------------------------------------------------------------------------
 HTML                    1          109          108            0            1
 |- CSS                  1          541          513           13           15
 |- JavaScript           1           12           10            0            2
 (Total)                            662          631           13           18
-------------------------------------------------------------------------------
 Markdown                9          566            0          382          184
 |- CSS                  1          116          102            7            7
 |- Markdown             1            3            0            2            1
 |- Rust                 1           48           34           10            4
 |- XML                  1           66           63            2            1
 (Total)                            799          199          403          197
-------------------------------------------------------------------------------
 Rust                   63        10800         8899          162         1739
 |- Markdown             4           33            6           26            1
 (Total)                          10833         8905          188         1740
===============================================================================
 Total                  78        12880        10171          571         2138
===============================================================================
```

As you see, everything is written in Rust. It only has 10k lines of code, but has rich features.

## Nushell

`git clone --depth=1 https://github.com/nushell/nushell;tokei nushell;rm -r nushell;`

My everyday shell. It's literally a *new* type of shell.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Batch                   1           36           29            0            7
 Dockerfile              1           31           21            5            5
 INI                     2           26           19            1            6
 JSON                   55          808          804            0            4
 PowerShell              1           30           23            1            6
 Python                  1          420          380           28           12
 Shell                   3           72           57            4           11
 Plain Text              4           97            0           96            1
 TOML                   36         1029          849           50          130
 XML                     1           22           18            0            4
 YAML                    2           47           38            3            6
-------------------------------------------------------------------------------
 Markdown               19          674            0          458          216
 |- BASH                 2            7            5            2            0
 |- Rust                 3           29           21            0            8
 |- Shell                2           65           65            0            0
 |- SQL                  1            1            1            0            0
 |- TOML                 1            2            2            0            0
 (Total)                            778           94          460          224
-------------------------------------------------------------------------------
 Rust                  953       162548       142110         3440        16998
 |- Markdown            76         1718           52         1359          307
 (Total)                         164266       142162         4799        17305
===============================================================================
 Total                1079       165840       144348         4086        17406
===============================================================================
```

It has 162k lines of Rust. You can see that it's written purely in Rust.

## Zig

`git clone --depth=1 https://github.com/ziglang/zig;tokei zig;rm -r zig;`

Another sexy language. I'll dive deeper into this language when it gets 1.0.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 GNU Style Assembly    745        35555        28373         4981         2201
 Autoconf               38        26506        20800         4552         1154
 C                    4177       217506       151991        44381        21134
 C Header             7471      2590320      1988413       309506       292401
 CMake                   7         1620         1447           63          110
 C++                   206       142957       118103         9452        15402
 C++ Header             54        21337        17036         1640         2661
 JavaScript              1         3426         2952          254          220
 JSON                    1          630          630            0            0
 Makefile                4            4            4            0            0
 Module-Definition     480        48829        45755         2867          207
 Objective-C             2           23           19            0            4
 Objective-C++           2           25           21            0            4
 Python                  4          432          333           16           83
 Shell                  14          963          627          142          194
 Plain Text             12         3608            0         2570         1038
 YAML                    3          329          273           14           42
 Zig                  2187       653445       547247        50049        56149
-------------------------------------------------------------------------------
 HTML                    5         2715         2396            0          319
 |- CSS                  1          535          478            6           51
 (Total)                           3250         2874            6          370
-------------------------------------------------------------------------------
 Markdown                3          364            0          316           48
 |- Zig                  1            8            0            8            0
 (Total)                            372            0          324           48
===============================================================================
 Total               15416      3750594      2926420       430803       393371
===============================================================================
```

It's not self-hosted yet. It has 2.8M lines of C. I guess 650k lines of Zig are for its std lib.

## Tokei

`git clone --depth=1 https://github.com/XAMPPRocky/tokei;tokei tokei;rm -r tokei;`

It's the tool that I've been using for this blog post. It's super-fast and super-accurate!

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 BASH                    4           48           30           10            8
 JSON                    1         1544         1544            0            0
 Shell                   1           49           38            1           10
 TOML                    3          123          102            5           16
-------------------------------------------------------------------------------
 HTML                    1           12            9            1            2
 |- JavaScript           1           15           11            4            0
 (Total)                             27           20            5            2
-------------------------------------------------------------------------------
 Markdown                5         1503            0         1196          307
 |- JSON                 1           47           47            0            0
 |- Rust                 1            7            4            3            0
 |- Shell                1           16           14            0            2
 (Total)                           1573           65         1199          309
-------------------------------------------------------------------------------
 Rust                   23         4305         3616          126          563
 |- Markdown            13          374            5          318           51
 (Total)                           4679         3621          444          614
===============================================================================
 Total                  38         7584         5339         1339          906
===============================================================================
```

The table is relatively simple. Everything is written in Rust, with documents in markdown and html.

## Flutter

`git clone --depth=1 https://github.com/flutter/flutter;tokei flutter;rm -r flutter;`

Flutter is a cross-platform SDK made by Google. It has an awesome declarative syntax.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 BASH                    5          249           97          118           34
 Batch                   7          362          211           97           54
 C Header               82         1762          793          528          441
 CMake                  28         1948         1337          318          293
 C++                    33         2534         1819          228          487
 CSS                     2          280          226            8           46
 Dart                 3704      1438378      1086963       199595       151820
 Dockerfile              1          117           77           25           15
 GLSL                    1          102           83            4           15
 Java                   44         2751         2115          264          372
 JavaScript              2          138          116           11           11
 JSON                  143        21818        21736            0           82
 Kotlin                 10          245          187           20           38
 Objective-C            76         2642         1834          297          511
 PowerShell              1           98           62           22           14
 Prolog                  2           28           24            0            4
 Protocol Buffers        1          135           55           46           34
 Ruby                    1          289          140          102           47
 Shell                  15         1073          640          290          143
 SVG                    14           59           58            1            0
 Swift                  32          662          438          119          105
 Plain Text             20          283            0          261           22
 Xcode Config          112          374          282           68           24
 XML                   112         2500         1530          765          205
 YAML                  102         9840         8288          842          710
-------------------------------------------------------------------------------
 HTML                   25          496          350          118           28
 |- CSS                  2           23           20            2            1
 |- JavaScript          13          393          309           72           12
 (Total)                            912          679          192           41
-------------------------------------------------------------------------------
 Markdown              101         3961            0         2799         1162
 |- BASH                 4           21           20            1            0
 |- Dart                 5          110           76           20           14
 |- Java                 1           12           10            0            2
 |- JSON                 1            9            9            0            0
 |- Objective-C          1            4            3            0            1
 |- Ruby                 1            8            6            1            1
 |- Shell                5           14           14            0            0
 |- YAML                 1            2            2            0            0
 (Total)                           4141          140         2821         1180
===============================================================================
 Total                4676      1493124      1129461       206946       156717
===============================================================================
```

The main language is Dart, which has 1.4M lines. It also has Kotlin and Swift for mobile platforms.

## Visual Studio Code

`git clone --depth=1 https://github.com/microsoft/vscode;tokei vscode;rm -r vscode;`

No.1 cross platform text editor made by Microsoft. It powers most of my projects.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 BASH                    1           34           22            6            6
 Batch                  20          594          374           75          145
 C                       1           30           27            2            1
 C#                      1           27           23            1            3
 Clojure                 1           52           38            8            6
 CoffeeScript            2           43           32            0           11
 C++                     6           89           73            6           10
 CSS                   209        21098        16194         1413         3491
 Dart                    1           19           15            1            3
 Dockerfile              1           14           10            1            3
 F#                      1           18           11            3            4
 Fish                    1           96           45           36           15
 Go                      2           26           21            1            4
 Groovy                  1          219          106           49           64
 Handlebars              2           54           46            1            7
 HLSL                    1           13           10            0            3
 INI                     1           10            7            2            1
 Java                    1           42           23           11            8
 JavaScript            269        55886        49716         3757         2413
 JSON                  587       419578       419514            0           64
 JSX                     1           35           24            5            6
 Julia                   1           26           23            1            2
 LESS                    4           62           50            1           11
 Lua                     1           12           10            1            1
 Makefile                1           92           65            7           20
 Objective-C             1           52           33            9           10
 Objective-C++           1           52           33            9           10
 Perl                    2           77           55            9           13
 PHP                     3           58           43            5           10
 PowerShell              9          353          253           55           45
 Pug                     2           30           22            5            3
 Python                  2          101           81            4           16
 R                       1           24           11            9            4
 Razor                   1           46           34            4            8
 ReStructuredText        1           98           60            0           38
 Ruby                    1           46           26           14            6
 Rust                    2           41           31            0           10
 Sass                    4          355          265           53           37
 Shell                  37         1465         1013          221          231
 SQL                     1            6            6            0            0
 SVG                    72         4243         4206            9           28
 Swift                   1           13           13            0            0
 TeX                     2           40           34            0            6
 Plain Text             75        19423            0        15375         4048
 TSX                     1            6            5            0            1
 TypeScript           3956      1116847       857594        99188       160065
 Visual Basic            1           25           16            6            3
 XML                     5           61           60            1            0
 YAML                   28         4337         3662          168          507
 Zsh                     4          165          118           26           21
-------------------------------------------------------------------------------
 HTML                   43         3712         3562           42          108
 |- CSS                  8          261          225            0           36
 |- JavaScript          30         3265         2458          406          401
 (Total)                           7238         6245          448          545
-------------------------------------------------------------------------------
 Jupyter Notebooks       1            0            0            0            0
 |- Markdown             1            1            0            1            0
 |- Python               1            2            2            0            0
 (Total)                              3            2            1            0
-------------------------------------------------------------------------------
 Markdown               68         2031            0         1331          700
 |- BASH                 1           25           14            6            5
 |- JSON                 1           23           23            0            0
 (Total)                           2079           37         1337          705
===============================================================================
 Total                5442      1651876      1357715       121931       172230
===============================================================================
```

The table is huge. It's even bigger than Linux and Flutter. Javascript and Typescript are the main languages of this repo. It uses JSON for config files. I guess shell scripts are for cross platform installations. It even has Rust, Go and Coffeescript. What are those for?

## Go

`git clone --depth=1 https://github.com/golang/go;tokei go;rm -r go;`

Though I'm not a gopher, I wanted something that's similar to Rust: a language that's relatively new, self-hosted and has big community support.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Alex                    2          117          101            0           16
 GNU Style Assembly    553       153829       122681        16584        14564
 Autoconf                9          283          274            0            9
 BASH                   16          874          505          250          119
 Batch                   5          307          183           69           55
 C                     129         8763         6584          938         1241
 C Header               25          906          493          287          126
 C++                     2           49           24           14           11
 CSS                     2          273          269            0            4
 Dockerfile              2           56           25           20           11
 FORTRAN Modern          2           12            8            3            1
 Go                   8902      2260077      1764816       303437       191824
 JavaScript              9         1432         1119          140          173
 JSON                   16         2855         2855            0            0
 Makefile                5           40           20           10           10
 Markdown               16         2202            0         1702          500
 Objective-C             2           21           15            3            3
 Perl                    9         1358         1030          170          158
 Python                  1          611          420           70          121
 Shell                  14         2624         2154          321          149
 Plain Text           1000       190310            0       179839        10471
-------------------------------------------------------------------------------
 HTML                   15        18720        18273           39          408
 |- CSS                  3         2070         1851           10          209
 |- HTML                 1          219          212            0            7
 |- JavaScript           7         6919         6875           16           28
 (Total)                          27928        27211           65          652
===============================================================================
 Total               10736      2645719      1921849       503896       219974
===============================================================================
```

It's perfectly self hosted. It has 2.2M lines of Go, which is bigger than 1.8M of Rust.

## Bevy

`git clone --depth=1 https://github.com/bevyengine/bevy;tokei bevy;rm -r bevy;`

Bevy is a Rust game engine.

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 C Header                1            1            1            0            0
 GLSL                    2           44           34            0           10
 Makefile                1           25           18            0            7
 Objective-C             1            6            5            0            1
 Pan                     1          314          221           28           65
 Rusty Object Nota|      1           64           64            0            0
 Shell                   3          141          109           13           19
 SVG                     5          641          641            0            0
 Plain Text              3           44            0           27           17
 TOML                   50         2840         2294          106          440
-------------------------------------------------------------------------------
 HTML                    1            9            9            0            0
 |- CSS                  1           16           16            0            0
 |- JavaScript           1            2            2            0            0
 (Total)                             27           27            0            0
-------------------------------------------------------------------------------
 Markdown               25         4056            0         3421          635
 |- BASH                 3           23           21            2            0
 |- JSON                 1            3            3            0            0
 |- Nix                  1           13           13            0            0
 |- Rust                 7          446          326           39           81
 |- TOML                 2           25           14            7            4
 (Total)                           4566          377         3469          720
-------------------------------------------------------------------------------
 Rust                  549       105993        90482         3715        11796
 |- Markdown           403        14746          309        11990         2447
 (Total)                         120739        90791        15705        14243
===============================================================================
 Total                 643       114178        93878         7310        12990
===============================================================================
```

It has 105k lines of Rust. It's interesting that it only has 44 lines of shader language.