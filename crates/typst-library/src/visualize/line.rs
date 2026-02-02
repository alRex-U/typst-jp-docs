use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Content, NativeElement, Packed, Show, StyleChain};
use crate::layout::{Abs, Angle, Axes, BlockElem, Length, Rel};
use crate::visualize::Stroke;

/// ある点から別の点への線。
///
/// # 例
/// ```example
/// #set page(height: 100pt)
///
/// #line(length: 100%)
/// #line(end: (50%, 50%))
/// #line(
///   length: 4cm,
///   stroke: 2pt + maroon,
/// )
/// ```
#[elem(Show)]
pub struct LineElem {
    /// 線の始点。
    ///
    /// 2要素の[Relative Length](relative)からなる配列でなければなりません。
    #[resolve]
    pub start: Axes<Rel<Length>>,

    /// 線の終点。
    #[resolve]
    pub end: Option<Axes<Rel<Length>>>,

    /// 線の長さ。これは`end`オプションが`{none}`である場合にのみ有効です。
    #[resolve]
    #[default(Abs::pt(30.0).into())]
    pub length: Rel<Length>,

    /// 線の始点からの傾き。これは`end`オプションが`{none}`である場合にのみ有効です。
    pub angle: Angle,

    /// 線の描画に使用する[stroke]。
    ///
    /// ```example
    /// #set line(length: 100%)
    /// #stack(
    ///   spacing: 1em,
    ///   line(stroke: 2pt + red),
    ///   line(stroke: (paint: blue, thickness: 4pt, cap: "round")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: "dashed")),
    ///   line(stroke: (paint: blue, thickness: 1pt, dash: ("dot", 2pt, 4pt, 2pt))),
    /// )
    /// ```
    #[resolve]
    #[fold]
    pub stroke: Stroke,
}

impl Show for Packed<LineElem> {
    fn show(&self, engine: &mut Engine, _: StyleChain) -> SourceResult<Content> {
        Ok(BlockElem::single_layouter(self.clone(), engine.routines.layout_line)
            .pack()
            .spanned(self.span()))
    }
}
