use super::*;
use crate::layout::PadNode;

/// `pad`: Pad content at the sides.
///
/// # Positional parameters
/// - Padding for all sides: `padding`, of type `linear` relative to sides.
/// - Body: of type `template`.
///
/// # Named parameters
/// - Left padding: `left`, of type `linear` relative to parent width.
/// - Right padding: `right`, of type `linear` relative to parent width.
/// - Top padding: `top`, of type `linear` relative to parent height.
/// - Bottom padding: `bottom`, of type `linear` relative to parent height.
///
/// # Return value
/// A template that sets the body into a padded area.
pub fn pad(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    let all = args.find(ctx);
    let left = args.get(ctx, "left");
    let top = args.get(ctx, "top");
    let right = args.get(ctx, "right");
    let bottom = args.get(ctx, "bottom");
    let body = args.require::<TemplateValue>(ctx, "body").unwrap_or_default();

    let padding = Sides::new(
        left.or(all).unwrap_or_default(),
        top.or(all).unwrap_or_default(),
        right.or(all).unwrap_or_default(),
        bottom.or(all).unwrap_or_default(),
    );

    Value::template("pad", move |ctx| {
        let child = ctx.exec_group(&body).into();
        ctx.push_into_par(PadNode { padding, child });
    })
}