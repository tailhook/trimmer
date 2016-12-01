use std::fmt;
use std::io;

use {Pos};


quick_error! {
    /// This error is used to describe invalid variable usage in template
    #[derive(Debug)]
    pub enum DataError {
        /// Unsupported get attribute operation
        AttrUnsupported(typename: &'static str) {
            description("object doesn't support getting attribute `a.b`")
            display("object {} doesn't support getting attribute", typename)
        }
        /// Unsupported subscription operation
        SubscriptUnsupported(typename: &'static str) {
            description("object doesn't support subscription `a[b]`")
            display("object {} doesn't support subscription", typename)
        }
        /// The object can't be created
        OutputUnsupported(typename: &'static str) {
            description("can't print object of type")
            display("can't print object of type {}", typename)
        }
        /// Variable not found
        VariableNotFound(name: String) {
            description("variable not found")
            display("variable {:?} not found", name)
        }
    }
}


/// Structure that tracks where error occured
#[derive(Debug)]
pub struct ErrorTracker {
    source: (Pos, Pos),
    destination: usize,
    error: DataError,
}


quick_error! {
    /// Error rendering template
    #[derive(Debug)]
    pub enum RenderError {
        /// Error writing into output buffer
        Io(err: io::Error) {
            display("I/O error: {}", err)
            description("I/O error")
            from()
        }
        /// Error formatting value
        ///
        /// TODO(tailhook) move it to the list of errors
        Fmt(err: fmt::Error) {
            description("error formatting value")
            from()
        }
        /// Error when some of the variable has unexpected type or does
        /// not support required operation
        ///
        /// When this kind of error occurs we try to skip error and do our
        /// best to continue rendering and collect more errors
        Data(errs: Vec<ErrorTracker>) {
            display("data error: {}", errs.iter()
                .map(|t| format!("{}: {}", t.source.0, t.error))
                .collect::<Vec<_>>().join("\n  "))
            description("data error")
        }
    }
}

pub fn tracker(source_span: (Pos, Pos), dest_pos: usize, err: DataError)
    -> ErrorTracker
{
    ErrorTracker {
        source: source_span,
        destination: dest_pos,
        error: err,
    }
}
