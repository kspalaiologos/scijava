/*
    scijava
    Copyright (C) 2022 Kamila Szewczyk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JObject, JValue, JMap};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jstring, jlong, jint, jobject, jboolean};

use rug::integer::IsPrime;
use rug::ops::Pow;
use rug::{Integer, Complete};

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_free(_env: JNIEnv, _class: JClass, ptr: jlong) {
    let ptr = ptr as *mut Integer;
    unsafe { ptr.drop_in_place(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_fromInteger(env: JNIEnv, _class: JClass, n: jint) -> jobject {
    let n = Integer::from(n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_fromString(env: JNIEnv, _class: JClass, s: JString) -> jobject {
    let s = env.get_string(s);
    let s: String = match s {
        Ok(s) => s.into(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            return JObject::null().into_raw();
        }
    };
    let n = Integer::from_str_radix(&s, 10);
    let n = match n {
        Ok(n) => n,
        Err(_) => {
            let _ = env.throw(("java/lang/ArithmeticException", "Failed to parse the numeric string."));
            return JObject::null().into_raw();
        }
    };
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_toInteger(env: JNIEnv, _class: JClass, ptr: jlong) -> jint {
    let ptr = ptr as *mut Integer;
    let n = unsafe { &*ptr };
    match n.to_i32() {
        Some(n) => n,
        None => {
            let _ = env.throw(("java/lang/ArithmeticException", "Failed to convert the integer to a Java integer (overflow/underflow?)."));
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_fromStringRadix(env: JNIEnv, _class: JClass, s: JString, radix: jint) -> jobject {
    let s = env.get_string(s);
    let s: String = match s {
        Ok(s) => s.into(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            return JObject::null().into_raw();
        }
    };

    let n = Integer::from_str_radix(&s, radix);
    let n = match n {
        Ok(n) => n,
        Err(_) => {
            let _ = env.throw(("java/lang/ArithmeticException", "Failed to parse the numeric string."));
            return JObject::null().into_raw();
        }
    };

    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_toString(env: JNIEnv, _class: JClass, ptr: jlong) -> jstring {
    let ptr = ptr as *mut Integer;
    let n = unsafe { &*ptr };
    let s = n.to_string();
    match env.new_string(s) {
        Ok(s) => s.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate string."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_toStringRadix(env: JNIEnv, _class: JClass, ptr: jlong, radix: jint) -> jstring {
    let ptr = ptr as *mut Integer;
    let n = unsafe { &*ptr };
    if !(2..=36).contains(&radix) {
        let _ = env.throw(("java/lang/IllegalArgumentException", "Radix must be between 2 and 36."));
        return JObject::null().into_raw();
    }
    let s = n.to_string_radix(radix);
    match env.new_string(s) {
        Ok(s) => s.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate string."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_add(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a + b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_sub(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a - b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_mul(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a * b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_div(env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if b == &0 {
        let _ = env.throw(("java/lang/ArithmeticException", "Division by zero."));
        return;
    }
    *dest = (a / b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_rem(env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if b == &0 {
        let _ = env.throw(("java/lang/ArithmeticException", "Division by zero."));
        return;
    }
    *dest = (a % b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_pow(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = (a.pow(b as u32)).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_negate(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = (-a).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_abs(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.abs_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_gcd(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.gcd_ref(b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_lcm(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.lcm_ref(b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_factorial(env: JNIEnv, _class: JClass, a: jlong, b: jint) {
    let a = a as *mut Integer;
    let a = unsafe { &mut *a };
    if b < 0 {
        let _ = env.throw(("java/lang/ArithmeticException", "Factorial of negative number."));
        return;
    }
    *a = Integer::factorial(b as u32).into()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_signum(_env: JNIEnv, _class: JClass, dest:jlong, a: jlong) {
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = dest as *mut Integer;
    let dest = unsafe { &mut *dest };
    *dest = a.signum_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_lt(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a < b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_lte(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a <= b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_gt(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a > b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_gte(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a >= b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_eq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a == b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_neq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a != b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_compare(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    a.cmp(b) as jint
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_and(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.bitand(b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_or(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.bitor(b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_xor(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.bitxor(b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_not(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.not().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_shl(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if b < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to shl with negative shift");
        return;
    }
    *dest = a.shl(b as u32).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_shr(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if b < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to shr with negative shift");
        return;
    }
    *dest = a.shr(b as u32).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_setBit(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint, val: jboolean) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if b < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to setBit with negative index");
        return;
    }
    *dest = a.clone();
    dest.set_bit(b as u32, val != 0);
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_clearBit(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if b < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to clearBit with negative index");
        return;
    }
    *dest = a.clone();
    dest.set_bit(b as u32, false);
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_flipBit(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if b < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to flipBit with negative index");
        return;
    }
    *dest = a.clone();
    dest.toggle_bit(b as u32);
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_testBit(_env: JNIEnv, _class: JClass, a: jlong, b: jint) -> jboolean {
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    if b < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to testBit with negative index");
        return 0;
    }
    a.get_bit(b as u32) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_bitLength(_env: JNIEnv, _class: JClass, a: jlong) -> jint {
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    a.signed_bits() as jint
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_bitCount(_env: JNIEnv, _class: JClass, a: jlong) -> jint {
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    if a < &0 {
        a.count_ones().unwrap() as jint
    } else {
        a.count_zeros().unwrap() as jint
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_isPrime(_env: JNIEnv, _class: JClass, a: jlong, certainty: jint) -> jboolean {
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    if certainty < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to isPrime with negative certainty");
        return 0;
    }
    a.is_probably_prime(certainty as u32) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_nextPrime(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.next_prime_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_clamp(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, min: jlong, max: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let min = min as *mut Integer;
    let max = max as *mut Integer;
    let a = unsafe { &*a };
    let min = unsafe { &*min };
    let max = unsafe { &*max };
    let dest = unsafe { &mut *dest };
    *dest = a.clamp_ref(min, max).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_divmod(_env: JNIEnv, _class: JClass, destdiv: jlong, destmod: jlong, a: jlong, b: jlong) {
    let destdiv = destdiv as *mut Integer;
    let destmod = destmod as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let destdiv = unsafe { &mut *destdiv };
    let destmod = unsafe { &mut *destmod };
    if b == &0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to divmod with zero divisor");
        return;
    }
    let (div, modu) = a.div_rem_ref(b).complete();
    *destdiv = div;
    *destmod = modu;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_fibonacci(_env: JNIEnv, _class: JClass, dest: jlong, a: jint) {
    let dest = dest as *mut Integer;
    let dest = unsafe { &mut *dest };
    if a < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to fibonacci with negative index");
        return;
    }
    *dest = Integer::fibonacci(a as u32).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_lucas(_env: JNIEnv, _class: JClass, dest: jlong, a: jint) {
    let dest = dest as *mut Integer;
    let dest = unsafe { &mut *dest };
    if a < 0 {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to lucas with negative index");
        return;
    }
    *dest = Integer::lucas(a as u32).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_hamming(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    match a.hamming_dist(b) {
        Some(x) => x as jint,
        None => {
            let _ = _env.throw_new("java/lang/ArithmeticException", "Call to hamming with negative index");
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_sqrt(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.sqrt_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_square(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.square_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_legendre(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    a.legendre(b)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_jacobi(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    a.jacobi(b)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_copy(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
}

fn factor_using_pollard_rho(factors: &mut HashMap<Integer, Integer>, mut n: Integer, a: u64) {
    let mut x = Integer::from(2);
    let mut z = Integer::from(2);
    let mut y = Integer::from(2);
    let mut p = Integer::from(1);
    let mut k = 1;
    let mut l = 1;
    let mut t:Integer;
    'outer: while n != 1 {
        loop {
            t = (&x * &x).into();
            x = (t % &n) + a;
            t = (&z - &x).into();
            p *= &t;
            p %= &n;
            if k % 32 == 1 {
                t = p.gcd_ref(&n).into();
                if t != 1 {
                    // factor found.
                    loop {
                        t = (&y * &y).into();
                        y = (t % &n) + a;
                        t = (&z - &y).into();
                        t.gcd_mut(&n);
                        if t != 1 {
                            break;
                        }
                    }
                    n /= &t;
                    if t.is_probably_prime(50) == IsPrime::No {
                        factor_using_pollard_rho(factors, t, a + 1);
                    } else if factors.contains_key(&t) {
                        let val = factors.get_mut(&t).unwrap();
                        *val += 1;
                    } else {
                        factors.insert(t, Integer::from(1));
                    }
                    if n.is_probably_prime(50) != IsPrime::No {
                        if factors.contains_key(&n) {
                            let val = factors.get_mut(&n).unwrap();
                            *val += 1;
                        } else {
                            factors.insert(n.clone(), Integer::from(1));
                        }
                        return;
                    }
                    x %= &n;
                    z %= &n;
                    y %= &n;
                    continue 'outer;
                }
                y = x.clone();
            }
            k -= 1;
            if k == 0 {
                break;
            }
        }
        z = x.clone();
        k = l;
        l *= 2;
        for _i in 0..k {
            t = (&x * &x).into();
            x = (t % &n) + a;
        }
        y = x.clone();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_factor(env: JNIEnv, _class: JClass, dest: JObject, a: jlong) {
    let dest = JMap::from_env(&env, dest).unwrap();
    let a = a as *mut Integer;
    let mut a = unsafe { &*a }.clone();
    let mut factors: HashMap<Integer, Integer> = HashMap::new();
    
    if a < 0 {
        a = -a;
        // Push -1 to vector of SciIntegers destFactors and 1 to destPowers.
        factors.insert(Integer::from(-1), Integer::from(1));
    }

    if a == 0 {
        return;
    }

    factor_using_pollard_rho(&mut factors, a, 1);

    factors.iter().for_each(|(k, v)| {
        let key_ptr = Box::into_raw(Box::new(k.clone())) as jlong;
        let value_ptr = Box::into_raw(Box::new(v.clone())) as jlong;
        let key = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[JValue::Long(key_ptr)]).unwrap();
        let value = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[JValue::Long(value_ptr)]).unwrap();
        let _ = dest.put(key, value);
    });
}

// stub out functions.
macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        #[cfg(feature = "unwind-stubs")]
        pub extern "C" fn $name() -> ! { loop {} }
    };
}

stub!(_Unwind_Resume);
stub!(_Unwind_GetIP);
stub!(_Unwind_GetLanguageSpecificData);
stub!(_Unwind_GetRegionStart);
stub!(_Unwind_SetGR);
stub!(_Unwind_GetGR);
stub!(_Unwind_SetIP);
stub!(_Unwind_GetDataRelBase);
stub!(_Unwind_Backtrace);
stub!(_Unwind_GetIPInfo);
stub!(_Unwind_GetTextRelBase);
