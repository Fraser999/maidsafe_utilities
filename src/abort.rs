// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

/// This macro is intended to be used in all cases where we unwrap() a result to deliberately panic
/// in case of error - eg., in test-cases. Such unwraps don't give a precise point of failure in
/// our code and instead indicate some line number in core library. This macro will provide a
/// precise point of failure and will decorate the failure for easy viewing.
///
/// #Examples
///
/// ```
/// # #[macro_use] extern crate maidsafe_utilities;
/// # fn main() {
/// let some_result: Result<String, std::io::Error> = Ok("Hello".to_string());
/// let string_length = eval_result!(some_result).len();
/// assert_eq!(string_length, 5);
/// # }
/// ```
#[macro_export]
macro_rules! eval_result {
    ($result:expr) => {
        $result.unwrap_or_else(|error| {
            let decorator = (0..50).map(|_| "-").collect::<String>();
            panic!("\n\n {}\n| {:?}\n {}\n\n", decorator, error, decorator)
        })
    }
}

/// This macro is intended to be used in all cases where we unwrap() an option to deliberately
/// panic in case of error - eg., in test-cases. Such unwraps don't give a precise point of
/// failure in our code and instead indicate some line number in core library. This macro will
/// provide a precise point of failure and will decorate the failure for easy viewing.
///
/// #Examples
///
/// ```
/// # #[macro_use] extern crate maidsafe_utilities;
/// # fn main() {
/// let some_option = Some("Hello".to_string());
/// let string_length = eval_option!(some_option, "This is a user-supplied text.").len();
/// assert_eq!(string_length, 5);
/// # }
/// ```
#[macro_export]
macro_rules! eval_option {
    ($option:expr, $user_string:expr) => {
        $option.unwrap_or_else(|| {
            let error = "Option Evaluated to None ! ".to_string() + $user_string;
            let decorator = (0..50).map(|_| "-").collect::<String>();
            panic!("\n\n {}\n| {:?}\n {}\n\n", decorator, error, decorator)
        })
    }
}
