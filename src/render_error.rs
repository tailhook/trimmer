use std::fmt;
use std::io;

use {Pos, TargetKind};


quick_error! {
    /// This error is used to describe invalid variable usage in template
    #[derive(Debug)]
    pub enum DataError {
        /// Unsupported get attribute operation
        AttrUnsupported(typename: &'static str) {
            description("object doesn't support getting attribute `a.b`")
            display("object {} doesn't support getting attribute", typename)
        }
        /// No suche attribute on this object
        AttrNotFound {
            description("object doesn't have such attibute")
        }
        /// Unsupported subscription operation
        IndexUnsupported(typename: &'static str) {
            description("object doesn't support subscription `a[b]`")
            display("object {} doesn't support subscription", typename)
        }
        /// Unsupported using this object as a key in dictionary subscription
        StrKeyUnsupported(typename: &'static str) {
            description("can't be stringified for subsciption `a[b]`")
            display("object {} can't be stringified to be used as key", typename)
        }
        /// Unsupported using this object as a key in array subscription
        IntKeyUnsupported(typename: &'static str) {
            description("can't used as integer key for subscription")
            display("object {} can't be a key for subscription", typename)
        }
        /// No such index on this object
        IndexNotFound {
            description("object doesn't have value at specified index")
        }
        /// The object can't be output
        OutputUnsupported(typename: &'static str) {
            description("can't print object of this type")
            display("can't print object of type {}", typename)
        }
        /// The object can't be boolean
        BoolUnsupported(typename: &'static str) {
            description("can't treat object of this type as bool")
            display("can't treat object of type {} as bool", typename)
        }
        /// The object can't be boolean
        IterationUnsupported(typename: &'static str, kind: TargetKind) {
            description("can't iterate over the object")
            display("can't iterate over the object of type {}, \
                (kind {})", typename, kind)
        }
        /// Variable or attribute not found
        VariableNotFound(name: String) {
            description("variable or attribute not found")
            display("variable or attribute {:?} not found", name)
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
