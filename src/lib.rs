#![feature(plugin_registrar, box_syntax, rustc_private)]
#[macro_use] extern crate rustc;
extern crate syntax;

use rustc::lint::{LintPass, LintArray, Context};
use syntax::attr::contains_name;
use syntax::ast;
use rustc::middle::const_eval::eval_const_expr;
use rustc::middle::const_eval::const_val::*;
use rustc::plugin::registry::Registry;

declare_lint!(STATIC_ASSERT, Deny,
              "check compile-time information");

struct StaticAssertPass;

impl LintPass for StaticAssertPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(STATIC_ASSERT)
    }

    fn check_item(&mut self, cx: &Context, it: &ast::Item) {
        if !contains_name(&it.attrs, "static_assert_") {
            return;
        }
        let evaluated = match it.node {
            ast::ItemConst(_, ref expr) => eval_const_expr(&cx.tcx, expr),
            _ => return,
        };
        match evaluated {
            const_bool(true) => return,
            // FIXME: https://github.com/rust-lang/rust/issues/25307
            const_int(i) if i == 1 => return,
            const_uint(_) => cx.sess().span_err(it.span, "static assertion on uint"),
            const_float(_) => cx.sess().span_err(it.span, "static assertion on float"),
            const_str(_) => cx.sess().span_err(it.span, "static assertion on str"),
            const_binary(_) => cx.sess().span_err(it.span, "static assertion on binary"),
            Struct(_) => cx.sess().span_err(it.span, "static assertion on struct"),
            Tuple(_) => cx.sess().span_err(it.span, "static assertion on tuple"),
            _ => {}
        }
        cx.span_lint(STATIC_ASSERT, it.span, "static assertion failed");
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box StaticAssertPass); // as LintPassObject
}
