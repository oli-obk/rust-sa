#![feature(plugin_registrar, box_syntax, rustc_private)]
#[macro_use] extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_front;
extern crate syntax;

use rustc::lint::{LintPass, LateLintPass, LintArray, LateContext, LintContext};
use syntax::attr::contains_name;
use rustc::middle::const_eval::eval_const_expr;
use rustc::middle::const_eval::ConstVal::*;
use rustc_plugin::registry::Registry;
use rustc_front::hir;

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
        if !contains_name(&it.attrs, "static_assert_") {
            return;
        }
        let evaluated = if let hir::ItemConst(_, ref expr) = it.node {
            eval_const_expr(&cx.tcx, expr)
        } else {
            return
        };
        match evaluated {
            Bool(true) => {},
            Bool(false) => cx.span_lint(STATIC_ASSERT, it.span, "static assertion failed"),
            // FIXME: https://github.com/rust-lang/rust/issues/25307
            Int(i) if i == 1 => {},
            _ => cx.sess().span_err(it.span, &format!("static assertion on {:?}", evaluated)),
        }
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_late_lint_pass(box StaticAssertPass); // as LintPassObject
}

#[macro_export]
macro_rules! static_assert(
    ($e:expr) => {
        #[static_assert_]
        #[allow(dead_code)]
        const COND: bool = $e;
    }
);
