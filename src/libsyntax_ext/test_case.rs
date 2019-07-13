// http://rust-lang.org/COPYRIGHT.
//

// #[test_case] is used by custom test authors to mark tests
// When building for test, it needs to make the item public and gensym the name
// Otherwise, we'll omit the item. This behavior means that any item annotated
// with #[test_case] is never addressable.
//
// We mark item with an inert attribute "rustc_test_marker" which the test generation
// logic will pick up on.

use syntax::ext::base::*;
use syntax::ext::build::AstBuilder;
use syntax::ext::hygiene::SyntaxContext;
use syntax::ast;
use syntax::source_map::respan;
use syntax::symbol::sym;
use syntax_pos::Span;

pub fn expand(
    ecx: &mut ExtCtxt<'_>,
    attr_sp: Span,
    _meta_item: &ast::MetaItem,
    anno_item: Annotatable
) -> Vec<Annotatable> {
    if !ecx.ecfg.should_test { return vec![]; }

    let sp = attr_sp.with_ctxt(SyntaxContext::empty().apply_mark(ecx.current_expansion.mark));
    let mut item = anno_item.expect_item();
    item = item.map(|mut item| {
        item.vis = respan(item.vis.span, ast::VisibilityKind::Public);
        item.ident = item.ident.gensym();
        item.attrs.push(
            ecx.attribute(sp,
                ecx.meta_word(sp, sym::rustc_test_marker))
        );
        item
    });

    return vec![Annotatable::Item(item)]
}
