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
use rug::{Float, Integer};
use rug::float::Round;
use rug::ops::NegAssign;

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
