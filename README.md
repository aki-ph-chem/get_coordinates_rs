# coordinates_picker.rs

Rustで実装されたGaussianの\*LOGファイルから構造最適化された座標を取り出すツール

## インストール

ソースコードをクローンして`Cargo`でビルドして下さい。

```bash
$ git clone git@github.com:aki-ph-chem/get_coordinates_rs.git
$ cd coordinates_picker
$ cargo build --release
```

ビルドされたバイナリは`target/release/coordinates_picker`にあるのでそれを適当なパスの通ったディレクトリに配置して下さい。

もしArch Linuxユーザーならば、`build_pacman`([ここ](./build_pacman))にPKGBUILD等のパッケージングを行うスクリプトがあるのでそれを使って
パッケージをビルド&インストールして下さい。

補助スクリプト`build.sh`を使った例を以下に示します。

```bash
$ cd build_pacman 
$ ./build.sh
```

## 使い方

`build_pacman`にあるスクリプトを用いてインストールした場合では利用できるコマンド名は`pick_co`です。(別の方法でインストールした場合では異なります(自分で設定が可能))

このコマンドは以下の様に使います。
ここで、第1引数はGaussianの\*.LOGファイル、第2引数は取り出したい分子の原子座標の原子数で、第3引数は結果を書き込むファイル名です。

```bash
$ pick_co <file name of input> <numbers of atoms> <file name of output>
```

### 使用例

以下では`coordinates_picker/for_test`にあるサンプルデータを使い、このコマンド名の場合で使用例を示します。

```bash
$ pick_co test_input.LOG 12 output.csv 
```

このコマンドを実行した後では、現在いるディレクトリにoutput.csvというファイルが
生成されます。

このファイルの中身を見てみると以下のようなcsvファイルになっています。

```bash
$ cat gomi.csv
1,6,0,-0.042214,1.387483,-0.124879
2,6,0,-1.373387,0.677396,0.071920
3,6,0,-1.373402,-0.677370,0.071909
4,6,0,-0.042245,-1.387488,-0.124878
5,1,0,0.021754,1.869823,-1.110964
6,1,0,0.144642,2.156219,0.637581
7,1,0,-2.286675,1.266023,0.171891
8,1,0,-2.286704,-1.265977,0.171869
9,1,0,0.021727,-1.869859,-1.110946
10,1,0,0.144590,-2.156201,0.637611
11,15,0,1.320856,-0.000008,-0.013380
12,1,0,1.415322,-0.000024,1.439234
```

## ライセンス

MITライセンス
