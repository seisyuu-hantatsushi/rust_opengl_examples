# Introduction
## コンピュータで絵を書くということ
現在のコンピュータにはディスプレイが接続されています.
ディスプレイは三原色(赤,青,緑)の点で絵を表示しているので,
コンピュータがディスプレイに絵を表示するということは,
コンピュータがメモリ上のデータを三原色(赤,青,緑)と
ディスプレイのどの位置に三原色(赤,青,緑)を置くかということを
解釈して送っているということになります.

では,3次元空間の物体を2次元平面で描くためにはどうすればいいでしょうか.
それは,以下のような図を考えることになります.
視点と3次元の物体の間に,ディスプレイあるとして,
3次元物体と視点結ぶ線とディスプレイ平面の交点で色をつけることにより,
絵を書くことになります.

さて,この交点の位置を求めるためになにが必要でしょうか?
必要なのは,簡単な線形代数と幾何の知識となります.

