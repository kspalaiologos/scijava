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

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JObject, JValue};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlong, jint, jobject};
use rug::Float;
use rug::float::{Constant, Special};
use rug::ops::Pow;

use crate::scifloat_elementary::xlat_rounding;

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_rf(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let n = n as *mut Float;
    let a = unsafe { &*a };
    let n = unsafe { &*n };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    // gamma(a + n) / gamma(a)
    let ga = dest.clone().gamma();
    *dest += n;
    dest.gamma_mut();
    *dest /= ga;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ff(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let n = n as *mut Float;
    let a = unsafe { &*a };
    let n = unsafe { &*n };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    // gamma(a + 1) / gamma(a - n + 1)
    let mut ga = dest.clone();
    ga += 1;
    ga.gamma_mut();
    *dest -= n;
    *dest += 1;
    dest.gamma_mut();
    *dest /= ga;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_beta(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    let mut bp = b.clone();
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    if b.prec() != precision as u32 {
        bp.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    let mut apb = dest.clone() + &bp;
    dest.gamma_mut();
    bp.gamma_mut();
    apb.gamma_mut();
    *dest *= bp;
    *dest /= apb;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sech(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.cosh_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ai(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.ai_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_csch(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.sinh_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_coth(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.tanh_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_rgamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.gamma_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_loggamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.ln_gamma_mut();
}

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

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sinc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.sin_mut();
    *dest /= a;
}

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
            let l1 = z.ln();
            let l2 = l1.clone().ln();
            return (l1.clone() - &l2 + l2.clone() / &l1 + l2.clone() * (l2 - 2.0) / (2.0 * l1.pow(2)), false);
        } else if k == -1.0 && (-0.36787944117144..0.0).contains(&z) {
            let l1 = (-z).ln();
            return (l1.clone() - (-l1).ln(), false);
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_y0(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.y0_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.y0_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_y1(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.y1_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.y1_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_yn(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jint) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.yn_mut(n);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.yn_mut(n);
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gammainc(
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asinInplace(
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acosInplace(
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atanInplace(
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

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asinhInplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.asinh_mut();
    } else {
        a.set_prec(precision as u32);
        a.asinh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acoshInplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.acosh_mut();
    } else {
        a.set_prec(precision as u32);
        a.acosh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atanhInplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.atanh_mut();
    } else {
        a.set_prec(precision as u32);
        a.atanh_mut();
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_harmonic(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    // H_n = eulergamma + digamma(n + 1)
    *dest += 1;
    dest.digamma_mut();
    *dest += Float::with_val(precision as u32, Constant::Euler);
}

fn bernoulli(prec: u32, n: i32) -> Float {
    Float::with_val(prec, 1 - n).zeta() * -n
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_bernoulli(
        env: JNIEnv, _class: JClass, precision: jint, n: jint) -> jobject {
    let n = bernoulli(precision as u32, n);
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
pub extern "system" fn Java_palaiologos_scijava_SciFloat_zeta(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.zeta_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.zeta_mut();
    }
}
