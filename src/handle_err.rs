use erg_common::error::ErrorKind;
// use erg_common::style::{remove_style, StyledString, Color};
use erg_compiler::error::{CompileErrors, CompileError};
use erg_compiler::context::Context;

pub(crate) fn filter_errors(ctx: &Context, errors: CompileErrors) -> CompileErrors {
    errors.into_iter().filter_map(|error| filter_error(ctx, error)).collect()
}

fn filter_error(_ctx: &Context, error: CompileError) -> Option<CompileError> {
    match error.core.kind {
        ErrorKind::VisibilityError => None,
        ErrorKind::AssignError => handle_assign_error(error),
        _ => Some(error),
    }
}

fn handle_assign_error(error: CompileError) -> Option<CompileError> {
    if error.core.main_message.ends_with("cannot be assigned more than once") {
        None
    } else {
        Some(error)
    }
}
