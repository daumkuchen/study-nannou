// nannouのアイテムをロードする。
// Rustにはグルーバルに適用されるモジュールはなく、ファイル単位で必要モジュールをロードする必要がある。
use nannou::prelude::*;

// 実行ファイルのエントリポイントとなる。
// appに定義すべき関数などを任意で渡したのち実行する。
fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

// appの状態をModelという構造体に定義していく。
// MVCパターンのModelに相当。
// Modelは、eventなどによって更新されていくことを想定されたパラメータ群となる。
struct Model {}

// Model構造体のインスタンスを生成して返却する
fn model(_app: &App) -> Model {
    Model {}
}

// イベントハンドラに相当。eventに応じて、modelを更新していく。
fn event(_app: &App, _model: &mut Model, _event: Event) {
}

// draw関数に相当。
fn view(_app: &App, _model: &Model, frame: &Frame) {
    frame.clear(RED);
}   