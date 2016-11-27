use std::io;

use {Pos};


quick_error! {
    /// This error is used to describe invalid variable usage in template
    #[derive(Debug)]
    pub enum DataError {
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
