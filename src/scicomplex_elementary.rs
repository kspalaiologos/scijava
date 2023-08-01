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
use rug::{Float, Complex, Integer, Rational};
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_free(_env: JNIEnv, _class: JClass, ptr: jlong) {
    let ptr = ptr as *mut Complex;
    unsafe { ptr.drop_in_place(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_fromInteger(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jint) -> jobject {
    let n = Complex::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciComplex", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_fromSciInteger(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jlong) -> jobject {
    let n: &Integer = unsafe { &*(n as *const Integer) };
    let n = Complex::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciComplex", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_fromSciRational(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jlong) -> jobject {
    let n: &Rational = unsafe { &*(n as *const Rational) };
    let n = Complex::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciComplex", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_fromSciFloat(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, n: jlong) -> jobject {
    let n: &Float = unsafe { &*(n as *const Float) };
    let n = Complex::with_val(precision as u32, n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciComplex", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_toString(
        env: JNIEnv, _class: JClass, ptr: jlong) -> jstring {
    let ptr = ptr as *mut Complex;
    let n = unsafe { &*ptr };
    let s = n.to_string();
    let s = env.new_string(s).unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_fromString(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, s: JString) -> jobject {
    let s = env.get_string(s);
    let s: String = match s {
        Ok(s) => s.into(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            return JObject::null().into_raw();
        }
    };
    let n = Complex::parse(s);
    let n = match n {
        Ok(n) => Complex::with_val(precision as u32, n),
        Err(_) => {
            let _ = env.throw(("java/lang/NumberFormatException", "Failed to parse string."));
            return JObject::null().into_raw();
        }
    };
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciComplex", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_agm(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_add(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_sub(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_mul(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_div(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_mod(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_sqrt(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_norm(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.norm_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.norm_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_cbrt(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_imag(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) -> jobject {
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let ptr = Box::into_raw(Box::new(a.imag().clone()));
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_real(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) -> jobject {
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let ptr = Box::into_raw(Box::new(a.real().clone()));
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_neg(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_abs(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_arg(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.arg_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.arg_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_conj(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.conj_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.conj_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_factorial(
        env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, dest: jlong, a: jint) {
    let dest = dest as *mut Complex;
    let dest = unsafe { &mut *dest };
    if a < 0 {
        let _ = env.throw(("java/lang/IllegalArgumentException", "Factorial of negative number"));
        return;
    }
    *dest = Complex::with_val(precision as u32, Complex::factorial(a as u32));
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_eq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Complex;
    let b = b as *mut Complex;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a == b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_neq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Complex;
    let b = b as *mut Complex;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a != b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_compare(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Complex;
    let b = b as *mut Complex;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    a.total_cmp(b) as jint
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_isFinite(
        _env: JNIEnv, _class: JClass, ptr: jlong) -> jboolean {
    let ptr = ptr as *mut Complex;
    let n = unsafe { &*ptr };
    n.is_finite() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_copy(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_getMathContext(
        env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let ptr = ptr as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_exp(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_exp2(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_exp10(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_ln(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_hypot(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_log2(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_log10(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_isInf(
        _env: JNIEnv, _class: JClass, a: jlong) -> jboolean {
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    a.is_infinite() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_isNaN(
        _env: JNIEnv, _class: JClass, a: jlong) -> jboolean {
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    a.is_nan() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_drop_caches(
        _env: JNIEnv, _class: JClass) {
    rug::complex::free_cache(FreeCache::All)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_root(
        env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jint)  {
    let a = unsafe { &*(a as *const Complex) };
    let dest = unsafe { &mut *(dest as *mut Complex) };
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_log(
        env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, k: jlong)  {
    let a = unsafe { &*(a as *const Complex) };
    let dest = unsafe { &mut *(dest as *mut Complex) };
    let k = unsafe { &*(k as *const Complex) };
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
pub extern "system" fn Java_palaiologos_scijava_SciComplex_recip(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciComplex_pow(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Complex;
    let a = a as *mut Complex;
    let b = b as *mut Complex;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.pow_assign(b);
}
