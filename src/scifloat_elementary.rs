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

use std::hint::unreachable_unchecked;
use std::ops::DivAssign;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JObject, JValue};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jstring, jlong, jint, jobject, jboolean};
use rug::rand::RandState;
use rug::{Float, Integer, Rational};
use rug::float::{Round, FreeCache, Constant};
use rug::ops::{NegAssign, PowAssign};

pub fn xlat_rounding(mode: jint) -> Round {
    match mode {
        0 => Round::Up,
        1 => Round::Down,
        2 => Round::Nearest,
        3 => Round::Zero,
        _ => unsafe { unreachable_unchecked() } // lol
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_free(_env: JNIEnv, _class: JClass, ptr: jlong) {
    let ptr = ptr as *mut Float;
    unsafe { ptr.drop_in_place(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_fromInteger(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jint) -> jobject {
    let n = Float::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_fromSciInteger(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jlong) -> jobject {
    let n: &Integer = unsafe { &*(n as *const Integer) };
    let n = Float::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_fromSciRational(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jlong) -> jobject {
    let n: &Rational = unsafe { &*(n as *const Rational) };
    let n = Float::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_toString(
        env: JNIEnv, _class: JClass, ptr: jlong) -> jstring {
    let ptr = ptr as *mut Float;
    let n = unsafe { &*ptr };
    let s = n.to_string();
    let s = env.new_string(s).unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_fromString(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, s: JString) -> jobject {
    let s = env.get_string(s);
    let s: String = match s {
        Ok(s) => s.into(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            return JObject::null().into_raw();
        }
    };
    let n = Float::parse(s);
    let n = match n {
        Ok(n) => Float::with_val(precision as u32, n),
        Err(_) => {
            let _ = env.throw(("java/lang/NumberFormatException", "Failed to parse string."));
            return JObject::null().into_raw();
        }
    };
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_agm(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.agm_mut(b);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.agm_mut(b);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_add(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        *dest += b;
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        *dest += b;
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sub(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        *dest -= b;
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        *dest -= b;
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_mul(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        *dest *= b;
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        *dest *= b;
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_div(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        *dest /= b;
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        *dest /= b;
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_mod(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        *dest %= b;
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        *dest %= b;
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sqrt(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sqrt_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sqrt_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cbrt(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cbrt_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cbrt_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_neg(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.neg_assign();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.neg_assign();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_abs(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.abs_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.abs_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_factorial(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, dest: jlong, a: jint) {
    let dest = dest as *mut Float;
    let dest = unsafe { &mut *dest };
    if a < 0 {
        let _ = env.throw(("java/lang/IllegalArgumentException", "Factorial of negative number"));
        return;
    }
    *dest = Float::with_val(precision as u32, Float::factorial(a as u32));
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_lt(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a < b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_lte(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a <= b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gt(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a > b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gte(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a >= b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_eq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a == b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_neq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a != b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_compare(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    a.total_cmp(b) as jint
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_isFinite(
        _env: JNIEnv, _class: JClass, ptr: jlong) -> jboolean {
    let ptr = ptr as *mut Float;
    let n = unsafe { &*ptr };
    n.is_finite() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_copy(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_getMathContext(
        env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let ptr = ptr as *mut Float;
    let n = unsafe { &*ptr };
    let precision = n.prec() as jint;
    let mc = env.new_object("palaiologos/scijava/MathContext", "(II)V", &[JValue::Int(precision), JValue::Int(0)]);
    match mc {
        Ok(mc) => mc.into_raw(),
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", format!("Error creating MathContext: {}", e));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_clamp(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, min: jlong, max: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let min = min as *mut Float;
    let max = max as *mut Float;
    let a = unsafe { &*a };
    let min = unsafe { &*min };
    let max = unsafe { &*max };
    let dest = unsafe { &mut *dest };
    if min > max {
        let _ = _env.throw_new("java/lang/ArithmeticException", "Call to clamp with min > max");
        return;
    }
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.clamp_mut(min, max);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.clamp_mut(min, max);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ceil(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.ceil_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.ceil_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_floor(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.floor_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.floor_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_exp(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.exp_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.exp_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_exp2(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.exp2_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.exp2_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_exp10(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.exp10_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.exp10_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ln(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.ln_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.ln_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_fract(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.fract_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.fract_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_hypot(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.hypot_mut(b);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.hypot_mut(b);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ldexp(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jint, b: jint) -> jobject {
    let n = Float::with_val(precision as u32, Float::i_exp(a, b));
    let n = Box::into_raw(Box::new(n));
    let n = n as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(n)]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Error allocating SciFloat."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_log2(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.log2_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.log2_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_log10(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.log10_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.log10_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_chop(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, eps: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let eps = eps as *mut Float;
    let a = unsafe { &*a };
    let eps = unsafe { &*eps };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    let mut fract = Float::with_val(precision as u32, a.fract_ref());
    if fract.total_cmp(eps).is_le() {
        dest.trunc_mut();
        return;
    }
    fract -= 1;
    if fract.total_cmp(eps).is_le() {
        dest.ceil_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_isInf(
        _env: JNIEnv, _class: JClass, a: jlong) -> jboolean {
    let a = a as *mut Float;
    let a = unsafe { &*a };
    a.is_infinite() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_isNaN(
        _env: JNIEnv, _class: JClass, a: jlong) -> jboolean {
    let a = a as *mut Float;
    let a = unsafe { &*a };
    a.is_nan() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_random(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, randptr: jlong) -> jobject {
    let randptr = randptr as *mut RandState;
    let randptr = unsafe { &mut *randptr };
    let a = Float::with_val(precision as u32, Float::random_cont(randptr));
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_drop_caches(
        _env: JNIEnv, _class: JClass) {
    rug::float::free_cache(FreeCache::All)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_pi(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = Float::with_val(precision as u32, Constant::Pi);
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_euler_gamma(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = Float::with_val(precision as u32, Constant::Euler);
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_degree(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = Float::with_val(precision as u32, Constant::Pi) / 180;
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_e(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = Float::with_val(precision as u32, 1).exp();
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_phi(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = (Float::with_val(precision as u32, 5).sqrt() + 1) / 2;
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_catalan(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = Float::with_val(precision as u32, Constant::Catalan);
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_apery(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let a = Float::with_val(precision as u32, 3).zeta();
    let ptr = Box::into_raw(Box::new(a)) as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(ptr)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to create SciFloat"));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_root(
        env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jint)  {
    let a = unsafe { &*(a as *const Float) };
    let dest = unsafe { &mut *(dest as *mut Float) };
    if n <= 0 {
        let _ = env.throw(("java/lang/IllegalArgumentException", "n must be positive"));
        return;
    }
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.root_mut(n as u32);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.root_mut(n as u32);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_log(
        env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, k: jlong)  {
    let a = unsafe { &*(a as *const Float) };
    let dest = unsafe { &mut *(dest as *mut Float) };
    let k = unsafe { &*(k as *const Float) };
    if k <= &0 {
        let _ = env.throw(("java/lang/IllegalArgumentException", "k must be positive"));
        return;
    }
    if a.prec() == precision as u32 {
        *dest = a.clone();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.ln_mut();
    let lnk = k.clone().ln();
    dest.div_assign(&lnk);
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_degrees(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= 180 / Float::with_val(precision as u32, Constant::Pi);
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_radians(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= Float::with_val(precision as u32, Constant::Pi) / 180;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_recip(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_pow(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.pow_assign(b);
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_intValue(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, a: jlong) -> jint {
    let a = unsafe { &*(a as *const Float) };
    let mut a = a.clone();
    if a.prec() != precision as u32 {
        a.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    a.to_i32_saturating().unwrap_or(0) as jint
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_scale(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jint) {
    let a = unsafe { &*(a as *const Float) };
    let dest = unsafe { &mut *(dest as *mut Float) };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= b;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_unscale(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jint) {
    let a = unsafe { &*(a as *const Float) };
    let dest = unsafe { &mut *(dest as *mut Float) };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest /= b;
}
