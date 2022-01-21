use crate::dbs::Executor;
use crate::dbs::Level;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::err::Error;
use crate::sql::comment::mightbespace;
use crate::sql::comment::shouldbespace;
use crate::sql::error::IResult;
use crate::sql::ident::{ident, Ident};
use crate::sql::value::Value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OptionStatement {
	pub name: Ident,
	pub what: bool,
}

impl OptionStatement {
	pub async fn compute(
		&self,
		_ctx: &Runtime,
		opt: &Options<'_>,
		exe: &mut Executor,
		_doc: Option<&Value>,
	) -> Result<Value, Error> {
		// Allowed to run?
		exe.check(opt, Level::Db)?;
		// Return nothing
		Ok(Value::None)
	}
}

impl fmt::Display for OptionStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.what {
			write!(f, "OPTION {}", self.name)
		} else {
			write!(f, "OPTION {} = FALSE", self.name)
		}
	}
}

pub fn option(i: &str) -> IResult<&str, OptionStatement> {
	let (i, _) = tag_no_case("OPTION")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, n) = ident(i)?;
	let (i, v) = opt(alt((
		map(tuple((mightbespace, tag("="), mightbespace, tag_no_case("TRUE"))), |_| true),
		map(tuple((mightbespace, tag("="), mightbespace, tag_no_case("FALSE"))), |_| false),
	)))(i)?;
	Ok((
		i,
		OptionStatement {
			name: n,
			what: v.unwrap_or(true),
		},
	))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn option_statement() {
		let sql = "OPTION IMPORT";
		let res = option(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("OPTION IMPORT", format!("{}", out));
	}

	#[test]
	fn option_statement_true() {
		let sql = "OPTION IMPORT = TRUE";
		let res = option(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("OPTION IMPORT", format!("{}", out));
	}

	#[test]
	fn option_statement_false() {
		let sql = "OPTION IMPORT = FALSE";
		let res = option(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("OPTION IMPORT = FALSE", format!("{}", out));
	}
}