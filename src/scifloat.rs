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

use std::cmp::max;
use std::f64::consts;
use std::hint::unreachable_unchecked;
use std::ops::DivAssign;

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
use jni::sys::{jstring, jlong, jint, jobject, jboolean, jfloat};
use rug::rand::RandState;
use rug::{Float, Integer};
use rug::float::{Round, FreeCache, Constant, Special};
use rug::ops::{NegAssign, Pow};

fn xlat_rounding(mode: jint) -> Round {
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_digamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.digamma_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.digamma_mut();
    }
}

// Implement Ei
#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ei(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.eint_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.eint_mut();
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sin(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sin_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sin_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cos(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cos_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cos_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_tan(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.tan_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.tan_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sec(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sec_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sec_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_csc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.csc_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.csc_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cot(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cot_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cot_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asin(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.asin_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.asin_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asin_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.asin_mut();
    } else {
        a.set_prec(precision as u32);
        a.asin_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acos(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.acos_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.acos_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acos_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.acos_mut();
    } else {
        a.set_prec(precision as u32);
        a.acos_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atan(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.atan_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.atan_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atan_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.atan_mut();
    } else {
        a.set_prec(precision as u32);
        a.atan_mut();
    }
}

// hyperbolic functions

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sinh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sinh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sinh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cosh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cosh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cosh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_tanh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.tanh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.tanh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asinh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.asinh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.asinh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acosh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.acosh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.acosh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atanh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.atanh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.atanh_mut();
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_erf(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.erf_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.erf_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_erfc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.erfc_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.erfc_mut();
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.gamma_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.gamma_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gamma_inc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, x: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let x = x as *mut Float;
    let a = unsafe { &*a };
    let x = unsafe { &*x };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.gamma_inc_mut(x);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.gamma_inc_mut(x);
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_j0(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.j0_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.j0_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_j1(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.j1_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.j1_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_jn(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jint) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.jn_mut(n);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.jn_mut(n);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_li2(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.li2_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.li2_mut();
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

// lambert w

fn lambertw_special(prec: u32, z: Float, k: f32) -> Float {
    if z.is_zero() {
        if k == 0.0 {
            return z;
        }
        return Float::with_val(prec, Special::NegInfinity);
    }
    if z.is_infinite() && k == 0.0 {
        return z;
    }
    z.ln()
}

fn lambertw_approx_hybrid(prec: u32, z: Float, k: f32) -> Float {
    if k == 0.0 {
        if z < -4.0 && z < 4.0 && -1.0 < z && z < 2.5 {
            // Taylor series near -1
            if z < -0.5 {
                return Float::with_val(prec, Special::Nan);
            }
            // return real type
            let r = Float::with_val(prec, -0.367879441171442);
            let mut z = z;
            if z > r {
                z = r.clone()
            }
            // Singularity near -1/e
            if z < -0.2 {
                let zmr = z - r;
                let zmr_r = zmr.clone().sqrt();
                return Float::with_val(prec, -1) + Float::with_val(prec, 2.33164398159712) * zmr_r - Float::with_val(prec, 1.81218788563936) * zmr;
            }
            // Taylor series near 0
            if z < 0.5 {
                return z;
            }
            // Simple linear approximation
            return Float::with_val(prec, 0.2) + Float::with_val(prec, 0.3) * z;
        }
        
        let l1 = z.ln();
        let l2 = l1.clone().ln();
        let a = l1.clone() - &l2;
        let b = l2.clone() / &l1;
        return a + b + l2.clone() * (l2 - 2) / (2 * l1.pow(2));
    } else if k == -1.0 {
        // return real type
        let r = Float::with_val(prec, -0.367879441171442);
        let mut z = z;
        if r < z && z < 0.0 {
            z = r.clone();
        }
        if z < 0.1 && -0.6 < z && z < -0.2 {
            let zmr = z - r;
            let zmr_r = zmr.clone().sqrt();
            return Float::with_val(prec, -1) - Float::with_val(prec, 2.33164398159712) * zmr_r - Float::with_val(prec, 1.81218788563936) * zmr;
        } else if (-0.2..0.0).contains(&z) {
            let l1 = (-z).ln();
            let ml1 = -l1.clone();
            return l1 - ml1.ln();
        } else {
            return Float::with_val(prec, Special::Nan);
        }
    }
    Float::with_val(prec, Special::Nan)
}

// return the logarithmic magnitude of a number, i.e. m such that |x| <= 2^m. m can be zero or infinity.
fn mag(x: &Float) -> f64 {
    if x.is_zero() {
        return f64::NEG_INFINITY;
    }
    if x.is_infinite() {
        return f64::INFINITY;
    }
    if x.is_sign_negative() {
        let x = -x.clone();
        let mut m = x.log2();
        m.floor_mut();
        -m.to_f64()
    } else {
        let mut m = x.clone().log2();
        m.floor_mut();
        m.to_f64()
    }
}

fn lambertw_series(prec: u32, z: Float, k: f32, tol: Float) -> (Float, bool) {
    let magz = mag(&z);
    if (-10.0..900.0).contains(&magz) && (-1000.0..1000.0).contains(&k) {
        // Near the branch point at -1/e
        if magz < 1.0 && (0.36787944117144..0.41787944117144).contains(&(z.clone() + 0.36787944117144)) {
            if k == 0.0 || k == -1.0 {
                let delta = Float::with_val(prec, -1).exp() + &z;
                let cancellation = -mag(&delta);
                let fc: Float = Float::with_val(prec, consts::E) * z + 1.0;
                let mut p = Float::with_val(prec + cancellation as u32, 2.0) * fc.sqrt();
                let mut u = vec![Float::with_val(prec, -1.0), Float::with_val(prec, 1.0)];
                let mut a = vec![Float::with_val(prec, 2.0), Float::with_val(prec, -1.0)];
                if k != 0.0 {
                    p = -p;
                }
                let mut s = Float::with_val(prec + cancellation as u32 / 2, 0.0);
                // The series converges, so we could use it directly, but unless
                // *extremely* close, it is better to just use the first few
                // terms to get a good approximation for the iteration
                for l in 2..max(2, cancellation as usize) {
                    if u.len() <= l {
                        a.push(Float::with_val(prec, 0.0));
                        for j in 0..l {
                            a[l] += u[j].clone() * &u[l + 1 - j];
                        }

                        u.push(Float::with_val(prec, 0.0));
                        u[l] = (l - 1) * (u[l - 2].clone() / 2.0 + a[l - 2].clone() / 4.0) / (l + 1) - a[l].clone() / 2.0 - u[l - 1].clone() / (l + 1);
                    }
                    let term = u[l].clone() * p.clone().pow(l as u32);
                    s += &term;
                    if mag(&term) < -tol.clone() {
                        return (s, true);
                    }
                }
                return (s, false);
            }
            if k == 0.0 || k == -1.0 {
                return (lambertw_approx_hybrid(prec, z, k), false);
            }
        }
        if k == 0.0 {
            if magz < -1.0 {
                return (z.clone() * (1.0 - z), false);
            }
            let L1 = z.ln();
            let L2 = L1.clone().ln();
            return (L1.clone() - &L2 + L2.clone() / &L1 + L2.clone() * (L2 - 2.0) / (2.0 * L1.pow(2)), false);
        } else if k == -1.0 && (-0.36787944117144..0.0).contains(&z) {
            let L1 = (-z).ln();
            return (L1.clone() - (-L1).ln(), false);
        }
    }
    (Float::with_val(prec, Special::Nan), false)
}

fn lambertw(env: JNIEnv, prec: u32, z: Float, k: f32) -> Option<Float> {
    if !z.is_normal() {
        return Some(lambertw_special(prec, z, k));
    }
    let wp = prec + 30;
    let tol = prec - 5;
    let (mut w, done) = lambertw_series(wp, z.clone(), k, Float::with_val(wp, 10.0).pow(tol).recip());
    if !done {
        // Use Halley iteration to solve w*exp(w) = z
        let two = Float::with_val(wp, 2.0);
        let mut converged = false;
        for _ in 0..100 {
            let ew = w.clone().exp();
            let wew = w.clone() * &ew;
            let wewz = wew.clone() - &z;
            let wn = w.clone() - &wewz / (wew.clone() + &ew - (w.clone() + &two) * &wewz / (two.clone() * &w + &two));
            if mag(&(wn.clone() - &w)) <= mag(&wn) - tol as f64 {
                w = wn;
                converged = true;
                break;
            } else {
                w = wn;
            }
        }
        if !converged {
            // throw exception
            let _ = env.throw(("java/lang/ArithmeticException", "Lambert W iteration failed to converge."));
            return None;
        }
    }
    Some(w)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_lambertw(
        env: JNIEnv, _class: JClass, prec: u32, z: jlong, k: jint) -> jobject {
    let z = unsafe { &*(z as *const Float) };
    let lw = lambertw(env, prec, z.clone(), k as f32);
    let lw = match lw {
        Some(lw) => lw,
        None => return JObject::null().into_raw(),
    };
    let lw = Box::into_raw(Box::new(lw));
    let lw = lw as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(lw)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/ArithmeticException", "Lambert W iteration failed to converge."));
            JObject::null().into_raw()
        }
    }
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

// cospi, sinpi

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sinpi(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= Float::with_val(precision as u32, Constant::Pi);
    dest.sin_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cospi(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= Float::with_val(precision as u32, Constant::Pi);
    dest.cos_mut();
}
