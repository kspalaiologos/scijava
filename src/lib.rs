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

use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jstring, jlong, jint, jobject, jboolean};

use rug::ops::Pow;
use rug::Integer;

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
    let obj = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[ptr.into()]).unwrap();
    obj.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_fromString(env: JNIEnv, _class: JClass, s: JString) -> jobject {
    let s: String = env.get_string(s).unwrap().into();
    let n = Integer::from_str_radix(&s, 10).unwrap();
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[ptr.into()]).unwrap();
    obj.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_reassignString(env: JNIEnv, _class: JClass, ptr: jlong, s: JString) {
    let s: String = env.get_string(s).unwrap().into();
    let a = ptr as *mut Integer;
    unsafe { *a = Integer::from_str_radix(&s, 10).unwrap(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_fromStringRadix(env: JNIEnv, _class: JClass, s: JString, radix: jint) -> jobject {
    let s: String = env.get_string(s).unwrap().into();
    let n = Integer::from_str_radix(&s, radix).unwrap();
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciInteger", "(J)V", &[ptr.into()]).unwrap();
    obj.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_toString(env: JNIEnv, _class: JClass, ptr: jlong) -> jstring {
    let ptr = ptr as *mut Integer;
    let n = unsafe { &*ptr };
    let s = n.to_string();
    env.new_string(s).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_toStringRadix(env: JNIEnv, _class: JClass, ptr: jlong, radix: jint) -> jstring {
    let ptr = ptr as *mut Integer;
    let n = unsafe { &*ptr };
    let s = n.to_string_radix(radix);
    env.new_string(s).unwrap().into_raw()
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
pub extern "system" fn Java_palaiologos_scijava_SciInteger_div(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    *dest = (a / b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_rem(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let b = b as *mut Integer;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
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
pub extern "system" fn Java_palaiologos_scijava_SciInteger_factorial(_env: JNIEnv, _class: JClass, a: jlong, b: jint) {
    let a = a as *mut Integer;
    let a = unsafe { &mut *a };
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
    *dest = a.shl(b).into();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciInteger_shr(_env: JNIEnv, _class: JClass, dest: jlong, a: jlong, b: jint) {
    let dest = dest as *mut Integer;
    let a = a as *mut Integer;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.shr(b).into();
}
