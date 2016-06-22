#![feature(plugin_registrar, box_syntax, rustc_private)]
#[macro_use] extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_const_eval;
extern crate syntax;

use syntax::attr::contains_name;
use rustc::lint::{LintPass, LateLintPass, LintArray, LateContext, LintContext};
use rustc_const_eval::eval_const_expr;
use rustc::middle::const_val::ConstVal::*;
use rustc_plugin::registry::Registry;
use rustc::hir;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, TTMacroExpander, MacEager};
use syntax::ext::base::SyntaxExtension::NormalTT;
use syntax::codemap::Span;
use syntax::ast::{TokenTree, ItemKind};
use syntax::parse::token::{intern, gensym_ident};
use syntax::ext::build::AstBuilder;
use syntax::feature_gate::AttributeType;

declare_lint!(STATIC_ASSERT, Deny,
              "check compile-time information");

struct StaticAssertPass;

impl LintPass for StaticAssertPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(STATIC_ASSERT)
    }
}

impl LateLintPass for StaticAssertPass {
    fn check_item(&mut self, cx: &LateContext, it: &hir::Item) {
        if !contains_name(&it.attrs, "static_assert_helper_attribute") {
            return;
        }
        let evaluated = if let hir::ItemConst(_, ref expr) = it.node {
            eval_const_expr(cx.tcx, expr)
        } else {
            return
        };
        match evaluated {
            Bool(true) => {},
            Bool(false) => cx.span_lint(STATIC_ASSERT, it.span, "static assertion failed"),
            c => cx.sess().struct_span_err(it.span, &format!("static assertion on {:?}", c)).emit(),
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(box StaticAssertPass);
    reg.register_syntax_extension(intern("static_assert"), NormalTT(box StaticAssertMacro, None, false));
    reg.register_attribute("static_assert_helper_attribute".to_owned(), AttributeType::Whitelisted)
}

struct StaticAssertMacro;

impl TTMacroExpander for StaticAssertMacro {
    fn expand<'cx>(&self, cx: &'cx mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'cx> {
        match cx.new_parser_from_tts(args).parse_expr() {
            Ok(e) => {
                let item = cx.item(
                    sp,
                    gensym_ident("ASSERTION"),
                    vec![
                        cx.attribute(sp, cx.meta_word(sp, intern("static_assert_helper_attribute").as_str())),
                        cx.attribute(sp, cx.meta_list(sp, intern("allow").as_str(), vec![cx.meta_word(sp, intern("dead_code").as_str())])),
                    ],
                    ItemKind::Const(cx.ty_ident(sp, cx.ident_of("bool")), e),
                );
                box MacEager {
                    items: Some(syntax::util::small_vector::SmallVector::one(item.clone())),
                    stmts: Some(syntax::util::small_vector::SmallVector::one(cx.stmt_item(sp, item.clone()))),
                    expr: Some(cx.expr_block(cx.block(sp, vec![cx.stmt_item(sp, item)], None))),
                    .. MacEager::default()
                }
            },
            Err(mut e) => {
                e.emit();
                DummyResult::any(sp)
            },
        }
    }
}
