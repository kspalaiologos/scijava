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
use jni::objects::{JClass, JObject};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlong, jint, jarray};
use rug::float::Constant;
use rug::ops::Pow;
use rug::{Integer, Float};
use rug::rand::RandState;

fn wrap_float(env: JNIEnv, n: Float) -> Option<JObject> {
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => Some(obj),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            None
        }
    }
}

fn wrap_nodepair(env: JNIEnv, x1: Float, x2: Float) -> Option<JObject> {
    let x1 = wrap_float(env, x1);
    let x2 = wrap_float(env, x2);
    let x1 = match x1 {
        Some(x1) => x1,
        None => return None
    };
    let x2 = match x2 {
        Some(x2) => x2,
        None => return None
    };
    match env.new_object_array(2, "palaiologos/scijava/SciFloat", JObject::null()) {
        Ok(x) => Some(unsafe { JObject::from_raw(x) }),
        Err(_) => {
            let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
            None
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_TanhSinhIntegrator_getNodes(
            env: JNIEnv, _class: JClass, precision: jint, degree: jint) -> jarray {
    let wp = precision as u32 + 30;
    let tol = Float::with_val(wp, Float::i_exp(1, -precision-10));
    let pi4 = Float::with_val(wp, Constant::Pi) / 4;
    let t0 = Float::with_val(wp, Float::i_exp(1, -degree));
    let mut node_id: i32 = 0;
    let mut nodes: jarray;
    let mut h: Float;
    if degree == 1 {
        nodes = match env.new_object_array(2 + 20*(2_i32.pow(degree as u32)), "[palaiologos/scijava/SciFloat", JObject::null()) {
            Ok(nodes) => nodes,
            Err(_) => {
                let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                return JObject::null().into_raw();
            }
        };
        h = Float::with_val(wp, &t0);
        let pair = wrap_nodepair(env, Float::with_val(wp, 0), Float::with_val(wp, Constant::Pi) / 2);
        match pair {
            Some(pair) => {
                match env.set_object_array_element(nodes, node_id, pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return JObject::null().into_raw();
                    }
                }
                node_id += 1;
            },
            None => return JObject::null().into_raw()
        }
    } else {
        nodes = match env.new_object_array(1 + 20*(2_i32.pow(degree as u32)), "[palaiologos/scijava/SciFloat", JObject::null()) {
            Ok(nodes) => nodes,
            Err(_) => {
                let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                return JObject::null().into_raw();
            }
        };
        h = Float::with_val(wp, 2 * &t0);
    };
    let expt0 = t0.exp();
    let mut a = Float::with_val(wp, &pi4 * &expt0);
    let mut b = Float::with_val(wp, &pi4 / &expt0);
    let udelta = Float::with_val(wp, h.exp_ref());
    let urdelta = Float::with_val(wp, udelta.recip_ref());
    for i in 0..(1+20*(2_i32.pow(degree as u32))) {
        let c = Float::with_val(wp, &a - &b).exp();
        let d = c.clone().recip();
        let mut co = Float::with_val(wp, &c + &d);
        let mut si = Float::with_val(wp, &c - &d);
        co /= 2; si /= 2;
        let x = Float::with_val(wp, &si / &co);
        let w: Float = Float::with_val(wp, &a + &b) / co.pow(2);
        let diff = Float::with_val(wp, &x - 1).abs();
        if diff <= tol {
            break;
        }
        a *= &udelta;
        b *= &urdelta;
        let pair = wrap_nodepair(env, x.clone(), w.clone());
        match pair {
            Some(pair) => {
                match env.set_object_array_element(nodes, node_id, pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return JObject::null().into_raw();
                    }
                }
                node_id += 1;
            },
            None => return JObject::null().into_raw()
        }
        let pair = wrap_nodepair(env, -x, w);
        match pair {
            Some(pair) => {
                match env.set_object_array_element(nodes, node_id, pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return JObject::null().into_raw();
                    }
                }
                node_id += 1;
            },
            None => return JObject::null().into_raw()
        }
    }
    nodes
}
