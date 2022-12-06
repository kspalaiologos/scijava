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
use jni::sys::{jlong, jint, jarray, jobject};
use rug::float::Constant;
use rug::ops::{Pow, NegAssign};
use rug::{Integer, Float};

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
        Ok(x) => {
            env.set_object_array_element(x, 0, x1).unwrap();
            env.set_object_array_element(x, 1, x2).unwrap();
            Some(unsafe { JObject::from_raw(x) })
        },
        Err(_) => {
            let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
            None
        }
    }
}

fn float_from_obj<'a>(env: JNIEnv, obj: JObject<'a>) -> &'a Float {
    let ptr = env.get_field(obj, "ptr", "J").unwrap().j().unwrap();
    unsafe { &*(ptr as *mut Float) }
}

// evil shit
unsafe fn mut_float_from_obj<'a>(env: JNIEnv, obj: JObject<'a>) -> &'a mut Float {
    let ptr = env.get_field(obj, "ptr", "J").unwrap().j().unwrap();
    unsafe { &mut *(ptr as *mut Float) }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_TanhSinhIntegrator_transformNodes(
            env: JNIEnv, _class: JClass, precision: jint, nodes: jarray, a: JObject, b: JObject) -> jarray {
    let orig_a = a;
    let orig_b = b;
    let a = float_from_obj(env, a);
    let b = float_from_obj(env, b);
    let half: Float = Float::with_val(precision as u32, 1) / 2;
    if a == &-1 && b == &1 {
        return nodes;
    }
    if a.is_infinite() || b.is_infinite() {
        if (a.is_infinite() && a.is_sign_negative()) && (b.is_infinite() && b.is_sign_positive()) {
            let new_nodes = match env.new_object_array(env.get_array_length(nodes).unwrap(), "[Lpalaiologos/scijava/SciFloat", JObject::null()) {
                Ok(nodes) => nodes,
                Err(_) => {
                    let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                    return JObject::null().into_raw();
                }
            };
            for i in 0..env.get_array_length(nodes).unwrap() {
                let node = env.get_object_array_element(nodes, i).unwrap();
                let x = float_from_obj(env, env.get_object_array_element(*node, 0).unwrap());
                let w = float_from_obj(env, env.get_object_array_element(*node, 1).unwrap());

                let x2 = x.clone().square();
                let px1: Float = -x2 + 1;
                let spx1 = px1.clone().recip_sqrt();
                let x = x.clone() * &spx1;
                let w = w * (spx1 / px1);

                let node = wrap_nodepair(env, x, w);
                let node = match node {
                    Some(node) => node,
                    None => return JObject::null().into_raw()
                };
                env.set_object_array_element(new_nodes, i, node).unwrap();
            }
            nodes
        } else if a.is_infinite() && a.is_sign_negative() {
            let new_nodes = match env.new_object_array(env.get_array_length(nodes).unwrap(), "[Lpalaiologos/scijava/SciFloat", JObject::null()) {
                Ok(nodes) => nodes,
                Err(_) => {
                    let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                    return JObject::null().into_raw();
                }
            };
            for i in 0..env.get_array_length(nodes).unwrap() {
                let node = env.get_object_array_element(nodes, i).unwrap();
                let x = float_from_obj(env, env.get_object_array_element(*node, 0).unwrap());
                let w = float_from_obj(env, env.get_object_array_element(*node, 1).unwrap());

                let u: Float = 2 / (x.clone() + 1);
                let x = b.clone() - &u + 1;
                let w = w.clone() * &half * u.square();

                let node = wrap_nodepair(env, x, w);
                let node = match node {
                    Some(node) => node,
                    None => return JObject::null().into_raw()
                };
                env.set_object_array_element(new_nodes, i, node).unwrap();
            }
            nodes
        } else if b.is_infinite() && b.is_sign_positive() {
            let new_nodes = match env.new_object_array(env.get_array_length(nodes).unwrap(), "[Lpalaiologos/scijava/SciFloat", JObject::null()) {
                Ok(nodes) => nodes,
                Err(_) => {
                    let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                    return JObject::null().into_raw();
                }
            };
            for i in 0..env.get_array_length(nodes).unwrap() {
                let node = env.get_object_array_element(nodes, i).unwrap();
                let x = float_from_obj(env, env.get_object_array_element(*node, 0).unwrap());
                let w = float_from_obj(env, env.get_object_array_element(*node, 1).unwrap());

                let u: Float = 2 / (x.clone() + 1);
                let x = a.clone() + &u - 1;
                let w = w.clone() * &half * u.square();

                let node = wrap_nodepair(env, x, w);
                let node = match node {
                    Some(node) => node,
                    None => return JObject::null().into_raw()
                };
                env.set_object_array_element(new_nodes, i, node).unwrap();
            }
            nodes
        } else if (a.is_infinite() && a.is_sign_positive()) && (b.is_infinite() && b.is_sign_negative()) {
            let new_nodes = Java_palaiologos_scijava_TanhSinhIntegrator_transformNodes(env, _class, precision, nodes, orig_b, orig_a);
            for i in 0..env.get_array_length(nodes).unwrap() {
                let node = env.get_object_array_element(nodes, i).unwrap();
                let w = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 1).unwrap()) };
                w.neg_assign();
            }
            new_nodes
        } else {
            let _ = env.throw(("java/lang/IllegalArgumentException", "Invalid interval"));
            JObject::null().into_raw()
        }
    } else {
        // Linear change of variables.
        let c: Float = (b.clone() - a) / 2;
        let d: Float = (b.clone() + a) / 2;
        let new_nodes = match env.new_object_array(env.get_array_length(nodes).unwrap(), "[Lpalaiologos/scijava/SciFloat", JObject::null()) {
            Ok(nodes) => nodes,
            Err(_) => {
                let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                return JObject::null().into_raw();
            }
        };
        for i in 0..env.get_array_length(nodes).unwrap() {
            let node = env.get_object_array_element(nodes, i).unwrap();
            let x1 = float_from_obj(env, env.get_object_array_element(*node, 0).unwrap());
            let x2 = float_from_obj(env, env.get_object_array_element(*node, 1).unwrap());
            let x1 = c.clone() * x1 + &d;
            let x2 = c.clone() * x2;
            let node = wrap_nodepair(env, x1, x2);
            let node = match node {
                Some(node) => node,
                None => return JObject::null().into_raw()
            };
            env.set_object_array_element(new_nodes, i, node).unwrap();
        }
        nodes
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
    let nodes: jarray;
    let h: Float;
    if degree == 1 {
        nodes = match env.new_object_array(2 + 20*(2_i32.pow(degree as u32)), "[Lpalaiologos/scijava/SciFloat", JObject::null()) {
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
        nodes = match env.new_object_array(1 + 20*(2_i32.pow(degree as u32)), "[Lpalaiologos/scijava/SciFloat", JObject::null()) {
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
    let mut b = pi4 / &expt0;
    let udelta = h.exp();
    let urdelta = Float::with_val(wp, udelta.recip_ref());
    for _ in 0..(1+20*(2_i32.pow(degree as u32))) {
        let c = Float::with_val(wp, &a - &b).exp();
        let d = c.clone().recip();
        let mut co = Float::with_val(wp, &c + &d);
        let mut si = c - &d;
        co /= 2; si /= 2;
        let x = si / &co;
        let w: Float = Float::with_val(wp, &a + &b) / co.pow(2);
        // x generally converges to 1, check how close we are
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
