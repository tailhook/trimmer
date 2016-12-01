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
        Data(errs: Vec<(Pos, DataError)>) {
            display("data error: {}", errs.iter()
                .map(|&(p, ref e)| format!("{}: {}", p, e))
                .collect::<Vec<_>>().join("\n  "))
            description("data error")
        }
    }
}
