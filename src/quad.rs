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

use std::f64::consts::PI;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JObject, JList};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlong, jint, jobject};
use rug::float::Constant;
use rug::ops::{Pow, NegAssign};
use rug::Float;

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
    &mut *(ptr as *mut Float)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_integrator_RealIntegrator_transformNodes(
            env: JNIEnv, _class: JClass, precision: jint, nodes: JObject, a: JObject, b: JObject) {
    let nodes = JList::from_env(&env, nodes).unwrap();
    let orig_a = a;
    let orig_b = b;
    let a = float_from_obj(env, a);
    let b = float_from_obj(env, b);
    let half: Float = Float::with_val(precision as u32, 1) / 2;
    if a == &-1 && b == &1 {
        return;
    }
    let mut a = a.clone();
    a.set_prec(precision as u32);
    let mut b = b.clone();
    b.set_prec(precision as u32);
    if a.is_infinite() || b.is_infinite() {
        if (a.is_infinite() && a.is_sign_negative()) && (b.is_infinite() && b.is_sign_positive()) {
            for i in 0..nodes.size().unwrap() {
                let node = nodes.get(i).unwrap().unwrap();
                let x = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 0).unwrap()) };
                let w = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 1).unwrap()) };

                let x2 = x.clone().square();
                let px1: Float = -x2 + 1;
                let spx1 = px1.clone().recip_sqrt();
                *x *= &spx1;
                *w *= spx1 / px1;
            }
        } else if a.is_infinite() && a.is_sign_negative() {
            for i in 0..nodes.size().unwrap() {
                let node = nodes.get(i).unwrap().unwrap();
                let x = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 0).unwrap()) };
                let w = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 1).unwrap()) };

                let u: Float = 2 / (x.clone() + 1);
                *x = b.clone() - &u + 1;
                *w *= u.square() * &half;
            }
        } else if b.is_infinite() && b.is_sign_positive() {
            for i in 0..nodes.size().unwrap() {
                let node = nodes.get(i).unwrap().unwrap();
                let x = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 0).unwrap()) };
                let w = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 1).unwrap()) };

                let u: Float = 2 / (x.clone() + 1);
                *x = a.clone() + &u - 1;
                *w *= u.square() * &half;
            }
        } else if (a.is_infinite() && a.is_sign_positive()) && (b.is_infinite() && b.is_sign_negative()) {
            Java_palaiologos_scijava_integrator_RealIntegrator_transformNodes(env, _class, precision, *nodes, orig_b, orig_a);
            for i in 0..nodes.size().unwrap() {
                let node = nodes.get(i).unwrap().unwrap();
                let w = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 1).unwrap()) };
                w.neg_assign();
            }
        } else {
            let _ = env.throw(("java/lang/IllegalArgumentException", "Invalid interval"));
        }
    } else {
        // Linear change of variables.
        let c: Float = (b.clone() - &a) / 2;
        let d: Float = (b.clone() + &a) / 2;
        for i in 0..nodes.size().unwrap() {
            let node = nodes.get(i).unwrap().unwrap();
            let x1 = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 0).unwrap()) };
            let x2 = unsafe { mut_float_from_obj(env, env.get_object_array_element(*node, 1).unwrap()) };
            *x1 *= &c; *x1 += &d;
            *x2 *= &c;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_integrator_RealIntegrator_estimateError(
            env: JNIEnv, _class: JClass, precision: jint, epsilon: JObject, nodes: JObject) -> jobject {
    let epsilon = float_from_obj(env, epsilon);
    let nodes = JList::from_env(&env, nodes).unwrap();
    let sz = nodes.size().unwrap();
    if sz == 1 {
        let res = epsilon.clone();
        return match wrap_float(env, res) {
            Some(res) => res.into_raw(),
            None => {
                JObject::null().into_raw()
            }
        }
    }
    let a = float_from_obj(env, nodes.get(sz - 1).unwrap().unwrap());
    let b = float_from_obj(env, nodes.get(sz - 2).unwrap().unwrap());
    if sz == 2 {
        let res = (b.clone() - a).abs();
        return match wrap_float(env, res) {
            Some(res) => res.into_raw(),
            None => {
                JObject::null().into_raw()
            }
        }
    };
    let c = float_from_obj(env, nodes.get(sz - 3).unwrap().unwrap());
    if a == c && a == b {
        let zero = Float::with_val(precision as u32, 0);
        return match wrap_float(env, zero) {
            Some(res) => res.into_raw(),
            None => {
                JObject::null().into_raw()
            }
        }
    }
    let d1 = (a.clone() - b).abs().log10();
    let d2 = (a.clone() - c).abs().log10();
    let d3 = Float::with_val(precision as u32, -precision);
    let d4 = Float::min(Float::max(Float::max(d1.clone().square() / &d2, &(d1 * 2)), &d3), &Float::with_val(precision as u32, 0));
    let error = Float::with_val(precision as u32, 10).pow(d4.floor());
    match wrap_float(env, error) {
        Some(res) => res.into_raw(),
        None => {
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_integrator_RealTanhSinhIntegrator_getNodes(
            env: JNIEnv, _class: JClass, nodes: JObject, precision: jint, degree: jint) {
    let nodes = JList::from_env(&env, nodes).unwrap();
    let wp = precision as u32 + 30;
    let tol = Float::with_val(wp, Float::i_exp(1, -precision-10));
    let pi4 = Float::with_val(wp, Constant::Pi) / 4;
    let t0 = Float::with_val(wp, Float::i_exp(1, -degree));
    let h: Float;
    if degree == 1 {
        h = Float::with_val(wp, &t0);
        let pair = wrap_nodepair(env, Float::with_val(wp, 0), Float::with_val(wp, Constant::Pi) / 2);
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => {
                return;
            }
        }
    } else {
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
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => {
                return;
            }
        }
        let pair = wrap_nodepair(env, -x, w);
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => { return; }
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_integrator_RealGaussLegendreIntegrator_getNodes(
            env: JNIEnv, _class: JClass, nodes: JObject, precision: jint, degree: jint) {
    let nodes = JList::from_env(&env, nodes).unwrap();
    let epsilon = Float::with_val(precision as u32, Float::i_exp(1, -precision-8));
    let precision = precision + precision / 2;
    if degree == 1 {
        let x = (Float::with_val(precision as u32, 3) / 5_i32).sqrt();
        let w: Float = Float::with_val(precision as u32, 5) / 9_i32;
        let pair = wrap_nodepair(env, x.clone(), w.clone());
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => { return; }
        }
        let pair = wrap_nodepair(env, -x, w);
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => { return; }
        }
        let x = Float::with_val(precision as u32, 0);
        let w = Float::with_val(precision as u32, 8) / 9_i32;
        let pair = wrap_nodepair(env, x, w);
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => { return; }
        }
        return;
    }

    let n = 3 * 2.pow((degree - 1) as u32);
    let upto = (n / 2) + 1;
    for j in 1..upto {
        // Roughly compute the starting value for Newton-Raphson iterations.
        // The precision being sligtly off as a result of us using f64 doesn't matter much.
        let mut r = Float::with_val(precision as u32, (PI*((j as f64)-0.25)/((n as f64)+0.5)).cos());
        let mut t4: Float;
        'inner: loop {
            let mut t1 = Float::with_val(precision as u32, 1);
            let mut t2 = Float::with_val(precision as u32, 0);
            for j1 in 1..(n+1) {
                let newt1: Float = (r.clone() * (2 * j1 - 1) * &t1 - Float::with_val(precision as u32, j1-1) * &t2) / j1;
                t2 = t1; t1 = newt1;
            }
            t4 = Float::with_val(precision as u32, n) * (r.clone() * &t1 - &t2) / (r.clone().square() - 1);
            let a: Float = t1 / &t4;
            r -= &a;
            if a.abs() <= epsilon {
                break 'inner;
            }
        }
        let x = r.clone();
        let w: Float = Float::with_val(precision as u32, 2) / ((1 - r.pow(2)) * t4.pow(2));
        let pair = wrap_nodepair(env, x.clone(), w.clone());
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => {
                return;
            }
        }
        let pair = wrap_nodepair(env, -x, w);
        match pair {
            Some(pair) => {
                match nodes.add(pair) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = env.throw(("java/lang/OutOfMemoryError", "Could not allocate memory for nodes"));
                        return;
                    }
                }
            },
            None => { return; }
        }
    }
}
