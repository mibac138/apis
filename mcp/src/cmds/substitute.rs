use crate::options::substitute::Args;
use failure::Error;
use itertools::Itertools;
use sheesy_tools::substitute::StreamOrPath;

use sheesy_tools::substitute::substitute;

pub fn execute(args: Args) -> Result<(), Error> {
    let args = args.sanitized()?;
    let replacements: Vec<_> = args.replacements.into_iter().tuples().collect();
    substitute(
        args.engine,
        &args.data.unwrap_or(StreamOrPath::Stream),
        &args.specs,
        &args.separator,
        args.validate,
        &replacements,
    )
}
