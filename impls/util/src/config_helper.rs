use std::str::FromStr;

use abi_stable::std_types::{RResult, RStr, RString};
use RResult::RErr;

// use fractal_func::ROptionsMap;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct ConfigHelper {
//     config: ROptionsMap,
// }

// impl ConfigHelper {
//     // pub fn new(options: &[(&str, &str)]) -> Self {
//     // }
//     pub fn new() -> Self {
//         Self {
//             config: Default::default(),
//         }
//     }

//     pub fn with<V: ToString>(mut self, option_name: &str, v: V) -> Self {
//         self.config
//             .insert(option_name.to_owned().into(), v.to_string().into());
//         self
//     }

//     pub fn config(&self) -> &ROptionsMap {
//         &self.config
//     }

//     pub fn setter<'a, T: Clone>(
//         &self,
//         target: T,
//         name: RStr<'a>,
//         value: RStr<'a>,
//     ) -> OptionSetter<'a, T> {
//         OptionSetter::Unmatched {
//             target,
//             name: name.as_str(),
//             value: value.as_str(),
//         }
//     }
// }

pub enum OptionSetter<'a, T> {
    Unmatched {
        target: T,
        name: &'a str,
        value: &'a str,
    },
    Matched(Result<T, String>),
}

use OptionSetter::*;

impl<'a, T> OptionSetter<'a, T> {
    pub fn new(target: &T, name: RStr<'a>, value: RStr<'a>) -> OptionSetter<'a, T>
    where
        T: Clone,
    {
        OptionSetter::Unmatched {
            target: target.clone(),
            name: name.as_str(),
            value: value.as_str(),
        }
    }

    pub fn option<SF, V>(self, option_name: &str, setter_func: SF) -> Self
    where
        SF: Fn(&mut T, V),
        V: FromStr,
        <V as FromStr>::Err: ToString,
    {
        match self {
            Unmatched {
                target,
                name,
                value,
            } if name == option_name => Matched(
                V::from_str(value)
                    .map_err(|e| format!("error parsing option {}: {}", name, e.to_string()))
                    .map(move |v| {
                        let mut t = target;
                        setter_func(&mut t, v);
                        t
                    }),
            ),
            _ => self,
        }
    }

    pub fn try_option<SF, V, R>(self, option_name: &str, setter_func: SF) -> Self
    where
        SF: Fn(&mut T, V) -> R,
        V: FromStr,
        R: Into<Result<T, String>>,
        <V as FromStr>::Err: ToString,
    {
        match self {
            Unmatched {
                target,
                name,
                value,
            } if name == option_name => Matched(
                V::from_str(value)
                    .map_err(|e| format!("error parsing option {}: {}", name, e.to_string()))
                    .and_then(move |v| {
                        let mut t = target;
                        setter_func(&mut t, v).into()?;
                        Ok(t)
                    }),
            ),
            _ => self,
        }
    }

    pub fn mutate<F>(self, func: F) -> Self
    where
        F: Fn(&mut T),
    {
        match self {
            Matched(Ok(mut t)) => {
                func(&mut t);
                Matched(Ok(t))
            }
            _ => self,
        }
    }

    pub fn finish<TO>(self) -> RResult<TO, RString>
    where
        T: Into<TO>,
    {
        match self {
            Unmatched { name, .. } => RErr(format!("field not found: {}", name).into()),
            Matched(res) => RResult::from(res.map(|t| t.into()).map_err(RString::from)),
        }
    }
}
