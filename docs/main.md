# Policy of re-implementation of particle module
ひとまず日本語で整理する。   
Python で実装されている [particle](https://github.com/Surpris/particle) パッケージに実装されているサブモジュールは次の二つ。

* `core`
* `shape`

Python での実装は object-oriented である一方で、Rust では functional な実装になる。そのため次のような対応が基本となると考えている。

* Python でクラスとして実装しているものは Rust で struct/module/trait のいずれかで実装する。
* Python で関数群として実装されているスクリプトは Rust で module として実装する。

以下、この対応方針を基に各サブモジュールの Rust での実装方針を考える。

## Re-implementation of `core`
`core` に含まれているスクリプトの機能と、それらを Rust で実装する際に struct にするのか trait にするのかどうかという方針を下表に示す。

| Script           | Type in Python     | Type in Rust     |
| :--------------- | :----------------- | :--------------- |
| ensemble.py      | wrapper class      | struct           |
| in_put_shape.py  | functions          | module           |
| mathfunctions.py | functions          | module           |
| particle.py      | wrapper class      | struct           |
| slicefft.py      | class of functions | trait and module |
| space.py         | class              | struct           |

* space.py は Space 構造体として実装し、空間の情報の一部を保持して必要な時に空間メッシュを計算する。
* slicefft.py は Slicefft トレイトとして実装する。FFT 計算する部分は別のモジュールまたは構造体として実装する。
    * Slife
* in_out_shape.py は関数群なので utils モジュール内で InOutShape モジュールとして実装する。
* mathfunctions.py は関数群なので utils モジュール内で MathFunctions モジュールとして実装する。
* particle.py は `shape` を包括的に扱うためのクラスであり、ラッパー構造体として用意する。
* ensemble.py は `ensemble_system` と `ensemble` とあるのだが、前者は `Universe` として実装する。
    * `ensemble` は Ensemble モジュールとして実装する。
    * Universe が展開（初期化）されて、その中に次を持たせる。
        * `space` オブジェクト
        * `particle` オブジェクトのベクタを持つ `ensemble`
    * Universe は `ensemble` の FFT 実装を呼び出す。




## Re-implementation of `shape`
| Script            | Type in Python     | Type in Rust |
| :---------------- | :----------------- | :----------- |
| `<shape_name>`.py | wrapper class      | struct       |
| particleshpae.py  | function           | module       |
| polyhedron.py     | class of functions | super trait  |
| shapeslice.py     | class of functions | trait        |
| spheroid.py       | class              | trait        |
