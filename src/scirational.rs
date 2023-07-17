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

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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

use rug::ops::Pow;
use rug::{Rational, Float, Integer, Assign};

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_free(_env: JNIEnv, _class: JClass, ptr: jlong) {
    let ptr = ptr as *mut Rational;
    unsafe { ptr.drop_in_place(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fromInteger(env: JNIEnv, _class: JClass, n: jint) -> jobject {
    let n = Rational::from(n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciRational", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fromSciFloat(env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let ptr = ptr as *mut Float;
    let n = unsafe { &*ptr };
    let n = match n.to_rational() {
        Some(n) => n,
        None => {
            let _ = env.throw(("java/lang/ArithmeticException", "Failed to convert Float to Rational, (Float not finite?)."));
            return JObject::null().into_raw();
        }
    };
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciRational", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fromSciInteger(env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let ptr = ptr as *mut Integer;
    let n = unsafe { &*ptr };
    let n = Rational::from(n);
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciRational", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_num(env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let ptr = ptr as *mut Rational;
    let n = unsafe { &*ptr };
    let n = n.numer();
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
pub extern "system" fn Java_palaiologos_scijava_SciRational_den(env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let ptr = ptr as *mut Rational;
    let n = unsafe { &*ptr };
    let n = n.denom();
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
pub extern "system" fn Java_palaiologos_scijava_SciRational_fromString(env: JNIEnv, _class: JClass, s: JString) -> jobject {
    let s = env.get_string(s);
    let s: String = match s {
        Ok(s) => s.into(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            return JObject::null().into_raw();
        }
    };
    let n = Rational::from_str_radix(&s, 10);
    let n = match n {
        Ok(n) => n,
        Err(_) => {
            let _ = env.throw(("java/lang/ArithmeticException", "Failed to parse the numeric string."));
            return JObject::null().into_raw();
        }
    };
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciRational", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fromStringRadix(env: JNIEnv, _class: JClass, s: JString, radix: jint) -> jobject {
    let s = env.get_string(s);
    let s: String = match s {
        Ok(s) => s.into(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            return JObject::null().into_raw();
        }
    };

    if !(2..=36).contains(&radix) {
        let _ = env.throw(("java/lang/IllegalArgumentException", "Radix must be between 2 and 36."));
        return JObject::null().into_raw();
    }

    let n = Rational::from_str_radix(&s, radix);
    let n = match n {
        Ok(n) => n,
        Err(_) => {
            let _ = env.throw(("java/lang/ArithmeticException", "Failed to parse the numeric string."));
            return JObject::null().into_raw();
        }
    };

    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciRational", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_toString(env: JNIEnv, _class: JClass, ptr: jlong) -> jstring {
    let ptr = ptr as *mut Rational;
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
pub extern "system" fn Java_palaiologos_scijava_SciRational_toStringRadix(env: JNIEnv, _class: JClass, ptr: jlong, radix: jint) -> jstring {
    let ptr = ptr as *mut Rational;
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
pub extern "system" fn Java_palaiologos_scijava_SciRational_add(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a + b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_sub(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a - b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_hash(env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    let ptr = ptr as *mut Rational;
    let n = unsafe { &*ptr };
    let mut hasher = DefaultHasher::new();
    n.hash(&mut hasher);
    hasher.finish() as jlong
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_lt(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a < b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_lte(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a <= b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_gt(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a > b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_gte(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a >= b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_eq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a == b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_neq(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jboolean {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    (a != b) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_compare(_env: JNIEnv, _class: JClass, a: jlong, b: jlong) -> jint {
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    a.cmp(b) as jint
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_copy(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
}

// --

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_recip(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.recip_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_ceil(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.ceil_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_floor(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.floor_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fract_ceil(_env: JNIEnv, _class: JClass, dest: jlong, dest2: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let dest2 = dest2 as *mut Integer;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    let dest2 = unsafe { &mut *dest2 };
    (dest, dest2).assign(a.fract_ceil_ref());
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fract_floor(_env: JNIEnv, _class: JClass, dest: jlong, dest2: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let dest2 = dest2 as *mut Integer;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    let dest2 = unsafe { &mut *dest2 };
    (dest, dest2).assign(a.fract_floor_ref());
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fract_round(_env: JNIEnv, _class: JClass, dest: jlong, dest2: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let dest2 = dest2 as *mut Integer;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    let dest2 = unsafe { &mut *dest2 };
    (dest, dest2).assign(a.fract_round_ref());
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_fract_trunc(_env: JNIEnv, _class: JClass, dest: jlong, dest2: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let dest2 = dest2 as *mut Integer;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    let dest2 = unsafe { &mut *dest2 };
    (dest, dest2).assign(a.fract_trunc_ref());
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_rem_ceil(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.rem_ceil_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_rem_floor(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.rem_floor_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_rem_round(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.rem_round_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_rem_trunc(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.rem_trunc_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_round(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.round_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_signum(_env: JNIEnv, _class: JClass, a: jlong) -> jlong {
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let a: Integer = a.signum_ref().into();
    a.to_i64().unwrap()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_square(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.square_ref().into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_mul(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a * b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_div(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let b = b as *mut Rational;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a / b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_neg(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = (-a).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciRational_pow(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, exp: jint) {
    let dest = dest as *mut Rational;
    let a = a as *mut Rational;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.pow(exp).into();
}

