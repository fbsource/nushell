use crate::errors::ShellError;
use crate::prelude::*;

pub fn skip(args: CommandArgs) -> Result<OutputStream, ShellError> {
    if args.positional.len() == 0 {
        if let Some(span) = args.name_span {
            return Err(ShellError::labeled_error(
                "Skip requires an amount",
                "needs parameter",
                span,
            ));
        } else {
            return Err(ShellError::string("skip requires an amount."));
        }
    }

    let amount = args.positional[0].as_i64();

    let amount = match amount {
        Ok(o) => o,
        Err(_) => {
            return Err(ShellError::labeled_error(
                "Value is not a number",
                "expected integer",
                args.positional[0].span,
            ))
        }
    };

    let input = args.input;

    Ok(input
        .skip(amount as u64)
        .map(|v| ReturnValue::Value(v))
        .boxed())
}
