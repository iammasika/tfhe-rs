use crate::integer::prelude::*;
use crate::integer::{BooleanBlock, RadixCiphertext, ServerKey as IntegerServerKey};
use crate::strings::ciphertext::{FheString, GenericPatternRef, UIntArg};
use crate::strings::server_key::pattern::IsMatch;
use crate::strings::server_key::{FheStringIsEmpty, FheStringLen, ServerKey};
use std::borrow::Borrow;

impl<T: Borrow<IntegerServerKey> + Sync> ServerKey<T> {
    // Replaces the pattern ignoring the first `start` chars (i.e. these are not replaced)
    // Also returns the length up to the end of `to` in the replaced str, or 0 if there's no match
    fn replace_once(
        &self,
        replace: &BooleanBlock,
        find_index: &RadixCiphertext,
        from_len: &FheStringLen,
        enc_to_len: &RadixCiphertext,
        str: &FheString,
        to: &FheString,
    ) -> (FheString, RadixCiphertext) {
        let sk = self.inner();

        // When there's match we get the part of the str before and after the pattern by shifting.
        // Then we concatenate the left part with `to` and with the right part.
        // Visually:
        //
        // 1. We have str = [lhs, from, rhs]
        //
        // 2. Get the [lhs] and [rhs] by shifting str right and left, respectively
        //
        // 3. Concat [lhs] + [to] + [rhs]
        //
        // 4. We get [lhs, to, rhs]

        let (mut replaced, rhs) = rayon::join(
            || {
                let str_len = sk.create_trivial_radix(str.len() as u32, 16);

                // Get the [lhs] shifting right by [from, rhs].len()
                let shift_right = sk.sub_parallelized(&str_len, find_index);
                let mut lhs = self.right_shift_chars(str, &shift_right);
                // As lhs is shifted right we know there aren't nulls on the right, unless empty
                lhs.set_is_padded(false);

                let mut replaced = self.concat(&lhs, to);

                // Reverse the shifting such that nulls go to the new end
                replaced = self.left_shift_chars(&replaced, &shift_right);
                replaced.set_is_padded(true);

                replaced
            },
            || {
                // Get the [rhs] shifting left by [lhs, from].len()
                let shift_left = match from_len {
                    FheStringLen::NoPadding(len) => {
                        sk.scalar_add_parallelized(find_index, *len as u32)
                    }
                    FheStringLen::Padding(enc_len) => sk.add_parallelized(find_index, enc_len),
                };

                let mut rhs = self.left_shift_chars(str, &shift_left);
                rhs.set_is_padded(true);

                rhs
            },
        );

        replaced = self.concat(&replaced, &rhs);

        rayon::join(
            // Return the replaced value only when there is match, else return the original str
            || self.conditional_string(replace, &replaced, str),
            || {
                // If there's match we return [lhs, to].len(), else we return 0 (index default)
                let add_to_index = sk.if_then_else_parallelized(
                    replace,
                    enc_to_len,
                    &sk.create_trivial_zero_radix(16),
                );
                sk.add_parallelized(find_index, &add_to_index)
            },
        )
    }

    fn replace_n_times(
        &self,
        iterations: u16,
        result: &mut FheString,
        from: GenericPatternRef<'_>,
        to: &FheString,
        enc_n: Option<&RadixCiphertext>,
    ) {
        let sk = self.inner();

        let mut skip = sk.create_trivial_zero_radix(16);
        let trivial_or_enc_from = match from {
            GenericPatternRef::Clear(from) => FheString::trivial(self, from.str()),
            GenericPatternRef::Enc(from) => from.clone(),
        };

        let ((from_is_empty, from_len), (str_len, enc_to_len)) = rayon::join(
            || {
                rayon::join(
                    || self.is_empty(&trivial_or_enc_from),
                    || self.len(&trivial_or_enc_from),
                )
            },
            || {
                rayon::join(
                    || self.len(result),
                    || match self.len(to) {
                        FheStringLen::Padding(enc_val) => enc_val,
                        FheStringLen::NoPadding(val) => sk.create_trivial_radix(val as u32, 16),
                    },
                )
            },
        );

        for i in 0..iterations {
            let prev = result.clone();

            let (_, no_more_matches) = rayon::join(
                || {
                    // We first shift str `skip` chars left to ignore them and check if there's a
                    // match
                    let shifted_str = self.left_shift_chars(result, &skip);

                    let (mut index, is_match) = self.find(&shifted_str, from);

                    // We add `skip` to get the actual index of the pattern (in the non shifted str)
                    sk.add_assign_parallelized(&mut index, &skip);

                    (*result, skip) =
                        self.replace_once(&is_match, &index, &from_len, &enc_to_len, result, to);
                },
                || self.no_more_matches(&str_len, &from_is_empty, i, enc_n),
            );

            let num_blocks = skip.blocks().len();

            rayon::join(
                || *result = self.conditional_string(&no_more_matches, &prev, result),
                // If we replace "" to "a" in the "ww" str, we get "awawa". So when `from_is_empty`
                // we need to move to the next space between letters by adding 1 to the skip value
                || match &from_is_empty {
                    FheStringIsEmpty::Padding(enc) => sk.add_assign_parallelized(
                        &mut skip,
                        &enc.clone().into_radix(num_blocks, sk),
                    ),
                    FheStringIsEmpty::NoPadding(clear) => {
                        sk.scalar_add_assign_parallelized(&mut skip, *clear as u8);
                    }
                },
            );
        }
    }

    fn no_more_matches(
        &self,
        str_len: &FheStringLen,
        from_is_empty: &FheStringIsEmpty,
        current_iteration: u16,
        enc_n: Option<&RadixCiphertext>,
    ) -> BooleanBlock {
        let sk = self.inner();

        let (mut no_more_matches, enc_n_is_exceeded) = rayon::join(
            // If `from_is_empty` and our iteration exceeds the length of the str, that means
            // there cannot be more empty string matches.
            //
            // For instance "ww" can at most have 3 empty string matches, so we only take the
            // result at iteration 0, 1, and 2
            || {
                let no_more_matches = match &str_len {
                    FheStringLen::Padding(enc) => sk.scalar_lt_parallelized(enc, current_iteration),
                    FheStringLen::NoPadding(clear) => {
                        sk.create_trivial_boolean_block(*clear < current_iteration as usize)
                    }
                };

                match &from_is_empty {
                    FheStringIsEmpty::Padding(enc) => sk.boolean_bitand(&no_more_matches, enc),
                    FheStringIsEmpty::NoPadding(clear) => {
                        let trivial = sk.create_trivial_boolean_block(*clear);
                        sk.boolean_bitand(&no_more_matches, &trivial)
                    }
                }
            },
            || enc_n.map(|n| sk.scalar_le_parallelized(n, current_iteration)),
        );

        if let Some(exceeded) = enc_n_is_exceeded {
            sk.boolean_bitor_assign(&mut no_more_matches, &exceeded);
        }

        no_more_matches
    }

    /// Returns a new encrypted string with a specified number of non-overlapping occurrences of a
    /// pattern (either encrypted or clear) replaced by another specified encrypted pattern.
    ///
    /// The number of replacements to perform is specified by a `UIntArg`, which can be either
    /// `Clear` or `Enc`. In the `Clear` case, the function uses a plain `u16` value for the count.
    /// In the `Enc` case, the count is an encrypted `u16` value, encrypted with `ck.encrypt_u16`.
    ///
    /// If the pattern to be replaced is not found or the count is zero, returns the original
    /// encrypted string unmodified.
    ///
    /// The pattern to search for can be either `GenericPatternRef::Clear` for a clear string or
    /// `GenericPatternRef::Enc` for an encrypted string, while the replacement pattern is always
    /// encrypted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tfhe::integer::{ClientKey, ServerKey};
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    /// use tfhe::strings::ciphertext::{FheString, GenericPattern, UIntArg};
    ///
    /// let ck = ClientKey::new(PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128);
    /// let sk = ServerKey::new_radix_server_key(&ck);
    /// let ck = tfhe::strings::ClientKey::new(ck);
    /// let sk = tfhe::strings::ServerKey::new(sk);
    /// let (s, from, to) = ("hello", "l", "r");
    ///
    /// let enc_s = FheString::new(&ck, s, None);
    /// let enc_from = GenericPattern::Enc(FheString::new(&ck, from, None));
    /// let enc_to = FheString::new(&ck, to, None);
    ///
    /// // Using Clear count
    /// let clear_count = UIntArg::Clear(1);
    /// let result_clear = sk.replacen(&enc_s, enc_from.as_ref(), &enc_to, &clear_count);
    /// let replaced_clear = ck.decrypt_ascii(&result_clear);
    ///
    /// assert_eq!(replaced_clear, "herlo");
    ///
    /// // Using Encrypted count
    /// let max = 1; // Restricts the range of enc_n to 0..=max
    /// let enc_n = ck.encrypt_u16(1, Some(max));
    /// let enc_count = UIntArg::Enc(enc_n);
    /// let result_enc = sk.replacen(&enc_s, enc_from.as_ref(), &enc_to, &enc_count);
    /// let replaced_enc = ck.decrypt_ascii(&result_enc);
    ///
    /// assert_eq!(replaced_enc, "herlo");
    /// ```
    pub fn replacen(
        &self,
        str: &FheString,
        from: GenericPatternRef<'_>,
        to: &FheString,
        count: &UIntArg,
    ) -> FheString {
        let sk = self.inner();

        let mut result = str.clone();

        if matches!(count, UIntArg::Clear(0)) {
            return result;
        }

        let trivial_or_enc_from = match from {
            GenericPatternRef::Clear(from) => FheString::trivial(self, from.str()),
            GenericPatternRef::Enc(from) => from.clone(),
        };

        match self.length_checks(str, &trivial_or_enc_from) {
            IsMatch::Clear(false) => return result,

            IsMatch::Clear(true) => {
                // If `from` is empty and str too, there's only one match and one replacement
                if str.is_empty() {
                    if let UIntArg::Clear(_) = count {
                        return to.clone();
                    }

                    // We have to take into account that encrypted n could be 0
                    if let UIntArg::Enc(enc_n) = count {
                        let n_is_zero = sk.scalar_eq_parallelized(enc_n.cipher(), 0);

                        let mut re = self.conditional_string(&n_is_zero, &result, to);

                        // When result or to are empty we get padding via the conditional_string
                        // (pad_ciphertexts_lsb). And the condition result may or may not have
                        // padding in this case.
                        re.append_null(self);
                        return re;
                    }
                }
            }
            // This happens when str is empty, so it's again one replacement if there's match or
            // if there isn't we return the str
            IsMatch::Cipher(val) => {
                if let UIntArg::Clear(_) = count {
                    return self.conditional_string(&val, to, str);
                }

                if let UIntArg::Enc(enc_n) = count {
                    let n_not_zero = sk.scalar_ne_parallelized(enc_n.cipher(), 0);
                    let and_val = sk.boolean_bitand(&n_not_zero, &val);

                    let mut re = self.conditional_string(&and_val, to, str);

                    // When result or to are empty we get padding via the conditional_string
                    // (pad_ciphertexts_lsb). And the condition result may or may not have
                    // padding in this case.
                    re.append_null(self);
                    return re;
                }
            }
            _ => (),
        }

        match count {
            UIntArg::Clear(n) => {
                let max = max_matches(str, &trivial_or_enc_from);

                // If n > max number of matches we use that max to avoid unnecessary iterations
                let iterations = if *n > max { max } else { *n };

                self.replace_n_times(iterations, &mut result, from, to, None);
            }

            UIntArg::Enc(enc_n) => {
                // As we don't know the number n we perform the maximum number of iterations
                let max = (enc_n.max().unwrap_or(u16::MAX) as usize).min(result.len() + 1) as u16;

                self.replace_n_times(max, &mut result, from, to, Some(enc_n.cipher()));
            }
        }

        result
    }

    /// Returns a new encrypted string with all non-overlapping occurrences of a pattern (either
    /// encrypted or clear) replaced by another specified encrypted pattern.
    ///
    /// If the pattern to be replaced is not found, returns the original encrypted string
    /// unmodified.
    ///
    /// The pattern to search for can be either `GenericPatternRef::Clear` for a clear string or
    /// `GenericPatternRef::Enc` for an encrypted string, while the replacement pattern is always
    /// encrypted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tfhe::integer::{ClientKey, ServerKey};
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128;
    /// use tfhe::strings::ciphertext::{ClearString, FheString, GenericPattern};
    ///
    /// let ck = ClientKey::new(PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128);
    /// let sk = ServerKey::new_radix_server_key(&ck);
    /// let ck = tfhe::strings::ClientKey::new(ck);
    /// let sk = tfhe::strings::ServerKey::new(sk);
    /// let (s, from, to) = ("hi", "i", "o");
    ///
    /// let enc_s = FheString::new(&ck, s, None);
    /// let enc_from = GenericPattern::Enc(FheString::new(&ck, from, None));
    /// let enc_to = FheString::new(&ck, to, None);
    ///
    /// let result = sk.replace(&enc_s, enc_from.as_ref(), &enc_to);
    /// let replaced = ck.decrypt_ascii(&result);
    ///
    /// assert_eq!(replaced, "ho"); // "i" is replaced by "o" in "hi"
    ///
    /// let clear_from_not_found = GenericPattern::Clear(ClearString::new(String::from("x")));
    /// let result_no_change = sk.replace(&enc_s, clear_from_not_found.as_ref(), &enc_to);
    /// let not_replaced = ck.decrypt_ascii(&result_no_change);
    ///
    /// assert_eq!(not_replaced, "hi"); // No match, original string returned
    /// ```
    pub fn replace(
        &self,
        str: &FheString,
        from: GenericPatternRef<'_>,
        to: &FheString,
    ) -> FheString {
        let mut result = str.clone();
        let trivial_or_enc_from = match from {
            GenericPatternRef::Clear(from) => FheString::trivial(self, from.str()),
            GenericPatternRef::Enc(from) => from.clone(),
        };

        match self.length_checks(str, &trivial_or_enc_from) {
            IsMatch::Clear(false) => return result,
            IsMatch::Clear(true) => {
                // If `from` is empty and str too, there's only one match and one replacement
                if str.is_empty() {
                    return to.clone();
                }
            }
            // This happens when str is empty, so it's again one replacement if there's match or
            // if there isn't we return the str
            IsMatch::Cipher(val) => return self.conditional_string(&val, to, str),
            _ => (),
        }

        let max = max_matches(str, &trivial_or_enc_from);

        self.replace_n_times(max, &mut result, from, to, None);

        result
    }
}

fn max_matches(str: &FheString, pat: &FheString) -> u16 {
    let str_len = str.len() - if str.is_padded() { 1 } else { 0 };

    if pat.is_padded() {
        // Max number of matches is str_len + 1 when pattern is empty
        (str_len + 1).try_into().expect("str should be shorter")
    } else {
        // If we know the actual `from` length, the max number of matches can be computed as
        // str_len - pat_len + 1. For instance "xx" matches "xxxx" at most 4 - 2 + 1 = 3 times.
        // This works as long as str_len >= pat_len (guaranteed due to the outer length checks)
        let pat_len = pat.len() as u16;
        str_len as u16 - pat_len + 1
    }
}
