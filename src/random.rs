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

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::JClass;

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlong, jint};
use rug::Integer;
use rug::rand::RandState;

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_Random_free(_env: JNIEnv, _class: JClass, ptr: jlong) {
    let ptr = ptr as *mut RandState;
    unsafe { ptr.drop_in_place(); }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_Random_newMersenneTwister(_env: JNIEnv, _class: JClass) -> jlong {
    let state = RandState::new_mersenne_twister();
    let ptr = Box::into_raw(Box::new(state));
    ptr as jlong
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_Random_range(env: JNIEnv, _class: JClass, ptr: jlong, max: jint) -> jlong {
    let state = unsafe { &mut *(ptr as *mut RandState) };
    if max < 1 {
        let _ = env.throw(("java/lang/IllegalArgumentException", "max must be positive and nonzero"));
        0
    } else {
        state.below(max as u32) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_Random_seed(_env: JNIEnv, _class: JClass, ptr: jlong, seed: jlong) {
    let state = unsafe { &mut *(ptr as *mut RandState) };
    let seed = unsafe {  &*(seed as *mut Integer) };
    state.seed(seed);
}
