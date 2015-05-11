#![feature(plugin_registrar, box_syntax, rustc_private)]
#[macro_use] extern crate rustc;
extern crate syntax;

use rustc::lint::{LintPass, LintArray, Context};
use syntax::attr::contains_name;
use syntax::ast;
use rustc::middle::const_eval::eval_const_expr;
use rustc::middle::const_eval::const_val::const_bool;
use rustc::plugin::registry::Registry;

declare_lint!(STATIC_ASSERT, Forbid,
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
            const_bool(true) => cx.span_lint(STATIC_ASSERT, it.span, "static assertion failed"),
            const_bool(false) => {},
            _ => cx.span_lint(STATIC_ASSERT, it.span, "static assertion on non-bool"),
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box StaticAssertPass); // as LintPassObject
}
